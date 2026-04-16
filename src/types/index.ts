// ─── TypeScript mirrors of Rust types ────────────────────────────────────────

export interface HardwareProfile {
  cpu: CpuInfo;
  gpu: GpuInfo;
  memory: MemoryInfo;
  storage: StorageInfo;
  network: NetworkInfo;
  os: OsInfo;
  games: DetectedGame[];
}

export interface CpuInfo {
  name: string;
  manufacturer: string;
  cores: number;
  threads: number;
  max_clock_mhz: number;
}

export interface GpuInfo {
  name: string;
  manufacturer: string;
  vram_mb: number;
}

export interface MemoryInfo {
  total_mb: number;
  speed_mhz: number;
  memory_type: string;
}

export interface StorageInfo {
  media_type: "NVMe" | "SSD" | "HDD" | "Unknown";
  total_gb: number;
  model: string;
}

export interface NetworkInfo {
  adapter_name: string;
  description: string;
  link_speed: string;
  connection_type: string;
  supports_interrupt_mod: boolean;
  supports_eee: boolean;
}

export interface OsInfo {
  caption: string;
  version: string;
  build_number: string;
  edition: string;
  is_desktop: boolean;
}

export interface DetectedGame {
  name: string;
  exe_name: string;
  install_path: string;
  source: "Steam" | "Manual" | "KnownList";
}

export interface Profile {
  id: string;
  name: string;
  description: string;
  power_ultimate: boolean;
  usb_suspend_disable: boolean;
  nagle_disable: boolean;
  interrupt_mod_disable: boolean;
  wake_on_lan_disable: boolean;
  mouse_accel_disable: boolean;
  disable_services: string[];
  process_priority_high: boolean;
  background_apps_disable: boolean;
}

export interface SnapshotInfo {
  exists: boolean;
  timestamp: string;
  profile: string;
  change_count: number;
}

export interface ProposedChange {
  id: string;
  category: "Power" | "Network" | "Input" | "Services" | "Process" | "Background";
  title: string;
  description: string;
  current_value: string;
  new_value: string;
  risk: "Safe" | "Caution" | "Info";
}

export type AppStep = "detecting" | "select_profile" | "preview" | "applying" | "results";
