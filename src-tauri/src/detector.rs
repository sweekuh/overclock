use serde::Deserialize;
use crate::types::*;

// ─── WMI Structs (deserialized by the wmi crate) ────────────────────────────

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_Processor")]
#[serde(rename_all = "PascalCase")]
struct WmiCpu {
    name: String,
    manufacturer: String,
    number_of_cores: u32,
    number_of_logical_processors: u32,
    max_clock_speed: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_VideoController")]
#[serde(rename_all = "PascalCase")]
struct WmiGpu {
    name: String,
    adapter_compatibility: Option<String>,
    adapter_ram: Option<u64>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_PhysicalMemory")]
#[serde(rename_all = "PascalCase")]
struct WmiMemory {
    capacity: u64,
    speed: Option<u32>,
    memory_type: Option<u32>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_DiskDrive")]
#[serde(rename_all = "PascalCase")]
struct WmiDisk {
    model: String,
    media_type: Option<String>,
    size: Option<u64>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_OperatingSystem")]
#[serde(rename_all = "PascalCase")]
struct WmiOs {
    caption: String,
    version: String,
    build_number: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_NetworkAdapter")]
#[serde(rename_all = "PascalCase")]
struct WmiNetwork {
    name: String,
    net_connection_id: Option<String>,
    speed: Option<u64>,
    adapter_type: Option<String>,
}

// ─── Detection Functions ─────────────────────────────────────────────────────

pub fn detect_hardware() -> Result<HardwareProfile, String> {
    // WMI requires COM initialized as MTA, but Tauri's WebView2 already
    // initialized COM as STA on the main thread. Spawn a dedicated thread
    // where we can safely call COMLibrary::new() without conflict.
    std::thread::spawn(|| {
        let com_lib = wmi::COMLibrary::new()
            .map_err(|e| format!("COM initialization failed: {}", e))?;
        let wmi_con = wmi::WMIConnection::new(com_lib)
            .map_err(|e| format!("WMI connection failed: {}", e))?;

        let cpu = detect_cpu(&wmi_con)?;
        let gpu = detect_gpu(&wmi_con)?;
        let memory = detect_memory(&wmi_con)?;
        let storage = detect_storage(&wmi_con)?;
        let network = detect_network(&wmi_con)?;
        let os = detect_os(&wmi_con)?;
        let games = scan_games();

        Ok(HardwareProfile {
            cpu,
            gpu,
            memory,
            storage,
            network,
            os,
            games,
        })
    })
    .join()
    .map_err(|_| "Hardware detection thread panicked".to_string())?
}

fn detect_cpu(wmi: &wmi::WMIConnection) -> Result<CpuInfo, String> {
    let results: Vec<WmiCpu> = wmi.query().map_err(|e| format!("CPU query failed: {}", e))?;
    let cpu = results.first().ok_or("No CPU found")?;
    Ok(CpuInfo {
        name: cpu.name.trim().to_string(),
        manufacturer: cpu.manufacturer.clone(),
        cores: cpu.number_of_cores,
        threads: cpu.number_of_logical_processors,
        max_clock_mhz: cpu.max_clock_speed,
    })
}

fn detect_gpu(wmi: &wmi::WMIConnection) -> Result<GpuInfo, String> {
    let results: Vec<WmiGpu> = wmi.query().map_err(|e| format!("GPU query failed: {}", e))?;
    let gpu = results.first().ok_or("No GPU found")?;
    Ok(GpuInfo {
        name: gpu.name.trim().to_string(),
        manufacturer: gpu.adapter_compatibility.clone().unwrap_or_default(),
        // WMI reports adapter_ram in bytes, convert to MB
        vram_mb: gpu.adapter_ram.unwrap_or(0) / (1024 * 1024),
    })
}

fn detect_memory(wmi: &wmi::WMIConnection) -> Result<MemoryInfo, String> {
    let results: Vec<WmiMemory> = wmi
        .query()
        .map_err(|e| format!("Memory query failed: {}", e))?;
    let total_bytes: u64 = results.iter().map(|m| m.capacity).sum();
    let speed = results.first().and_then(|m| m.speed).unwrap_or(0);
    let mem_type = results
        .first()
        .and_then(|m| m.memory_type)
        .map(|t| match t {
            26 => "DDR4".to_string(),
            34 => "DDR5".to_string(),
            _ => format!("Type {}", t),
        })
        .unwrap_or_else(|| "Unknown".to_string());

    Ok(MemoryInfo {
        total_mb: total_bytes / (1024 * 1024),
        speed_mhz: speed,
        memory_type: mem_type,
    })
}

fn detect_storage(wmi: &wmi::WMIConnection) -> Result<StorageInfo, String> {
    let results: Vec<WmiDisk> = wmi
        .query()
        .map_err(|e| format!("Disk query failed: {}", e))?;
    let disk = results.first().ok_or("No disk found")?;

    let media_type = if disk.model.to_lowercase().contains("nvme") {
        StorageType::NVMe
    } else if disk
        .media_type
        .as_ref()
        .map(|t| t.contains("SSD") || t.contains("Solid"))
        .unwrap_or(false)
    {
        StorageType::SSD
    } else if disk
        .media_type
        .as_ref()
        .map(|t| t.contains("Fixed") || t.contains("HDD"))
        .unwrap_or(false)
    {
        StorageType::HDD
    } else {
        // Heuristic: if model name contains "NVMe" or "SSD"
        if disk.model.to_lowercase().contains("ssd") {
            StorageType::SSD
        } else {
            StorageType::Unknown
        }
    };

    Ok(StorageInfo {
        media_type,
        total_gb: disk.size.unwrap_or(0) / (1024 * 1024 * 1024),
        model: disk.model.trim().to_string(),
    })
}

fn detect_network(wmi: &wmi::WMIConnection) -> Result<NetworkInfo, String> {
    let results: Vec<WmiNetwork> = wmi
        .query()
        .map_err(|e| format!("Network query failed: {}", e))?;

    // Find the first adapter with a connection ID (active adapter)
    let adapter = results
        .iter()
        .find(|a| a.net_connection_id.is_some())
        .ok_or("No active network adapter found")?;

    let speed_str = adapter
        .speed
        .map(|s| {
            if s >= 1_000_000_000 {
                format!("{} Gbps", s / 1_000_000_000)
            } else {
                format!("{} Mbps", s / 1_000_000)
            }
        })
        .unwrap_or_else(|| "Unknown".to_string());

    // Check for advanced properties via PowerShell (interrupt moderation, EEE)
    let (supports_int_mod, supports_eee) = check_adapter_advanced_properties(
        adapter.net_connection_id.as_deref().unwrap_or(""),
    );

    Ok(NetworkInfo {
        adapter_name: adapter
            .net_connection_id
            .clone()
            .unwrap_or_else(|| "Unknown".to_string()),
        description: adapter.name.trim().to_string(),
        link_speed: speed_str,
        connection_type: adapter
            .adapter_type
            .clone()
            .unwrap_or_else(|| "Ethernet".to_string()),
        supports_interrupt_mod: supports_int_mod,
        supports_eee: supports_eee,
    })
}

fn check_adapter_advanced_properties(adapter_name: &str) -> (bool, bool) {
    // Shell out to PowerShell to check NIC advanced properties (per ENG-2)
    let output = std::process::Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            &format!(
                "Get-NetAdapterAdvancedProperty -Name '{}' -ErrorAction SilentlyContinue | \
                 Select-Object -ExpandProperty DisplayName",
                adapter_name
            ),
        ])
        .output();

