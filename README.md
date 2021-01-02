# rush
A faster fetch. This only works on arch linux & probably doesn't support desktop environments.

# usage
```
rush 0.1.0
dicedtomato

USAGE:
    rush [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
        --icons      
        --theme      
    -V, --version    Prints version information

OPTIONS:
    -c, --cpu <cpu>                  [default: true]
    -d, --distro <distro>            [default: true]
    -g, --gpu <gpu>                  [default: true]
    -k, --kernel <kernel>            [default: true]
    -m, --mem <mem>                  [default: true]
    -p, --pacman <pacman>            [default: true]
    -r, --resolution <resolution>    [default: true]
    -s, --shell <shell>              [default: true]
    -t, --term <term>                [default: true]
    -u, --uptime <uptime>            [default: true]
    -w, --wm <wm>                    [default: true]
```

# config
by default rush creates a config file at `$XDG_CONFIG_HOME/rush/rush.toml` if `$XDG_CONFIG_HOME` doesn't exist then it will use `$HOME/.config`. It will be populated with the default configuration options
```toml
distro = true
kernel = true
shell = true
uptime = true
term = true
pacman = true
wm = true
resolution = true
cpu = true
mem = true
gpu = true
theme = false
icons = false
```

# using
download the binary from [releases](https://github.com/diced/rush/releases)
<br>**or**<br>
install from [crates.io (rush-bin)](https://crates.io/crates/rush-bin/) with
```sh
cargo install rush-bin
```

Then run with `rush`