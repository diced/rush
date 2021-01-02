#![deny(warnings)]

use crate::cli::Opts;
use crate::util::Result;
use serde_derive::{Deserialize, Serialize};
use std::io::Write;
use std::{fs::*, path::PathBuf};
use xdg::BaseDirectories;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub distro: Option<bool>,
    pub kernel: Option<bool>,
    pub shell: Option<bool>,
    pub uptime: Option<bool>,
    pub term: Option<bool>,
    pub pacman: Option<bool>,
    pub wm: Option<bool>,
    pub resolution: Option<bool>,
    pub cpu: Option<bool>,
    pub mem: Option<bool>,
    pub gpu: Option<bool>,
    pub theme: Option<bool>,
    pub icons: Option<bool>,
}

pub fn parse(path: PathBuf) -> Result<Config> {
    let content = read_to_string(path)?;
    let decoded: Config = toml::from_str(&content)?;
    Ok(decoded)
}

pub fn use_config_or_opts(opts: &Opts) -> Result<Opts> {
    let dir = BaseDirectories::with_prefix("rush").unwrap();
    let config_path = dir.place_config_file("rush.toml").expect("config bruh");

    if !config_path.exists() {
        let mut config_file = File::create(&config_path)?;
        let tomled = toml::to_string(opts)?;
        write!(config_file, "{}", tomled)?;
    }

    let config = parse(config_path)?;
    Ok(Opts {
        distro: config.distro.unwrap_or(opts.distro),
        kernel: config.kernel.unwrap_or(opts.kernel),
        shell: config.shell.unwrap_or(opts.shell),
        uptime: config.uptime.unwrap_or(opts.uptime),
        term: config.term.unwrap_or(opts.term),
        pacman: config.pacman.unwrap_or(opts.pacman),
        wm: config.wm.unwrap_or(opts.wm),
        resolution: config.resolution.unwrap_or(opts.resolution),
        cpu: config.cpu.unwrap_or(opts.cpu),
        mem: config.mem.unwrap_or(opts.mem),
        gpu: config.gpu.unwrap_or(opts.gpu),
        theme: config.theme.unwrap_or(opts.theme),
        icons: config.icons.unwrap_or(opts.icons),
    })
}
