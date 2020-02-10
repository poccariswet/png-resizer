# png-resizer

this project provide png resize command line interface. But not only png. if changing `ImageFormat`, it can use other image type.<br>
[clap](https://github.com/clap-rs/clap) is the best Command Line Argument Parser for Rust.

## Usage

```
png-resizer 1.0
poccariswet <poccariswet@gmail.com>
png resize command line interface

USAGE:
    png-resizer [OPTIONS] <FILES>...

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -h, --hegiht <NUM>         Sets a height value
    -t, --target-size <NUM>    Sets a target size value
    -w, --width <NUM>          Sets a width value

ARGS:
    <FILES>...    Sets for resize files
```


## Reference
- [https://docs.rs/image/0.23.0/image/](https://docs.rs/image/0.23.0/image/)
- [https://docs.rs/clap/2.33.0/clap/](https://docs.rs/clap/2.33.0/clap/)
- [https://qiita.com/benki/items/5303f5e3c5a498be2804](https://qiita.com/benki/items/5303f5e3c5a498be2804)