    match output {
        Ok(out) => {
            let text = String::from_utf8_lossy(&out.stdout);
            let supports_int_mod = text.contains("Interrupt Moderation");
            let supports_eee = text.contains("Energy Efficient Ethernet")
                || text.contains("Energy-Efficient Ethernet");
            (supports_int_mod, supports_eee)
        }
        Err(_) => (false, false), // Graceful degrade
    }
}

fn detect_os(wmi: &wmi::WMIConnection) -> Result<OsInfo, String> {
    let results: Vec<WmiOs> = wmi.query().map_err(|e| format!("OS query failed: {}", e))?;
    let os = results.first().ok_or("No OS info found")?;

    let edition = if os.caption.contains("Pro") {
        "Pro"
    } else if os.caption.contains("Enterprise") {
        "Enterprise"
    } else if os.caption.contains("Home") {
        "Home"
    } else {
        "Unknown"
    };

    let is_desktop = !std::path::Path::new("C:\\Windows\\System32\\BatteryLife.dll").exists()
        || std::process::Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                "(Get-CimInstance Win32_Battery -ErrorAction SilentlyContinue) -eq $null",
            ])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "True")
            .unwrap_or(true);

    Ok(OsInfo {
        caption: os.caption.trim().to_string(),
        version: os.version.clone(),
        build_number: os.build_number.clone(),
        edition: edition.to_string(),
        is_desktop,
    })
}

// ─── Game Detection ─────────────────────────────────────────────────────────

