use crate::util::Result;
use procfs::Meminfo;

pub fn get_memory_info() -> Result<String> {
    let s = Meminfo::new()?;
    let total = s.mem_total / 1024 / 1024;
    let available = s.mem_available.unwrap() / 1024 / 1024;

    let st = format!(
        "{} MB / {} MB",
        total - available,
        s.mem_total / 1024 / 1024
    );

    Ok(st)
}
