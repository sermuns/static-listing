use anyhow::Result;
use clap::Parser;
use maud::{DOCTYPE, html};
use std::{fs, path::PathBuf};
use walkdir::WalkDir;

const ICON_STR: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/media/logo.svg"));
const ICON_B64: &[u8] = &data_encoding_macro::base64!("filestylecss");
const STYLE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/style.css"));

#[derive(Parser, Debug)]
#[clap(version, author, about)]
struct Args {
    /// Which directory to generate listing of
    #[arg(default_value = ".")]
    input_dir: PathBuf,

    /// Which file to write generated HTML to
    #[arg(short, long, default_value = "index.html")]
    output: PathBuf,

    /// Which <title> to give the generated HTML
    #[arg(short, long, default_value = "Static Listing")]
    title: String,

    /// Which files/directories to NOT include in the output
    #[arg(short, long, value_delimiter = ',')]
    ignored: Vec<PathBuf>,
}

fn generate_html(paths: impl Iterator<Item = PathBuf>, title: &str) -> String {
    html! {
        (DOCTYPE)
        html {
            head {
                icon
                title {(&title)}
                style {(STYLE)}
            }
            body {
                ul {
                    @for path in paths {
                        li { (path.to_string_lossy()) }
                    }
                }
            }
        }
    }
    .into_string()
}

fn main() -> Result<()> {
    let args = Args::parse();

    let input_paths = WalkDir::new(args.input_dir)
        .into_iter()
        .filter_map(|e| match e {
            Ok(e) => Some(e.into_path()),
            _ => None,
        })
        .filter(|e| {
            for i in &args.ignored {
                if e.starts_with(i) {
                    return false;
                }
            }
            true
        });

    fs::write(args.output, generate_html(input_paths, &args.title))?;

    Ok(())
}
