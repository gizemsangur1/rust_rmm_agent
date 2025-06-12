use sysinfo::{System, SystemExt, CpuExt, DiskExt};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_usage = sys.global_cpu_info().cpu_usage();
    println!("CPU Kullanımı: {:.2}%", cpu_usage);

    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    println!("RAM Kullanımı: {}/{} KB", used_memory, total_memory);

    for disk in sys.disks() {
        println!(
            "Disk: {:?}, Toplam: {} GB, Kullanılabilir: {} GB",
            disk.name(),
            disk.total_space() / 1_000_000_000,
            disk.available_space() / 1_000_000_000
        );
    }
}
