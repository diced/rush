use std::fs::read_to_string;

use crate::util::Result;
use procfs::CpuInfo;

pub fn get_cpu_info() -> Result<String> {
    let cpu = CpuInfo::new()?;
    let model = cpu
        .fields
        .get("model name")
        .unwrap()
        .replace("(R)", "")
        .replace("(r)", "")
        .replace("(TM)", "")
        .replace("(tm)", "")
        .replace("(c)", "")
        .replace("(C)", "")
        .replace("CPU", "")
        .replace("Processor", "")
        .replace("Core", "")
        .replace("Hardware", "")
        .replace("  ", " ")
        .split('@')
        .collect::<Vec<&str>>()[0]
        .trim()
        .to_string();

    let cpu_freq = read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq")
        .expect("couldn't get cpu freq")
        .replace("\n", "")
        .parse::<f64>()?;

    let format = format!(
        "{} ({}) @ {:.3}GHz",
        &model,
        cpu.cpus.len(),
        cpu_freq / 1000000_f64
    );

    Ok(format)
}
