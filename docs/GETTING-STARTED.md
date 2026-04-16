<!-- GSD:docs-update generated -->
# Getting Started

> OVERCLOCK — Windows Performance Optimizer

## Prerequisites

| Tool | Version | Installation |
|------|---------|-------------|
| **Rust** | 1.75+ | [rustup.rs](https://rustup.rs/) — select MSVC toolchain |
| **Visual Studio Build Tools** | 2022+ | [VS Installer](https://visualstudio.microsoft.com/downloads/) — C++ Desktop workload |
| **Node.js** | 20+ | [nodejs.org](https://nodejs.org/) |
| **WebView2 Runtime** | Latest | Pre-installed on Windows 10 21H2+ and Windows 11 |

### Verify Installation

```powershell
rustc --version    # Should show 1.75+
cargo --version    # Should show 1.75+
node --version     # Should show v20+
npm --version      # Should show 9+
```

## Quick Start (Development)

```bash
# 1. Clone the repository
git clone https://github.com/sweekuh/overclock.git
cd overclock

# 2. Install Node dependencies
npm install

# 3. Run in development mode
npm run tauri dev
```

This starts both the Vite dev server (port 1420) and the Tauri application with hot-reload. Frontend changes reflect instantly; Rust changes trigger a recompile.

> **Note:** The app will prompt for admin elevation on launch. This is required because OVERCLOCK modifies registry keys and system services.

## Quick Start (Production Build)

```bash
# Build optimized release binaries
npm run tauri build
```

Output artifacts:
- `src-tauri/target/release/overclock-app.exe` — Standalone executable (9 MB)
- `src-tauri/target/release/bundle/nsis/OVERCLOCK_0.1.0_x64-setup.exe` — NSIS installer (2 MB)
- `src-tauri/target/release/bundle/msi/OVERCLOCK_0.1.0_x64_en-US.msi` — MSI installer (3.1 MB)

## Using OVERCLOCK

### Step 1: Launch as Administrator

Double-click `OVERCLOCK.exe` or run the installer. The UAC prompt will appear — click **Yes** to grant admin privileges.

### Step 2: Review Hardware

OVERCLOCK automatically detects your hardware via WMI:
- CPU (cores, threads, clock speed)
- GPU (name, VRAM)
- RAM (capacity, speed, type)
- Storage (NVMe/SSD/HDD)
- Network adapter (speed, supported properties)
- OS (edition, build)

Click **Continue** to proceed.

### Step 3: Choose a Profile

Select the optimization profile that matches your use case:

| Profile | Best For | Aggressiveness |
|---------|----------|---------------|
| **Competitive FPS** | CS2, Valorant, Apex | Maximum — disables most background services |
| **Casual Gaming** | Single-player, RPGs | Moderate — keeps useful services |
| **Video Editing** | Premiere, DaVinci | Throughput-focused — keeps I/O services |
| **Streaming** | Gaming + OBS | Balanced — preserves recording pipeline |
| **Productivity** | Office, development | Minimal — keeps everything |

### Step 4: Review & Toggle Changes

Each proposed change is shown with a checkbox. You can:
- **Deselect** individual changes you don't want
- Use **Select All / Deselect All** to toggle everything
- Review risk levels (Low / Medium / High) for each change

### Step 5: Apply

Click **Apply** to execute. A progress bar shows real-time status for each change (Applied ✓ / Failed ✗ / Skipped ⊘).

A snapshot of your original settings is saved automatically to `%APPDATA%/overclock/snapshot.json`.

### Step 6: Revert (Anytime)

On the next launch, OVERCLOCK detects the existing snapshot and offers to revert all changes. Click **Revert** to restore your original settings.

## Troubleshooting

### "Something went wrong" on startup

Click the **Try Again** button. If the error persists:
1. Check you're running with admin privileges
2. Verify WMI service is running: `Get-Service winmgmt`
3. Check Windows Event Viewer for WMI errors

### NIC tweaks show as "Skipped"

Your network adapter doesn't support that specific property. This is normal — OVERCLOCK only applies NIC tweaks that the adapter reports as available.

### Changes not taking effect

Some optimizations (power plan, services) take effect immediately. Others (Nagle, mouse acceleration) require a **reboot** to apply fully.
