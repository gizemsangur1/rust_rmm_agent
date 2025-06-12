use sysinfo::{System, SystemExt, CpuExt, DiskExt};
use simplelog::*;
use std::fs::File;
use log::{info, error};

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        WriteLogger::new(LevelFilter::Info, Config::default(), File::create("agent.log").unwrap()),
    ]).unwrap();

    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_usage = sys.global_cpu_info().cpu_usage();
    info!("CPU Kullanımı: {:.2}%", cpu_usage);

    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    info!("RAM Kullanımı: {}/{} KB", used_memory, total_memory);

    for disk in sys.disks() {
        let name = disk.name().to_string_lossy();
        let total = disk.total_space() / 1_000_000_000;
        let available = disk.available_space() / 1_000_000_000;
        info!("Disk: {}, Toplam: {} GB, Kullanılabilir: {} GB", name, total, available);
    }

}
