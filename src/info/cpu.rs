use std::fs::read_to_string;

use crate::util::Result;
use procfs::CpuInfo;
use regex::Regex;

pub fn get_cpu_info() -> Result<String> {
    let cpu = CpuInfo::new()?;
    let re = Regex::new(r"(?i)(\((r|tm|c)\)|cpu|processor|core|hardware)")?;
    
    let model_replaced = re.replace_all(cpu.fields.get("model name").unwrap(), "");
    let model = model_replaced.split("@")
        .collect::<Vec<&str>>()[0]
        .trim()
        .replace("  ", "");
  
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
