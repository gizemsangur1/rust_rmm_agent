use sysinfo::{System, SystemExt, CpuExt, DiskExt};
use simplelog::*;
use std::fs::File;
use log::{info, error};
use serde::{Serialize, Deserialize};
use reqwest::blocking::Client;
use std::fs;
use std::thread;
use std::time::Duration;

#[derive(Deserialize)]
struct Config {
    server_url: String,
    log_level: Option<String>,
    interval_seconds: Option<u64>,
}

fn load_config() -> Config {
    let content = fs::read_to_string("config.toml")
        .expect("config.toml dosyası okunamadı.");
    toml::from_str(&content)
        .expect("config.toml geçersiz formatta.")
}

fn parse_log_level(level: Option<String>) -> LevelFilter {
    match level.unwrap_or_else(|| "info".to_string()).to_lowercase().as_str() {
        "off" => LevelFilter::Off,
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info,
    }
}

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
    let config = load_config();
    let log_level = parse_log_level(config.log_level.clone());
    let interval = config.interval_seconds.unwrap_or(30);

    CombinedLogger::init(vec![
        TermLogger::new(log_level, simplelog::Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        WriteLogger::new(log_level, simplelog::Config::default(), File::create("agent.log").unwrap()),
    ]).unwrap();

    info!("Agent başlatıldı. Gönderim aralığı: {} saniye", interval);
    info!("Sunucu URL: {}", config.server_url);

    let client = Client::new();

    loop {
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

        info!("Bilgi gönderiliyor: CPU {:.2}%, RAM {}/{} KB", cpu_usage, used_memory, total_memory);

        let res = client.post(&config.server_url)
            .json(&stats)
            .send();

        match res {
            Ok(response) => info!("Gönderim başarılı: {}", response.status()),
            Err(e) => error!("Gönderim hatası: {}", e),
        }

        thread::sleep(Duration::from_secs(interval));
    }
}
