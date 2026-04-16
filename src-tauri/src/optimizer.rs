use crate::types::*;

// ─── Optimizer Engine ───────────────────────────────────────────────────────
// Each function: reads current value → records in snapshot → applies new value

pub struct ApplyResult {
    pub title: String,
    pub status: ChangeStatus,
    pub snapshot_entry: Option<SnapshotEntry>,
}

pub enum ChangeStatus {
    Applied,
    Skipped(String),
    Failed(String),
}

/// Apply all changes from a profile, capturing originals into a snapshot.
/// Changes whose key appears in `excluded` are skipped.
pub fn apply_profile(profile: &Profile, hardware: &HardwareProfile, excluded: &[String]) -> Vec<ApplyResult> {
    let mut results = Vec::new();

    // ─── Power ──────────────────────────────────────────────────────────
    if profile.power_ultimate && !excluded.contains(&"power_ultimate".to_string()) {
        results.push(apply_ultimate_performance());
    }
    if profile.usb_suspend_disable && !excluded.contains(&"usb_suspend_disable".to_string()) {
        results.push(apply_usb_suspend_disable());
    }

    // ─── Network ────────────────────────────────────────────────────────
    if profile.nagle_disable && !excluded.contains(&"nagle_disable".to_string()) {
        results.push(apply_nagle_disable());
    }
    if profile.interrupt_mod_disable && hardware.network.supports_interrupt_mod
        && !excluded.contains(&"interrupt_mod_disable".to_string())
    {
        results.push(apply_interrupt_mod_disable(&hardware.network.adapter_name));
    }
    if profile.wake_on_lan_disable && !excluded.contains(&"wake_on_lan_disable".to_string()) {
        results.push(apply_wake_on_lan_disable(&hardware.network.adapter_name));
    }

    // ─── Input ──────────────────────────────────────────────────────────
    if profile.mouse_accel_disable && !excluded.contains(&"mouse_accel_disable".to_string()) {
        results.push(apply_mouse_accel_disable());
    }

    // ─── Services ───────────────────────────────────────────────────────
    for svc in &profile.disable_services {
        let key = format!("service_{}", svc);
        if !excluded.contains(&key) {
            results.push(apply_disable_service(svc));
        }
    }

    // ─── Process ────────────────────────────────────────────────────────
    if profile.process_priority_high && !excluded.contains(&"process_priority_high".to_string()) {
        for game in &hardware.games {
            results.push(apply_process_priority(&game.exe_name));
        }
    }

    // ─── Background ─────────────────────────────────────────────────────
    if profile.background_apps_disable && !excluded.contains(&"background_apps_disable".to_string()) {
        results.push(apply_background_apps_disable());
    }

    results
}

/// Revert all changes from a snapshot
pub fn revert_snapshot(snap: &crate::types::Snapshot) -> Vec<ApplyResult> {
    let mut results = Vec::new();

    for entry in &snap.changes {
        if !entry.applied {
            continue;
        }

        let result = match entry.category.as_str() {
            "power_plan" => revert_power_plan(entry),
            "registry" => revert_registry(entry),
            "service" => revert_service(entry),
            "nic" => revert_nic(entry),
            _ => ApplyResult {
                title: format!("Revert: {}", entry.key),
                status: ChangeStatus::Skipped("Unknown category".to_string()),
                snapshot_entry: None,
            },
        };
        results.push(result);
    }

    results
}

// ─── Individual Optimizations ───────────────────────────────────────────────

