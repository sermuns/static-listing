<a href="https://github.com/sermuns/stil">
  <img src="media/banner.png">
</a>

<div align="center">
  <a href="https://github.com/sermuns/stil/releases/latest">
    <img alt="release-badge" src="https://img.shields.io/github/v/release/sermuns/stil.svg"></a>
  <a href="https://github.com/sermuns/stil/blob/main/LICENSE">
    <img alt="WTFPL" src="https://img.shields.io/badge/License-WTFPL-brightgreen.svg"></a>
  <a href="https://crates.io/crates/stil"><img src="https://img.shields.io/crates/v/stil.svg" alt="Version info"></a>
</div>
<br>

`stil` is a dead simple static site generator that replicates the experience of viewing a dynamically generated index listing produced by web servers such as nginx and apache.

👉 See what `stil` generates for _this repo_: <a href="https://stil.samake.se/demo/" target="_blank">https://stil.samake.se/demo/</a>

## Usage

```
generate STatic site from Index Listing of directory

Usage: stil [OPTIONS] [INPUT_DIR]

Arguments:
  [INPUT_DIR]  Which directory to generate listing of [default: .]

Options:
  -o, --output-dir <OUTPUT_DIR>  Which directory to write generated HTML to [default: public]
  -t, --title <TITLE>            Which <title> to give the generated HTML [default: stil]
  -i, --ignored <IGNORED>        Which files/directories to NOT include in the output
  -u, --url-path <URL_PATH>      On which path the final page will be deployed [default: /]
  -h, --help                     Print help
  -V, --version                  Print version
```

## Installation

For each version, prebuilt binaries are automatically built for Linux, MacOS and Windows.

- You can download and unpack the
  latest release from the [releases page](https://github.com/sermuns/stil/releases/latest).

- Using [`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall)

  ```sh
  cargo binstall stil
  ```

- Using [`ubi`](https://github.com/houseabsolute/ubi):

  ```sh
  ubi -p sermuns/stil
  ```

- From source with Cargo

  ```sh
  cargo install stil
  ```

Actually, you don't need to install locally. It works fine in docker container:

```sh
docker run -v $(pwd):/app -u $(id -u):$(id -g) ghcr.io/sermuns/stil <arguments to stil>
```
