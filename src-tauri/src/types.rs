use serde::{Deserialize, Serialize};

// ─── Hardware Detection Types ────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareProfile {
    pub cpu: CpuInfo,
    pub gpu: GpuInfo,
    pub memory: MemoryInfo,
    pub storage: StorageInfo,
    pub network: NetworkInfo,
    pub os: OsInfo,
    pub games: Vec<DetectedGame>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfo {
    pub name: String,
    pub manufacturer: String,
    pub cores: u32,
    pub threads: u32,
    pub max_clock_mhz: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub name: String,
    pub manufacturer: String,
    pub vram_mb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub total_mb: u64,
    pub speed_mhz: u32,
    pub memory_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageInfo {
    pub media_type: StorageType,
    pub total_gb: u64,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StorageType {
    NVMe,
    SSD,
    HDD,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub adapter_name: String,
    pub description: String,
    pub link_speed: String,
    pub connection_type: String,
    pub supports_interrupt_mod: bool,
    pub supports_eee: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsInfo {
    pub caption: String,
    pub version: String,
    pub build_number: String,
    pub edition: String,
    pub is_desktop: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedGame {
    pub name: String,
    pub exe_name: String,
    pub install_path: String,
    pub source: GameSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameSource {
    Steam,
    Manual,
    KnownList,
}

// ─── Optimization Types ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposedChange {
    pub id: String,
    pub category: ChangeCategory,
    pub title: String,
    pub description: String,
    pub current_value: String,
    pub new_value: String,
    pub risk: RiskLevel,
    pub hardware_condition: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeCategory {
    Power,
    Network,
    Input,
    Services,
    Process,
    Background,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Safe,
    Caution,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeResult {
    Applied,
    Skipped { reason: String },
    Failed { error: String },
}

// ─── Profile Types ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub description: String,
    pub power_ultimate: bool,
    pub usb_suspend_disable: bool,
    pub nagle_disable: bool,
    pub interrupt_mod_disable: bool,
    pub wake_on_lan_disable: bool,
    pub mouse_accel_disable: bool,
    pub disable_services: Vec<String>,
    pub process_priority_high: bool,
    pub background_apps_disable: bool,
}

// ─── Snapshot Types ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub version: u32,
    pub timestamp: String,
    pub hardware_fingerprint: String,
    pub profile_applied: String,
    pub changes: Vec<SnapshotEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotEntry {
    pub category: String,
    pub key: String,
    pub original_value: serde_json::Value,
    pub new_value: serde_json::Value,
    pub applied: bool,
}

// ─── App State ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppStep {
    Detecting,
    SelectProfile,
    PreviewChanges,
    Applying,
    Results,
    RevertAvailable,
}