fn apply_ultimate_performance() -> ApplyResult {
    // Check if Ultimate Performance plan exists, if not create it
    let output = run_ps("powercfg /list");
    let has_ultimate = output.stdout.contains("e9a42b02-d5df-448d-aa00-03f14749eb61");

    if !has_ultimate {
        let _ = run_ps("powercfg /duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61 2>$null; if ($LASTEXITCODE -ne 0) { powercfg /duplicatescheme 8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c e9a42b02-d5df-448d-aa00-03f14749eb61 }");
    }

    // Get current active plan
    let current = run_ps("(powercfg /getactivescheme) -replace '.*: ','' -replace ' \\(.*',''");
    let current_guid = current.stdout.trim().to_string();

    // Activate Ultimate Performance
    let result = run_ps("powercfg /setactive e9a42b02-d5df-448d-aa00-03f14749eb61");

    if !result.success {
        ApplyResult {
            title: "Activate Ultimate Performance plan".to_string(),
            status: ChangeStatus::Failed(result.stderr),
            snapshot_entry: None,
        }
    } else {
        ApplyResult {
            title: "Activate Ultimate Performance plan".to_string(),
            status: ChangeStatus::Applied,
            snapshot_entry: Some(SnapshotEntry {
                category: "power_plan".to_string(),
                key: "active_scheme".to_string(),
                original_value: serde_json::Value::String(current_guid),
                new_value: serde_json::Value::String("e9a42b02-d5df-448d-aa00-03f14749eb61".to_string()),
                applied: true,
            }),
        }
    }
}

fn apply_usb_suspend_disable() -> ApplyResult {
    // USB selective suspend: registry under power settings
    let key_path = r"SYSTEM\CurrentControlSet\Services\USB";
    let original = read_registry_dword("HKLM", key_path, "DisableSelectiveSuspend");

    let success = write_registry_dword("HKLM", key_path, "DisableSelectiveSuspend", 1);

    ApplyResult {
        title: "Disable USB Selective Suspend".to_string(),
        status: if success { ChangeStatus::Applied } else { ChangeStatus::Failed("Registry write failed".to_string()) },
        snapshot_entry: Some(SnapshotEntry {
            category: "registry".to_string(),
            key: format!("HKLM\\{}\\DisableSelectiveSuspend", key_path),
            original_value: serde_json::json!(original),
            new_value: serde_json::json!(1),
            applied: success,
        }),
    }
}

fn apply_nagle_disable() -> ApplyResult {
    // Find all network interfaces and disable Nagle on each
    let interfaces_path = r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces";

    let output = run_ps(&format!(
        "Get-ChildItem 'HKLM:\\{}' | ForEach-Object {{ $_.PSChildName }}",
        interfaces_path
    ));

    let mut applied_count = 0;
    let mut total = 0;
    let mut originals: Vec<serde_json::Value> = Vec::new();

    for iface_id in output.stdout.lines() {
        let iface_id = iface_id.trim();
        if iface_id.is_empty() {
            continue;
        }
        total += 1;
        let iface_path = format!("{}\\{}", interfaces_path, iface_id);

        // Read current values before overwriting
        let orig_ack = read_registry_dword("HKLM", &iface_path, "TcpAckFrequency");
        let orig_nodelay = read_registry_dword("HKLM", &iface_path, "TCPNoDelay");

        originals.push(serde_json::json!({
            "iface": iface_id,
            "TcpAckFrequency": orig_ack,
            "TCPNoDelay": orig_nodelay
        }));

        let ok1 = write_registry_dword("HKLM", &iface_path, "TcpAckFrequency", 1);
        let ok2 = write_registry_dword("HKLM", &iface_path, "TCPNoDelay", 1);

        if ok1 && ok2 {
            applied_count += 1;
        }
    }

    ApplyResult {
        title: "Disable Nagle's Algorithm".to_string(),
        status: if applied_count > 0 {
            ChangeStatus::Applied
        } else if total == 0 {
            ChangeStatus::Skipped("No network interfaces found".to_string())
        } else {
            ChangeStatus::Failed("Registry writes failed".to_string())
        },
        snapshot_entry: Some(SnapshotEntry {
            category: "registry".to_string(),
            key: "nagle_algorithm".to_string(),
            original_value: serde_json::json!(originals),
            new_value: serde_json::json!({"TcpAckFrequency": 1, "TCPNoDelay": 1}),
            applied: applied_count > 0,
        }),
    }
}

