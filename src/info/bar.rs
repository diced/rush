use std::{process::Command, str::from_utf8};
use regex::{Captures, Regex};

use crate::util::Result;

pub fn get_bar() -> Result<String> {
    let ps_out = Command::new("ps")
        .arg("-e")
        .output()
        .expect("no ps");

    let all_proces = from_utf8(&ps_out.stdout)?;

    let re = Regex::new(r"(?i)polybar|i3bar|dzen2|tint2|xmobar|swaybar|lemonbar|taffybar")?;

    let result = re.captures_iter(all_proces).collect::<Vec<Captures>>();

    let mut _bar = String::new();

    if result.len() == 0 {
        _bar = "none".to_string();
    } else {
        _bar = result[0][0].trim().to_string();
    }

    Ok(_bar)
}
