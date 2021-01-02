use std::{process::Command, str::from_utf8};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn check_if_archlinux() -> Result<()> {
    let distro_out = Command::new("lsb_release").arg("-sd").output()?;
    let s = from_utf8(&distro_out.stdout)
        .unwrap()
        .replace("\"", "")
        .replace("\n", "");

    if s != "Arch Linux" {
        panic!("not archlinux ..")
    }

    Ok(())
}