fn apply_interrupt_mod_disable(adapter_name: &str) -> ApplyResult {
    // Read current value before changing
    let current = run_ps(&format!(
        "(Get-NetAdapterAdvancedProperty -Name '{}' -DisplayName 'Interrupt Moderation' -ErrorAction SilentlyContinue).DisplayValue",
        adapter_name
    ));
    let original_val = current.stdout.trim().to_string();
    let original_val = if original_val.is_empty() { "Enabled".to_string() } else { original_val };

    let result = run_ps(&format!(
        "Set-NetAdapterAdvancedProperty -Name '{}' -DisplayName 'Interrupt Moderation' -DisplayValue 'Disabled' -ErrorAction SilentlyContinue",
        adapter_name
    ));

    ApplyResult {
        title: format!("Disable Interrupt Moderation on {}", adapter_name),
        status: if result.success { ChangeStatus::Applied } else { ChangeStatus::Failed(result.stderr) },
        snapshot_entry: Some(SnapshotEntry {
            category: "nic".to_string(),
            key: format!("{}\\InterruptModeration", adapter_name),
            original_value: serde_json::json!(original_val),
            new_value: serde_json::json!("Disabled"),
            applied: result.success,
        }),
    }
}

fn apply_wake_on_lan_disable(adapter_name: &str) -> ApplyResult {
    // Read current WoL state
    let current = run_ps(&format!(
        "(Get-NetAdapterAdvancedProperty -Name '{}' -DisplayName 'Wake on Magic Packet' -ErrorAction SilentlyContinue).DisplayValue",
        adapter_name
    ));
    let original_val = current.stdout.trim().to_string();
    let original_val = if original_val.is_empty() { "Enabled".to_string() } else { original_val };

    let result = run_ps(&format!(
        "Set-NetAdapterAdvancedProperty -Name '{}' -DisplayName 'Wake on Magic Packet' -DisplayValue 'Disabled' -ErrorAction SilentlyContinue; \
         Set-NetAdapterAdvancedProperty -Name '{}' -DisplayName 'Wake on Pattern Match' -DisplayValue 'Disabled' -ErrorAction SilentlyContinue",
        adapter_name, adapter_name
    ));

    ApplyResult {
        title: "Disable Wake-on-LAN".to_string(),
        status: if result.success { ChangeStatus::Applied } else { ChangeStatus::Skipped("WoL settings not found".to_string()) },
        snapshot_entry: Some(SnapshotEntry {
            category: "nic".to_string(),
            key: format!("{}\\WakeOnLAN", adapter_name),
            original_value: serde_json::json!(original_val),
            new_value: serde_json::json!("Disabled"),
            applied: result.success,
        }),
    }
}

fn apply_mouse_accel_disable() -> ApplyResult {
    let key_path = r"Control Panel\Mouse";
    let original_speed = read_registry_string("HKCU", key_path, "MouseSpeed");
    
    let ok1 = write_registry_string("HKCU", key_path, "MouseSpeed", "0");
    let ok2 = write_registry_string("HKCU", key_path, "MouseThreshold1", "0");
    let ok3 = write_registry_string("HKCU", key_path, "MouseThreshold2", "0");

    let success = ok1 && ok2 && ok3;

    // Apply flat SPI curve via SystemParametersInfo
    if success {
        let _ = run_ps(
            "Add-Type -TypeDefinition 'using System;using System.Runtime.InteropServices;public class SPI{[DllImport(\"user32.dll\")]public static extern bool SystemParametersInfo(uint a,uint b,int[] c,uint d);}'; \
             $curve = @(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0); \
             [SPI]::SystemParametersInfo(0x001D, 0, $curve, 0x03)"
        );
    }

    ApplyResult {
        title: "Disable Mouse Acceleration".to_string(),
        status: if success { ChangeStatus::Applied } else { ChangeStatus::Failed("Registry write failed".to_string()) },
        snapshot_entry: Some(SnapshotEntry {
            category: "registry".to_string(),
            key: "mouse_acceleration".to_string(),
            original_value: serde_json::json!({"MouseSpeed": original_speed}),
            new_value: serde_json::json!({"MouseSpeed": "0", "MouseThreshold1": "0", "MouseThreshold2": "0"}),
            applied: success,
        }),
    }
}

