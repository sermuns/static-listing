use anyhow::{Context, Result};
use clap::Parser;
use maud::{DOCTYPE, html};
use std::ffi::{OsStr, OsString};
use std::fs::DirEntry;
use std::sync::LazyLock;
use std::time::Instant;
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

    /// On which path the final page will be deployed
    #[arg(short, long, default_value = "/")]
    url_path: PathBuf,
}

struct UsefulDirEntry {
    path: PathBuf,
    basename: OsString,
}

fn generate_html<'g>(
    useful_dir_entries: impl Iterator<Item = UsefulDirEntry>,
    root: &'g Path,
) -> String {
    let ancestor_paths_reversed = root
        .ancestors()
        .collect::<Vec<&'g Path>>()
        .into_iter()
        .rev()
        .skip(2);

    html! {
        (DOCTYPE)
        html {
            head {
                title {(ARGS.title)};
                style {(STYLE)};
                link rel="icon" type="image/svg+xml" href={"data:image/svg+xml;base64,"(LOGO_B64) };
            }
            body {
                header {
                    a href=(&ARGS.url_path.to_string_lossy()) {
                        ("/")
                    }
                    @for ancestor_path in ancestor_paths_reversed {
                        a href={(ARGS.url_path.join(ancestor_path.strip_prefix(".").unwrap()).to_string_lossy()) "/"} {
                            (ancestor_path.file_name().unwrap_or_else(|| OsStr::new("/")).to_string_lossy())
                        }
                        span { "/" }
                    }
                }
                main {
                    @for entry in useful_dir_entries {
                        a href={(ARGS.url_path.join(entry.path.strip_prefix(".").unwrap()).to_string_lossy()) "/"} {
                            (entry.basename.to_string_lossy())
                        }
                    }
                }
            }
        }
    }
    .into_string()
}

fn build(root: &Path) -> Result<()> {
    let to = &ARGS.output_dir.join(root.strip_prefix(".")?);
    if root.is_file() {
        fs::create_dir_all(to.parent().unwrap())?;
        let from = root;
        if fs::hard_link(from, ARGS.output_dir.join(root)).is_err() {
            fs::copy(from, to).with_context(|| {
                format!("failed copying {} to {}", &from.display(), &to.display())
            })?;
        };
    } else {
        fs::create_dir_all(to)?;
        let dir_entries: Vec<DirEntry> = root.read_dir()?.filter_map(|e| e.ok()).collect();
        fs::write(
            to.join("index.html"),
            generate_html(
                dir_entries.iter().map(|e| UsefulDirEntry {
                    path: e.path(),
                    basename: e.file_name(),
                }),
                root,
            ),
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
            build(&entry.path())?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let start_instant = Instant::now();
    match fs::remove_dir_all(&ARGS.output_dir) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        default => default,
    }
    .with_context(|| "unable to remove output dir")?;
    fs::create_dir(&ARGS.output_dir)?;

    build(&ARGS.input_dir)?;
    println!(
        "Built static index listing of `{}` to `{}` in {:?}",
        &ARGS.input_dir.to_string_lossy(),
        &ARGS.output_dir.to_string_lossy(),
        start_instant.elapsed(),
    );
    Ok(())
}
