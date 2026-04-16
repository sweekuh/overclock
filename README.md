# ⚡ OVERCLOCK

**One-click Windows performance optimizer for gaming.**

OVERCLOCK detects your hardware, lets you pick an optimization profile, and applies registry, power, network, and service tweaks — with a full rollback snapshot so you can undo everything.

## Features

- **Hardware Detection** — CPU, GPU, RAM, Storage, Network, OS via WMI  
- **5 Optimization Profiles** — Competitive FPS, Casual Gaming, Video Editing, Streaming, Productivity  
- **Per-Change Control** — Toggle individual optimizations on/off before applying  
- **Safe Rollback** — Snapshot captures all originals before modifying; one-click revert  
- **Game Detection** — Steam library scanner + known game list for IFEO priority boosting  
- **Zero Dependencies** — Single portable `.exe`, no installer required  

## What It Optimizes

| Category | Optimizations |
|----------|--------------|
| **Power** | Ultimate Performance plan, USB Selective Suspend |
| **Network** | Nagle's Algorithm (TCP ACK), Interrupt Moderation, Wake-on-LAN |
| **Input** | Mouse acceleration (flat SPI curve) |
| **Services** | SysMain, DiagTrack, XblAuthManager, WSearch, and more |
| **Process** | IFEO CPU priority for detected games |
| **Background** | Background app restrictions via AppPrivacy |

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Runtime | [Tauri v2](https://v2.tauri.app/) |
| Backend | Rust (WMI, windows-registry, PowerShell) |
| Frontend | React + TypeScript + Vite |
| Design | Precision Dark — custom design system |

## Build from Source

### Prerequisites

- **Rust** 1.75+ with MSVC toolchain
- **Visual Studio 2022 Build Tools** (C++ workload)
- **Node.js** 20+

### Development

```bash
# Install dependencies
npm install

# Run in dev mode (hot-reload)
npm run tauri dev
```

### Production Build

```bash
# Build release .exe + NSIS installer + MSI
npm run tauri build
```

Output: `src-tauri/target/release/OVERCLOCK.exe`  
Installers: `src-tauri/target/release/bundle/nsis/` and `msi/`

## Usage

1. **Run as Administrator** — OVERCLOCK needs admin privileges to modify registry and services
2. **Review hardware** — Verify detected CPU, GPU, RAM, Network, Storage, OS
3. **Pick a profile** — Choose based on your use case (Competitive FPS, Casual, etc.)
4. **Toggle changes** — Deselect any optimizations you don't want
5. **Apply** — Watch per-change progress with live status
6. **Revert anytime** — Next launch detects existing snapshot and offers rollback

## Architecture

```
┌─────────────────────────────────────────────────┐
│                 OVERCLOCK.exe                     │
│                                                   │
│  Frontend (React)          Backend (Rust)         │
│  ┌─────────────────┐      ┌──────────────────┐   │
│  │ DetectScreen     │      │ detector.rs      │   │
│  │ ProfileSelector  │─IPC─▶│ optimizer.rs     │   │
│  │ ChangePreview    │      │ snapshot.rs      │   │
│  │ ApplyProgress    │      │ profiles.rs      │   │
│  │ ResultsSummary   │◀─IPC─│ commands.rs      │   │
│  │ RevertPanel      │      │ types.rs         │   │
│  └─────────────────┘      └──────────────────┘   │
│                                                   │
│  Snapshot: %APPDATA%/overclock/snapshot.json      │
└─────────────────────────────────────────────────┘
```

## Safety

- **Snapshot-first** — Every original value is recorded before modification
- **PsResult exit-code checking** — All PowerShell commands verify success/failure
- **Hardware-conditional** — NIC tweaks only applied if adapter supports them
- **Graceful degradation** — Missing services, absent registry keys handled safely
- **prefers-reduced-motion** — UI respects Windows accessibility settings

## Project Structure

```
overclock-app/
├── src/                    # React frontend
│   ├── components/         # 8 UI components
│   ├── styles/             # Design tokens + animations
│   ├── types/              # TypeScript type mirrors
│   └── App.tsx             # Wizard orchestrator
├── src-tauri/
│   └── src/
│       ├── detector.rs     # WMI + Steam + game detection
│       ├── optimizer.rs    # Registry, PowerShell, services
│       ├── snapshot.rs     # Capture/save/restore/delete
│       ├── profiles.rs     # 5 preset optimization profiles
│       ├── commands.rs     # Tauri IPC exports
│       └── types.rs        # Shared data structures
└── DESIGN.md               # Precision Dark design system
```

## License

Private — All rights reserved.