fn apply_disable_service(service_name: &str) -> ApplyResult {
    // Get current startup type
    let current = run_ps(&format!(
        "(Get-Service '{}' -ErrorAction SilentlyContinue).StartType",
        service_name
    ));
    let current_type = current.stdout.trim().to_string();

    if current_type.is_empty() || !current.success {
        return ApplyResult {
            title: format!("Disable {}", service_name),
            status: ChangeStatus::Skipped(format!("Service {} not found", service_name)),
            snapshot_entry: None,
        };
    }

    if current_type == "Disabled" {
        return ApplyResult {
            title: format!("Disable {}", service_name),
            status: ChangeStatus::Skipped("Already disabled".to_string()),
            snapshot_entry: None,
        };
    }

    // Stop and disable
    let result = run_ps(&format!(
        "Stop-Service '{}' -Force -ErrorAction SilentlyContinue; Set-Service '{}' -StartupType Disabled -ErrorAction SilentlyContinue",
        service_name, service_name
    ));

    ApplyResult {
        title: format!("Disable {}", service_name),
        status: if result.success { ChangeStatus::Applied } else { ChangeStatus::Failed(result.stderr) },
        snapshot_entry: Some(SnapshotEntry {
            category: "service".to_string(),
            key: service_name.to_string(),
            original_value: serde_json::json!(current_type),
            new_value: serde_json::json!("Disabled"),
            applied: result.success,
        }),
    }
}

fn apply_process_priority(exe_name: &str) -> ApplyResult {
    let ifeo_path = format!(
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\{}",
        exe_name
    );

    // Create the IFEO key if it doesn't exist
    let result = run_ps(&format!(
        "New-Item -Path 'HKLM:\\{}' -Force | Out-Null; \
         New-ItemProperty -Path 'HKLM:\\{}' -Name 'CpuPriorityClass' -Value 3 -PropertyType DWord -Force | Out-Null; \
         New-Item -Path 'HKLM:\\{}\\PerfOptions' -Force | Out-Null; \
         New-ItemProperty -Path 'HKLM:\\{}\\PerfOptions' -Name 'CpuPriorityClass' -Value 3 -PropertyType DWord -Force | Out-Null",
        ifeo_path, ifeo_path, ifeo_path, ifeo_path
    ));

    ApplyResult {
        title: format!("Set {} priority to High", exe_name),
        status: if result.success { ChangeStatus::Applied } else { ChangeStatus::Failed(result.stderr) },
        snapshot_entry: Some(SnapshotEntry {
            category: "registry".to_string(),
            key: format!("IFEO\\{}", exe_name),
            original_value: serde_json::json!(null),
            new_value: serde_json::json!(3),
            applied: result.success,
        }),
    }
}

fn apply_background_apps_disable() -> ApplyResult {
    let key_path = r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy";
    let original = read_registry_dword("HKLM", key_path, "LetAppsRunInBackground");

    let success = write_registry_dword("HKLM", key_path, "LetAppsRunInBackground", 2);

    ApplyResult {
        title: "Disable background apps".to_string(),
        status: if success { ChangeStatus::Applied } else { ChangeStatus::Failed("Registry write failed".to_string()) },
        snapshot_entry: Some(SnapshotEntry {
            category: "registry".to_string(),
            key: format!("HKLM\\{}\\LetAppsRunInBackground", key_path),
            original_value: serde_json::json!(original),
            new_value: serde_json::json!(2),
            applied: success,
        }),
    }
}

// ─── Revert Functions ───────────────────────────────────────────────────────

fn revert_power_plan(entry: &SnapshotEntry) -> ApplyResult {
    if let Some(guid) = entry.original_value.as_str() {
        let result = run_ps(&format!("powercfg /setactive {}", guid));
        ApplyResult {
            title: "Restore original power plan".to_string(),
            status: if result.success { ChangeStatus::Applied } else { ChangeStatus::Failed(result.stderr) },
            snapshot_entry: None,
        }
    } else {
        ApplyResult {
            title: "Restore power plan".to_string(),
            status: ChangeStatus::Skipped("No original GUID saved".to_string()),
            snapshot_entry: None,
        }
    }
}

