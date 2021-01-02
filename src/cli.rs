use clap::Clap;
use serde_derive::Serialize;

#[derive(Clap, Debug, Serialize)]
#[clap(version = "0.1.0", author = "dicedtomato")]
pub struct Opts {
    #[clap(short, long, parse(try_from_str), default_value = "true")]
    pub distro: bool,
    #[clap(short, long, parse(try_from_str), default_value = "true")]
    pub kernel: bool,
    #[clap(short, long, parse(try_from_str), default_value = "true")]
    pub shell: bool,
    #[clap(short, long, parse(try_from_str), default_value = "true")]
    pub uptime: bool,
    #[clap(short, long, parse(try_from_str), default_value = "true")]
    pub term: bool,
    #[clap(short, long, parse(try_from_str), default_value = "true")]
    pub pacman: bool,
    #[clap(short, long, parse(try_from_str), default_value = "true")]
    pub wm: bool,
    #[clap(short, long, parse(try_from_str), default_value = "true")]
    pub resolution: bool,
    #[clap(short, long, parse(try_from_str), default_value = "true")]
    pub cpu: bool,
    #[clap(short, long, parse(try_from_str), default_value = "true")]
    pub mem: bool,
    #[clap(short, long, parse(try_from_str), default_value = "true")]
    pub gpu: bool,
    #[clap(long)]
    pub theme: bool,
    #[clap(long)]
    pub icons: bool,
}
