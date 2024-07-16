<div align="center">

# unicom

<img src="https://github.com/moz-sec/unicom/blob/main/img/unicorn.png" width="200">

[![Lang](https://img.shields.io/badge/Rust-1.77+-blue.svg?logo=rust)](https://www.rust-lang.org/)
[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/)
[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/moz-sec/unicom)](https://rust-reportcard.xuri.me/report/github.com/moz-sec/unicom)
[![Coverage Status](https://coveralls.io/repos/github/moz-sec/unicom/badge.svg?branch=main)](https://coveralls.io/github/moz-sec/unicom?branch=main)
[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.11090375.svg)](https://doi.org/10.5281/zenodo.11090375)

`unicom` is Universal Compressor.

</div>

## Description

Currently, there are different methods of compressing files, and for each format, a unique tool is used.
However, each tool has different commands and options, making it difficult to use.
Therefore, unicom makes it possible to handle various compression formats in a unified interface.

## Usage

The following is the `unicom` help output.

```txt
unicom is Universal Compressor

Usage: unicom [OPTIONS] <FILES>...

Arguments:
  <FILES>...  Compress or Decompress files name

Options:
  -d, --decompress       Whether to decompress the input
  -e, --encrypt          Put passwords on compressed files
  -f, --format <FORMAT>  [default: zip] [possible values: gzip, zip]
  -r, --recursive        Whether to recursively greet
  -v, --verbose          Verbose mode
  -c, --count <COUNT>    Number of compressions to perform [default: 6]
  -h, --help             Print help
  -V, --version          Print version
```

## Installation

There are several installation methods.

### From Source

To build the project from source, you need to have Rust installed.

```bash
git clone https://github.com/moz-sec/unicom.git && cd unicom
cargo build --release
./target/release/unicom --help
```

### Releases Binaries

Binaries can be downloaded from the [Releases](https://github.com/moz-sec/unicom/releases) section of this repository.
Download and extract the binaries according to your environment.

```bash
tar -zxvf unicom-{version}_{architecture}.tar.gz
```

### Homebrew **(Recommended)**

If you are on a Mac, you can also install it using brew.

```bash
brew install unicom
```

### Docker

You can also use the Docker image.
unicom's docker repository is [here](https://hub.docker.com/repository/docker/mozsec/unicom/general).
Put the path containing the files or directories you want to compress in {Host Volume Path}.
Mount the container in "/workdir".
**The compressed file will be created in the {Host Volume Path}.**
Specify {unicom arguments} for the unicom command.

```bash
docker run -it -v {Host Volume Path}:/workdir --rm mozsec/unicom:latest {unicom arguments}
```

## Maintainers

- [@moz-sec](https://github.com/moz-sec)

## License

[MIT](https://github.com/moz-sec/unicom/blob/main/LICENSE)
