use sysinfo::{System, SystemExt, CpuExt, DiskExt};
use simplelog::*;
use std::fs::File;
use log::{info, error};
use serde::Serialize;
use reqwest::blocking::Client;

#[derive(Serialize)]
struct SystemStats {
    cpu_usage: f32,
    total_memory: u64,
    used_memory: u64,
    disks: Vec<DiskInfo>,
}

#[derive(Serialize)]
struct DiskInfo {
    name: String,
    total_gb: u64,
    available_gb: u64,
}

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        WriteLogger::new(LevelFilter::Info, Config::default(), File::create("agent.log").unwrap()),
    ]).unwrap();

    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();

    let disks = sys.disks().iter().map(|disk| {
        DiskInfo {
            name: disk.name().to_string_lossy().to_string(),
            total_gb: disk.total_space() / 1_000_000_000,
            available_gb: disk.available_space() / 1_000_000_000,
        }
    }).collect();

    let stats = SystemStats {
        cpu_usage,
        total_memory,
        used_memory,
        disks,
    };

    info!("Toplanan sistem bilgisi: CPU: {:.2}%, RAM: {}/{} KB", cpu_usage, used_memory, total_memory);

    let client = Client::new();
    let res = client.post("http://localhost:8000/log")
        .json(&stats)
        .send();

    match res {
        Ok(response) => info!("Sunucuya gönderildi: {}", response.status()),
        Err(e) => error!("Gönderim hatası: {}", e),
    }
}
