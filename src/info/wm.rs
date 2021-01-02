use crate::util::Result;
use std::{env::var, fs::read_to_string, path::Path};

pub fn get_wm_info() -> Result<String> {
    let home = var("HOME")?;
    let path = Path::new(&home).join(".xinitrc");

    let file = read_to_string(&path).expect("no xinitrc");
    let last = file.split(' ').last().expect("couldn't get wm/de");

    Ok(last.to_string().replace("\n", ""))
}
