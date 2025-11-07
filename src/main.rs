#![allow(unused_variables)]
use anyhow::{Context, Result};
use clap::Parser;
use maud::{DOCTYPE, html};
use std::fs::DirEntry;
use std::sync::LazyLock;
use std::{
    fs,
    path::{Path, PathBuf},
};

const LOGO_B64: &str = env!("LOGO_B64");
const STYLE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/style.css"));
static ARGS: LazyLock<Args> = LazyLock::new(|| {
    let mut args = Args::parse();
    let mut default_ignored = vec![PathBuf::from(".git"), args.output_dir.clone()];
    args.ignored.append(&mut default_ignored);
    args
});

#[derive(Parser, Debug)]
#[clap(version, author, about)]
struct Args {
    /// Which directory to generate listing of
    #[arg(default_value = ".")]
    input_dir: PathBuf,

    /// Which directory to write generated HTML to
    #[arg(short, long, default_value = "public")]
    output_dir: PathBuf,

    /// Which <title> to give the generated HTML
    #[arg(short, long, default_value = env!("CARGO_PKG_NAME"))]
    title: String,

    /// Which files/directories to NOT include in the output
    #[arg(short, long, value_delimiter = ',')]
    ignored: Vec<PathBuf>,
}

fn generate_html(dir_entries: impl Iterator<Item = PathBuf>) -> String {
    html! {
        (DOCTYPE)
        html {
            head {
                title {(ARGS.title)};
                style {(STYLE)};
                link rel="icon" type="image/svg+xml" href={"data:image/svg+xml;base64,"(LOGO_B64) };
            }
            body {
                ul {
                    @for entry in dir_entries {
                        a href=(entry.to_string_lossy()) { (entry.to_string_lossy()) }
                    }
                }
            }
        }
    }
    .into_string()
}

/// Recurse into directory structure
fn build(root: &Path, output_dir: &Path) -> Result<()> {
    if root.is_file() {
        let from = root;
        let to = &output_dir.join(root);
        if fs::hard_link(from, output_dir.join(root)).is_err() {
            fs::copy(from, to).with_context(|| {
                format!("failed copying {} to {}", &from.display(), &to.display())
            })?;
        };
    } else {
        let dir_entries: Vec<DirEntry> = root.read_dir()?.filter_map(|e| e.ok()).collect();
        fs::write(
            output_dir.join("index.html"),
            generate_html(dir_entries.iter().map(|e| e.path())),
        )
        .context("unable to write output index.html")?;
        for entry in root.read_dir()? {
            let entry = entry?;
            let entry_path = &entry.path();
            if ARGS
                .ignored
                .iter()
                .any(|i| &Path::new("./").join(i) == entry_path)
            {
                continue;
            }
            build(&entry.path(), &ARGS.output_dir)?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    match fs::remove_dir_all(&ARGS.output_dir) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        default => default,
    }
    .with_context(|| "unable to remove output dir")?;
    fs::create_dir(&ARGS.output_dir)?;

    build(&ARGS.input_dir, &ARGS.output_dir)
}
