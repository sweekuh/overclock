use crate::detector;
use crate::optimizer;
use crate::profiles;
use crate::snapshot;
use crate::types::*;

/// Detect all hardware and installed games
#[tauri::command]
pub fn detect_hardware() -> Result<HardwareProfile, String> {
    detector::detect_hardware()
}

/// Get all available optimization profiles
#[tauri::command]
pub fn get_profiles() -> Vec<Profile> {
    profiles::all_profiles()
}

/// Check if running with admin privileges (ENG-4 guard)
#[tauri::command]
pub fn check_admin() -> bool {
    is_elevated()
}

/// Check if a previous snapshot exists
#[tauri::command]
pub fn check_snapshot() -> Option<SnapshotInfo> {
    if let Some(snap) = snapshot::has_snapshot() {
        Some(SnapshotInfo {
            exists: true,
            timestamp: snap.timestamp,
            profile: snap.profile_applied,
            change_count: snap.changes.len(),
        })
    } else {
        None
    }
}

/// Apply a profile to the system — real registry writes + snapshot capture
#[tauri::command]
pub fn apply_profile(profile_id: String, excluded_keys: Option<Vec<String>>) -> Result<Vec<ApplyResultIpc>, String> {
    let profiles = profiles::all_profiles();
    let profile = profiles.iter()
        .find(|p| p.id == profile_id)
        .ok_or_else(|| format!("Profile '{}' not found", profile_id))?;

    // SECURITY FIX: Re-detect hardware securely on the backend instead of trusting frontend input
    let hardware = detector::detect_hardware()?;

    let excluded = excluded_keys.unwrap_or_default();
    let results = optimizer::apply_profile(profile, &hardware, &excluded);

    // Build snapshot entries from successful applies
    let snapshot_entries: Vec<SnapshotEntry> = results.iter()
        .filter_map(|r| r.snapshot_entry.clone())
        .collect();

    // Save the snapshot
    if !snapshot_entries.is_empty() {
        let snap = snapshot::capture_snapshot(&profile_id, snapshot_entries);
        snapshot::save_snapshot(&snap)?;
    }

    // Convert to IPC-friendly results
    let ipc_results: Vec<ApplyResultIpc> = results.into_iter().map(|r| {
        let (status, message) = match r.status {
            optimizer::ChangeStatus::Applied => ("applied".to_string(), None),
            optimizer::ChangeStatus::Skipped(reason) => ("skipped".to_string(), Some(reason)),
            optimizer::ChangeStatus::Failed(err) => ("failed".to_string(), Some(err)),
        };
        ApplyResultIpc {
            title: r.title,
            status,
            message,
        }
    }).collect();

    Ok(ipc_results)
}

/// Revert all changes from the saved snapshot
#[tauri::command]
pub fn revert_snapshot() -> Result<Vec<ApplyResultIpc>, String> {
    let snap = snapshot::has_snapshot()
        .ok_or("No snapshot found to revert")?;

    let results = optimizer::revert_snapshot(&snap);

    // Convert to IPC-friendly results
    let ipc_results: Vec<ApplyResultIpc> = results.into_iter().map(|r| {
        let (status, message) = match r.status {
            optimizer::ChangeStatus::Applied => ("applied".to_string(), None),
            optimizer::ChangeStatus::Skipped(reason) => ("skipped".to_string(), Some(reason)),
            optimizer::ChangeStatus::Failed(err) => ("failed".to_string(), Some(err)),
        };
        ApplyResultIpc {
            title: r.title,
            status,
            message,
        }
    }).collect();

    // Delete the snapshot after successful revert
    snapshot::delete_snapshot()?;

    Ok(ipc_results)
}

// ─── IPC Types ──────────────────────────────────────────────────────────────

#[derive(serde::Serialize)]
pub struct SnapshotInfo {
    pub exists: bool,
    pub timestamp: String,
    pub profile: String,
    pub change_count: usize,
}

#[derive(serde::Serialize)]
pub struct ApplyResultIpc {
    pub title: String,
    pub status: String,  // "applied" | "skipped" | "failed"
    pub message: Option<String>,
}

// ─── Internal Helpers ───────────────────────────────────────────────────────

fn is_elevated() -> bool {
    #[cfg(target_os = "windows")]
    {
        use windows::Win32::Foundation::HANDLE;
        use windows::Win32::Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
        use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

        unsafe {
            let mut token = HANDLE::default();
            if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).is_ok() {
                let mut elevation = TOKEN_ELEVATION::default();
                let mut size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;
                if GetTokenInformation(
                    token,
                    TokenElevation,
                    Some(&mut elevation as *mut _ as *mut _),
                    size,
                    &mut size,
                )
                .is_ok()
                {
                    return elevation.TokenIsElevated != 0;
                }
            }
        }
    }

    false
}
