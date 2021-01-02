use std::{process::Command, str::from_utf8};

use crate::util::Result;
use regex::{Captures, Regex};

pub fn get_resolution() -> Result<String> {
    let out = Command::new("xrandr")
        .arg("--nograb")
        .arg("--current")
        .output()
        .expect("no xrandr");

    let st = from_utf8(&out.stdout)
        .expect("")
        .split("\n")
        .collect::<Vec<&str>>()[0];

    let re = Regex::new(r"(?m)current [0-9]{4}\s?x\s?[0-9]{4}")?;

    let result = re.captures_iter(st).collect::<Vec<Captures>>();
    let matched = &result[0][0];

    let res = matched.replace("current ", "").replace(" x ", "x");

    Ok(res)
}

pub fn get_gtk_theme() -> Result<String> {
    let out = Command::new("grep")
        .arg("^[^#]*gtk-theme-name")
        .arg("/home/diced/.config/gtk-3.0/settings.ini")
        .output()
        .expect("no grep idk");

    let st = from_utf8(&out.stdout)
        .expect("no parse :(")
        .split("=")
        .collect::<Vec<&str>>()[1]
        .replace("\n", "");

    Ok(st)
}

pub fn get_gtk_icon() -> Result<String> {
    let out = Command::new("grep")
        .arg("^[^#]*gtk-icon-theme-name")
        .arg("/home/diced/.config/gtk-3.0/settings.ini")
        .output()
        .expect("no grep idk");

    let st = from_utf8(&out.stdout)
        .expect("no parse :(")
        .split("=")
        .collect::<Vec<&str>>()[1]
        .replace("\n", "");

    Ok(st)
}