fn revert_registry(entry: &SnapshotEntry) -> ApplyResult {
    // Nagle revert: delete the keys we added
    if entry.key == "nagle_algorithm" {
        let interfaces_path = r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces";
        let output = run_ps(&format!(
            "Get-ChildItem 'HKLM:\\{}' | ForEach-Object {{ $_.PSChildName }}",
            interfaces_path
        ));
        let mut reverted = 0;
        let mut total = 0;
        for iface in output.stdout.lines() {
            let iface = iface.trim();
            if iface.is_empty() { continue; }
            total += 1;
            let result = run_ps(&format!(
                "Remove-ItemProperty -Path 'HKLM:\\{}\\{}' -Name 'TcpAckFrequency' -ErrorAction SilentlyContinue; \
                 Remove-ItemProperty -Path 'HKLM:\\{}\\{}' -Name 'TCPNoDelay' -ErrorAction SilentlyContinue",
                interfaces_path, iface, interfaces_path, iface
            ));
            if result.success { reverted += 1; }
        }
        return ApplyResult {
            title: "Restore Nagle's Algorithm".to_string(),
            status: if reverted > 0 || total == 0 {
                ChangeStatus::Applied
            } else {
                ChangeStatus::Failed("Failed to remove Nagle registry keys".to_string())
            },
            snapshot_entry: None,
        };
    }

    // Mouse accel revert
    if entry.key == "mouse_acceleration" {
        let mut ok = false;
        if let Some(orig) = entry.original_value.as_object() {
            let speed = orig.get("MouseSpeed").and_then(|v| v.as_str()).unwrap_or("1");
            let ok1 = write_registry_string("HKCU", r"Control Panel\Mouse", "MouseSpeed", speed);
            let ok2 = write_registry_string("HKCU", r"Control Panel\Mouse", "MouseThreshold1", "6");
            let ok3 = write_registry_string("HKCU", r"Control Panel\Mouse", "MouseThreshold2", "10");
            ok = ok1 || ok2 || ok3; // At least one succeeded
        }
        return ApplyResult {
            title: "Restore Mouse Acceleration".to_string(),
            status: if ok { ChangeStatus::Applied } else { ChangeStatus::Failed("Registry write failed".to_string()) },
            snapshot_entry: None,
        };
    }

    // IFEO revert: remove the key
    if entry.key.starts_with("IFEO\\") {
        let exe = entry.key.strip_prefix("IFEO\\").unwrap_or(&entry.key);
        let ifeo_path = format!(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\{}",
            exe
        );
        let _ = run_ps(&format!(
            "Remove-Item -Path 'HKLM:\\{}' -Recurse -ErrorAction SilentlyContinue",
            ifeo_path
        ));
        return ApplyResult {
            title: format!("Remove {} priority override", exe),
            status: ChangeStatus::Applied,
            snapshot_entry: None,
        };
    }

    // Generic registry revert
    let parts: Vec<&str> = entry.key.splitn(2, '\\').collect();
    if parts.len() == 2 {
        let hive = parts[0];
        if let Some(last_sep) = parts[1].rfind('\\') {
            let path = &parts[1][..last_sep];
            let name = &parts[1][last_sep + 1..];
            let success = if entry.original_value.is_null() {
                // Key didn't exist — delete it
                let result = run_ps(&format!(
                    "Remove-ItemProperty -Path '{}:\\{}' -Name '{}' -ErrorAction SilentlyContinue",
                    hive, path, name
                ));
                result.success
            } else if let Some(val) = entry.original_value.as_u64() {
                write_registry_dword(hive, path, name, val as u32)
            } else if let Some(val) = entry.original_value.as_str() {
                write_registry_string(hive, path, name, val)
            } else {
                false
            };
            return ApplyResult {
                title: format!("Revert: {}", entry.key),
                status: if success { ChangeStatus::Applied } else { ChangeStatus::Failed("Registry revert failed".to_string()) },
                snapshot_entry: None,
            };
        }
    }

    ApplyResult {
        title: format!("Revert: {}", entry.key),
        status: ChangeStatus::Skipped("No revert action matched".to_string()),
        snapshot_entry: None,
    }
}

