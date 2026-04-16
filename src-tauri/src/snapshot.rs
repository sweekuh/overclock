use crate::types::*;
use std::path::PathBuf;

// ─── Snapshot Logic ─────────────────────────────────────────────────────────

const SNAPSHOT_VERSION: u32 = 1;

fn snapshot_dir() -> PathBuf {
    let appdata = std::env::var("APPDATA").unwrap_or_else(|_| ".".to_string());
    let dir = PathBuf::from(appdata).join("overclock");
    let _ = std::fs::create_dir_all(&dir);
    dir
}

fn snapshot_path() -> PathBuf {
    snapshot_dir().join("snapshot.json")
}

/// Check if a snapshot already exists
pub fn has_snapshot() -> Option<Snapshot> {
    let path = snapshot_path();
    if path.exists() {
        let content = std::fs::read_to_string(&path).ok()?;
        serde_json::from_str(&content).ok()
    } else {
        None
    }
}

/// Save a new snapshot to disk
pub fn save_snapshot(snapshot: &Snapshot) -> Result<(), String> {
    let path = snapshot_path();
    let json = serde_json::to_string_pretty(snapshot)
        .map_err(|e| format!("Failed to serialize snapshot: {}", e))?;
    std::fs::write(&path, json)
        .map_err(|e| format!("Failed to write snapshot to {}: {}", path.display(), e))?;
    Ok(())
}

/// Delete the current snapshot (after successful revert)
pub fn delete_snapshot() -> Result<(), String> {
    let path = snapshot_path();
    if path.exists() {
        std::fs::remove_file(&path)
            .map_err(|e| format!("Failed to delete snapshot: {}", e))?;
    }
    Ok(())
}

/// Create a snapshot from the current system state before applying changes
pub fn capture_snapshot(profile_id: &str, entries: Vec<SnapshotEntry>) -> Snapshot {
    let now = chrono::Local::now();
    Snapshot {
        version: SNAPSHOT_VERSION,
        timestamp: now.format("%Y-%m-%dT%H:%M:%S").to_string(),
        hardware_fingerprint: String::new(), // TODO: compute from HW
        profile_applied: profile_id.to_string(),
        changes: entries,
    }
}
