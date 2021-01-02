use std::{process::Command, str::from_utf8};

use crate::util::Result;

pub fn get_packages_info() -> Result<usize> {
    let cmd = Command::new("pacman").arg("-Q").output()?;
    let st = from_utf8(&cmd.stdout)?;
    let len = st.split("\n").count();

    Ok(len)
}
