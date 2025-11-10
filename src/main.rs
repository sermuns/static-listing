use anyhow::{Context, Result};
use argh::FromArgs;
use humansize::{BINARY, format_size};
use maud::{DOCTYPE, PreEscaped, html};
use std::ffi::{OsStr, OsString};
use std::fs::DirEntry;
use std::sync::LazyLock;
use std::time::Instant;
use std::{
    fs,
    path::{Path, PathBuf},
};
use time::OffsetDateTime;
use time::format_description::BorrowedFormatItem;
use time::macros::format_description;

const DATE_FORMAT: &[BorrowedFormatItem<'_>] =
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");

const LOGO_B64: &str = env!("LOGO_B64");
const STYLE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/style.css"));

#[derive(FromArgs, Debug)]
/// .
struct Args {
    /// directory to generate listing of
    #[argh(option, default = "PathBuf::from(\".\")")]
    input_dir: PathBuf,

    /// directory to write generated html to
    #[argh(option, default = "PathBuf::from(\"public\")")]
    output_dir: PathBuf,

    /// search hidden files and directories
    #[argh(switch)]
    hidden: bool,

    /// title to give the generated HTML
    #[argh(option, default = "env!(\"CARGO_PKG_NAME\").to_string()")]
    title: String,

    /// files/directories to not include in the output
    #[argh(option)]
    ignored: Vec<PathBuf>,

    /// on which url path the final page will be deployed
    #[argh(option, default = "PathBuf::from(\"/\")")]
    url_path: PathBuf,
}

static ARGS: LazyLock<Args> = LazyLock::new(|| {
    let mut args: Args = argh::from_env();
    let mut default_ignored = vec![PathBuf::from(".git"), args.output_dir.clone()];
    args.ignored.append(&mut default_ignored);
    args
});

struct UsefulDirEntry {
    path: PathBuf,
    basename: OsString,
    last_modified_str: String,
    human_size_str: String,
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
                    b {"Type"}
                    b {"Name"}
                    b {"Last modified"}
                    b {"Size"}
                    @for entry in useful_dir_entries {
                        @if entry.path.is_dir() {
                            span class="dir" {(PreEscaped("&#128448;"))}
                        }
                        @else {
                            //span class="file" {(PreEscaped("&#128462;"))}
                            span class="file" {}
                        }
                        a href={(ARGS.url_path.join(entry.path.strip_prefix(".").unwrap()).to_string_lossy()) "/"} {
                            (entry.basename.to_string_lossy())
                        }
                        span {
                            (entry.last_modified_str)
                        }
                        span {
                            (entry.human_size_str)
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
        let mut dir_entries: Vec<DirEntry> = root
            .read_dir()?
            .filter_map(|entry| {
                if let Ok(e) = entry {
                    let entry_path = e.path();
                    if !ARGS.hidden
                        && let Some(file_name) = entry_path.file_name()
                        && file_name.to_str().unwrap().starts_with(".")
                    {
                        return None;
                    }

                    if ARGS
                        .ignored
                        .iter()
                        .any(|i| Path::new("./").join(i) == entry_path)
                    {
                        return None;
                    }
                    return Some(e);
                }
                None
            })
            .collect();
        dir_entries.sort_by_key(|e| e.path());

        fs::write(
            to.join("index.html"),
            generate_html(
                dir_entries.iter().map(|e| {
                    let entry_metadata = e.path().metadata();
                    UsefulDirEntry {
                        path: e.path(),
                        basename: e.file_name(),
                        last_modified_str: {
                            if let Ok(meta) = &entry_metadata {
                                if let Ok(modified) = meta.modified() {
                                    let datetime: OffsetDateTime = modified.into();
                                    datetime.format(DATE_FORMAT).unwrap_or("-".to_string())
                                } else {
                                    "-".to_string()
                                }
                            } else {
                                "-".to_string()
                            }
                        },
                        human_size_str: {
                            if e.path().is_file()
                                && let Ok(meta) = &entry_metadata
                            {
                                format_size(meta.len(), BINARY)
                            } else {
                                "-".to_string()
                            }
                        },
                    }
                }),
                root,
            ),
        )
        .context("unable to write output index.html")?;

        for entry in dir_entries {
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
