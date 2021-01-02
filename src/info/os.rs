use std::{env::var, fs::read_to_string, path::Path, process::Command, str::from_utf8};

use crate::util::Result;

pub fn get_kernel_info() -> Result<String> {
    let out = Command::new("uname").arg("-r").output()?;

    let kernel = from_utf8(&out.stdout)?;

    Ok(kernel.to_string().replace("\n", ""))
}

pub fn get_distro_info() -> Result<String> {
    let distro_out = Command::new("lsb_release").arg("-sd").output()?;
    let arch_out = Command::new("uname").arg("-m").output()?;

    let arch = from_utf8(&arch_out.stdout)
        .expect("uname failed")
        .replace("\n", "");

    let distro = from_utf8(&distro_out.stdout)
        .expect("couldn't get out")
        .replace("\"", "")
        .replace("\n", "");

    Ok(format!("{} {}", distro, arch))
}

pub fn get_uptime_info() -> Result<String> {
    let t = read_to_string("/proc/uptime")
        .expect("cant open uptime")
        .split('.')
        .collect::<Vec<&str>>()[0]
        .parse::<u64>()?;

    let days = t / 86400;
    let hours = (t / 3600) % 24;
    let mins = (t / 60) % 60;

    let mut fmt: String = String::new();

    if days != 0 {
        fmt = format!("{} days, ", days);
    }
    if hours != 0 {
        fmt = format!("{}{} hours, ", fmt, hours);
    }
    if mins != 0 {
        fmt = format!("{}{} mins", fmt, mins);
    }

    Ok(fmt)
}

pub fn get_shell() -> Result<String> {
    let shell_path = var("SHELL").expect("no shell var");
    let path = Path::new(&shell_path);

    let shell = path.file_name().expect("brh");

    Ok(shell.to_str().unwrap().to_string())
}

pub fn get_hostname() -> Result<String> {
    let res = Command::new("hostname").output();
    let mut _hostname = String::new();

    match res {
        Ok(x) => {
            _hostname = from_utf8(&x.stdout).unwrap().replace("\n", "");
        }
        Err(_) => {
            _hostname = read_to_string("/etc/hostname").unwrap().replace("\n", "");
        }
    }

    Ok(_hostname)
}

pub fn get_user() -> Result<String> {
    let username = var("USER").expect("no USER env");

    Ok(username)
}

pub fn get_terminal() -> Result<String> {
    let ps_out = Command::new("ps")
        .arg("-o")
        .arg("ppid")
        .output()
        .expect("no ps");

    let ppid = from_utf8(&ps_out.stdout)
        .unwrap()
        .split("\n")
        .collect::<Vec<&str>>()[1]
        .trim();

    let status = read_to_string(Path::new("/proc").join(ppid).join("status"))
        .expect("couldn't open ppid proc status");

    let term = status.split("\n").collect::<Vec<&str>>()[0]
        .split(":")
        .collect::<Vec<&str>>()[1]
        .trim()
        .replace("\n", "");

    Ok(term)
}
