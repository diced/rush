use regex::Captures;
use regex::Regex;
use std::{process::Command, str::from_utf8};

use crate::util::Result;

pub fn get_gpu_info() -> Result<Vec<String>> {
    let re = Regex::new(r"(?i)(VGA compatible|3D) controller: (([a-zA-Z0-9\[\]]\s?)+)")?;

    let cmd = Command::new("lspci").output()?;
    let st = from_utf8(&cmd.stdout)?;

    let result = re.captures_iter(st).collect::<Vec<Captures>>();
    let mut gpus: Vec<String> = Vec::new();

    for capture in result {
        gpus.append(&mut vec![capture[2].trim().to_string()]);
    }

    Ok(gpus)
}