/// Known game executables for IFEO priority (fallback when Steam scan incomplete)
const KNOWN_GAMES: &[(&str, &str)] = &[
    ("Counter-Strike 2", "cs2.exe"),
    ("Battlefield 2042", "BF2042.exe"),
    ("Halo Infinite", "HaloInfinite.exe"),
    ("Red Dead Redemption 2", "RDR2.exe"),
    ("Deadlock", "deadlock.exe"),
    ("Valorant", "VALORANT-Win64-Shipping.exe"),
    ("Apex Legends", "r5apex.exe"),
    ("Fortnite", "FortniteClient-Win64-Shipping.exe"),
    ("Overwatch 2", "Overwatch.exe"),
    ("Call of Duty", "cod.exe"),
    ("PUBG", "TslGame.exe"),
    ("Rainbow Six Siege", "RainbowSix.exe"),
    ("Escape from Tarkov", "EscapeFromTarkov.exe"),
    ("League of Legends", "League of Legends.exe"),
    ("Dota 2", "dota2.exe"),
    ("Rocket League", "RocketLeague.exe"),
];

fn scan_games() -> Vec<DetectedGame> {
    let mut games = Vec::new();

    // Try Steam library scan
    if let Some(steam_games) = scan_steam_library() {
        games.extend(steam_games);
    }

    // Add known games that exist on disk as IFEO targets
    for (name, exe) in KNOWN_GAMES {
        // Check if already found via Steam
        if games.iter().any(|g| g.exe_name == *exe) {
            continue;
        }
        // Check common paths
        let ifeo_path = format!(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\{}",
            exe
        );
        // If the game exe is registered in IFEO or exists in Program Files, add it
        if windows_registry::LOCAL_MACHINE
            .open(&ifeo_path)
            .is_ok()
        {
            games.push(DetectedGame {
                name: name.to_string(),
                exe_name: exe.to_string(),
                install_path: String::new(),
                source: GameSource::KnownList,
            });
        }
    }

    games
}

fn scan_steam_library() -> Option<Vec<DetectedGame>> {
    // Find Steam install path from registry
    let steam_key = windows_registry::LOCAL_MACHINE
        .open(r"SOFTWARE\Valve\Steam")
        .or_else(|_| {
            windows_registry::LOCAL_MACHINE
                .open(r"SOFTWARE\WOW6432Node\Valve\Steam")
        })
        .ok()?;

    let install_path: String = steam_key.get_string("InstallPath").ok()?;
    let steamapps = std::path::Path::new(&install_path).join("steamapps");

    if !steamapps.exists() {
        return None;
    }

    let mut games = Vec::new();

    // Read appmanifest_*.acf files to find installed games
    if let Ok(entries) = std::fs::read_dir(&steamapps) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("appmanifest_") && name.ends_with(".acf") {
                    if let Some(game) = parse_acf_manifest(&path, &steamapps) {
                        games.push(game);
                    }
                }
            }
        }
    }

    Some(games)
}

fn parse_acf_manifest(
    path: &std::path::Path,
    steamapps: &std::path::Path,
) -> Option<DetectedGame> {
    let content = std::fs::read_to_string(path).ok()?;

    // Simple ACF parser — extract "name" and "installdir"
    let name = extract_acf_value(&content, "name")?;
    let installdir = extract_acf_value(&content, "installdir")?;

    let game_path = steamapps.join("common").join(&installdir);
    if !game_path.exists() {
        return None;
    }

    // Try to find the main exe — look for known games first
    let exe_name = KNOWN_GAMES
        .iter()
        .find(|(n, _)| name.contains(n) || n.contains(&name.as_str()))
        .map(|(_, exe)| exe.to_string())
        .unwrap_or_else(|| {
            // Fallback: find first .exe in the game directory
            find_main_exe(&game_path).unwrap_or_else(|| format!("{}.exe", installdir))
        });

    Some(DetectedGame {
        name,
        exe_name,
        install_path: game_path.to_string_lossy().to_string(),
        source: GameSource::Steam,
    })
}

fn extract_acf_value(content: &str, key: &str) -> Option<String> {
    // ACF format: "key"    "value"
    let pattern = format!("\"{}\"", key);
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(&pattern) {
            // Find the second quoted string
            let after_key = &trimmed[pattern.len()..];
            if let Some(start) = after_key.find('"') {
                if let Some(end) = after_key[start + 1..].find('"') {
                    return Some(after_key[start + 1..start + 1 + end].to_string());
                }
            }
        }
    }
    None
}

fn find_main_exe(dir: &std::path::Path) -> Option<String> {
    // Look for .exe files at the top level (bounded scan — max 50 entries)
    let entries: Vec<_> = std::fs::read_dir(dir)
        .ok()?
        .take(50)
        .flatten()
        .collect();

    entries
        .iter()
        .filter_map(|e| {
            let path = e.path();
            if path.extension()?.to_str()? == "exe" {
                Some(path.file_name()?.to_str()?.to_string())
            } else {
                None
            }
        })
        .next()
}
