# Lightnovel Downloader
An async lightnovel downloader written in Rust
Creates a epub for a given lightnovel.

### Usage : 
```
░█░░░█▀█░█▀▄░█▀█░█░█░█▀█
░█░░░█░█░█░█░█░█░█▄█░█░█
░▀▀▀░▀░▀░▀▀░░▀▀▀░▀░▀░▀░▀

Light Novel Downloader 0.3

USAGE:
    lndown [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -q, --query <query>        Query to search
    -t, --threads <threads>    No. of threads to be used (default : 5)
                               (Please use with caution as site *will* block increased requests)
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

### TODO : 
- [ ] add proxy
- [ ] add number of chapters in selection screen
- [ ] check status code before adding to chapter list
- [ ] non interactive query search & multiple downloads (download everything in search result)
- [ ] use path given by user
- [ ] add modularity
- [ ] add print details and exit (-d)
- [ ] print more details after selecting book [search.rs]
- [ ] add pagination to search
- [ ] create only html option

#### Powered by
 [![N|Solid](https://www.rust-lang.org/static/images/rust-logo-blk.svg)](https://www.rust-lang.org/)