# Lightnovel Downloader
An async lightnovel downloader written in Rust
Creates a epub for a given lightnovel.

### Usage : 
```
░█░░░█▀█░█▀▄░█▀█░█░█░█▀█
░█░░░█░█░█░█░█░█░█▄█░█░█
░▀▀▀░▀░▀░▀▀░░▀▀▀░▀░▀░▀░▀

Light Novel Downloader 0.2

USAGE:
    lndown [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -q, --query <query>        Query to search
    -t, --threads <threads>    No. of threads to be used (default : 7)
                               (Please use with caution as site can block increased requests)
    -u, --url <url>            Url of Lightnovel to Download

```
### How to get it :
```
    git clone https://github.com/krypton-kry/lndown
    cd lndown 
    cargo build --release
    ./target/release/lndown
```
### Sources : 
  wuxiaworld.co

#### Powered by
 [![N|Solid](https://www.rust-lang.org/static/images/rust-logo-blk.svg)](https://www.rust-lang.org/)
