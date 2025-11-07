<a href="https://github.com/sermuns/stil">
  <img src="media/banner.svg">
</a>

<div align="center">
  <a href="https://github.com/sermuns/stil/releases/latest">
    <img alt="release-badge" src="https://img.shields.io/github/v/release/sermuns/stil.svg"></a>
  <a href="https://github.com/sermuns/stil/blob/main/LICENSE">
    <img alt="WTFPL" src="https://img.shields.io/badge/License-WTFPL-brightgreen.svg"></a>
  <a href="https://crates.io/crates/stil"><img src="https://img.shields.io/crates/v/stil.svg" alt="Version info"></a>
</div>

`stil` is a dead simple [static site generator](https://en.wikipedia.org/wiki/Static_site_generator) that tries to replicate the experience of viewing a dynamically generated index listing typically produced by web servers such as nginx and apache.

## Usage

```
generate STatic site from Index Listing of directory

Usage: stil [OPTIONS] [INPUT_DIR]

Arguments:
  [INPUT_DIR]  Which directory to generate listing of [default: .]

Options:
  -o, --output-dir <OUTPUT_DIR>  Which directory to write generated HTML to
[default: public]
  -t, --title <TITLE>            Which <title> to give the generated HTML [d
efault: stil]
  -i, --ignored <IGNORED>        Which files/directories to NOT include in t
he output
  -u, --url-path <URL_PATH>      On which path the final page will be deploy
ed [default: /]
  -h, --help                     Print help
  -V, --version                  Print version
```

## Installation

For each version, prebuilt binaries are automatically built for Linux, MacOS and Windows.

- You can download and unpack the
  latest release from the [releases page](https://github.com/sermuns/stil/releases/latest).

- Using [`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall)

  ```bash
  cargo binstall stil
  ```

- Using [`ubi`](https://github.com/houseabsolute/ubi):

  ```bash
  ubi -p sermuns/stil
  ```

- From source with Cargo

  ```bash
  cargo install stil
  ```