fn revert_service(entry: &SnapshotEntry) -> ApplyResult {
    let svc_name = &entry.key;
    let original_type = entry.original_value.as_str().unwrap_or("Manual");

    let result = run_ps(&format!(
        "Set-Service '{}' -StartupType '{}' -ErrorAction SilentlyContinue",
        svc_name, original_type
    ));

    if original_type != "Disabled" {
        let _ = run_ps(&format!("Start-Service '{}' -ErrorAction SilentlyContinue", svc_name));
    }

    ApplyResult {
        title: format!("Restore {} ({})", svc_name, original_type),
        status: if result.success { ChangeStatus::Applied } else { ChangeStatus::Failed(result.stderr) },
        snapshot_entry: None,
    }
}

fn revert_nic(entry: &SnapshotEntry) -> ApplyResult {
    let parts: Vec<&str> = entry.key.splitn(2, '\\').collect();
    if parts.len() != 2 {
        return ApplyResult {
            title: format!("Revert NIC: {}", entry.key),
            status: ChangeStatus::Skipped("Invalid key format".to_string()),
            snapshot_entry: None,
        };
    }
    let adapter = parts[0];
    let prop = parts[1];

    let original_val = entry.original_value.as_str().unwrap_or("Enabled");

    let display_name = match prop {
        "InterruptModeration" => "Interrupt Moderation",
        "WakeOnLAN" => "Wake on Magic Packet",
        _ => prop,
    };

    let result = run_ps(&format!(
        "Set-NetAdapterAdvancedProperty -Name '{}' -DisplayName '{}' -DisplayValue '{}' -ErrorAction SilentlyContinue",
        adapter, display_name, original_val
    ));

    ApplyResult {
        title: format!("Restore {} on {}", display_name, adapter),
        status: if result.success { ChangeStatus::Applied } else { ChangeStatus::Failed(result.stderr) },
        snapshot_entry: None,
    }
}

// ─── Registry Helpers ───────────────────────────────────────────────────────

fn read_registry_dword(hive: &str, path: &str, name: &str) -> Option<u32> {
    let output = run_ps(&format!(
        "(Get-ItemProperty -Path '{}:\\{}' -Name '{}' -ErrorAction SilentlyContinue).'{}'",
        hive, path, name, name
    ));
    let trimmed = output.stdout.trim();
    if trimmed.is_empty() { None } else { trimmed.parse().ok() }
}

fn read_registry_string(hive: &str, path: &str, name: &str) -> Option<String> {
    let output = run_ps(&format!(
        "(Get-ItemProperty -Path '{}:\\{}' -Name '{}' -ErrorAction SilentlyContinue).'{}'",
        hive, path, name, name
    ));
    let trimmed = output.stdout.trim().to_string();
    if trimmed.is_empty() { None } else { Some(trimmed) }
}

fn write_registry_dword(hive: &str, path: &str, name: &str, value: u32) -> bool {
    let result = run_ps(&format!(
        "New-Item -Path '{}:\\{}' -Force -ErrorAction SilentlyContinue | Out-Null; \
         New-ItemProperty -Path '{}:\\{}' -Name '{}' -Value {} -PropertyType DWord -Force -ErrorAction SilentlyContinue | Out-Null",
        hive, path, hive, path, name, value
    ));
    result.success
}

fn write_registry_string(hive: &str, path: &str, name: &str, value: &str) -> bool {
    let result = run_ps(&format!(
        "New-Item -Path '{}:\\{}' -Force -ErrorAction SilentlyContinue | Out-Null; \
         Set-ItemProperty -Path '{}:\\{}' -Name '{}' -Value '{}' -Force -ErrorAction SilentlyContinue",
        hive, path, hive, path, name, value
    ));
    result.success
}

// ─── PowerShell Runner ──────────────────────────────────────────────────────

struct PsResult {
    stdout: String,
    stderr: String,
    success: bool,
}


fn run_ps(command: &str) -> PsResult {
    let output = std::process::Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", command])
        .output();

    match output {
        Ok(out) => PsResult {
            stdout: String::from_utf8_lossy(&out.stdout).to_string(),
            stderr: String::from_utf8_lossy(&out.stderr).to_string(),
            success: out.status.success(),
        },
        Err(e) => PsResult {
            stdout: String::new(),
            stderr: format!("Error running PowerShell: {}", e),
            success: false,
        },
    }
}
