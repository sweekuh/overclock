<!-- GSD:docs-update generated -->
# Architecture

> OVERCLOCK — Windows Performance Optimizer  
> Tauri v2 · React · Rust

## System Overview

OVERCLOCK is a desktop application that detects hardware, applies system-level performance optimizations (registry, services, power plans, network adapter settings), and provides full rollback via a snapshot system. It runs as a single-window wizard with 6 steps.

```
┌─────────────────────────────────────────────────────────────┐
│                    OVERCLOCK.exe (Tauri v2)                   │
│                                                               │
│  ┌───────────────────────┐    ┌────────────────────────────┐ │
│  │   Frontend (WebView)   │    │     Backend (Rust)          │ │
│  │                        │    │                             │ │
│  │  React 19 + TypeScript │◄──►│  detector.rs  (WMI/Steam)  │ │
│  │  Vite 7 dev server     │IPC │  optimizer.rs (reg/PS/svc) │ │
│  │  CSS design system     │    │  snapshot.rs  (JSON state)  │ │
│  │                        │    │  profiles.rs  (5 presets)   │ │
│  │  800×600 frameless     │    │  commands.rs  (IPC bridge)  │ │
│  │  custom title bar      │    │  types.rs     (shared API)  │ │
│  └───────────────────────┘    └────────────────────────────┘ │
│                                                               │
│  build.rs: Windows admin manifest (requireAdministrator)      │
│  Snapshot: %APPDATA%/overclock/snapshot.json                  │
└─────────────────────────────────────────────────────────────┘
```

## Layer Boundaries

### Frontend → Backend (Tauri IPC)

All communication uses Tauri v2's type-safe `invoke()` mechanism. The frontend never touches the OS directly.

| IPC Command | Direction | Purpose |
|-------------|-----------|---------|
| `detect_hardware` | FE → BE | Triggers WMI queries, returns `HardwareProfile` |
| `get_profiles` | FE → BE | Returns the 5 preset `Profile` definitions |
| `apply_profile` | FE → BE | Applies optimizations, returns per-change `ApplyResult[]` |
| `revert_snapshot` | FE → BE | Restores all original values from snapshot |
| `check_snapshot` | FE → BE | Returns `bool` — does a snapshot exist? |
| `delete_snapshot` | FE → BE | Removes snapshot file after confirmed revert |

Commands are registered in `commands.rs` and exposed through `tauri::Builder` in `lib.rs`.

### Backend → OS

The Rust backend interacts with Windows through three mechanisms:

1. **WMI (Windows Management Instrumentation)** — Read-only hardware detection via the `wmi` crate. Runs on a dedicated thread with COM STA initialization to avoid threading issues.

2. **Windows Registry** — Direct read/write via the `windows-registry` crate (`HKCU`, `HKLM`). Used for mouse acceleration, Nagle's algorithm, background app settings, and IFEO game priority.

3. **PowerShell subprocess** — For operations that require cmdlet access: `powercfg.exe` (power plans), `sc.exe` (services), `Set-NetAdapterAdvancedProperty` (NIC tuning), `Get-ChildItem` (registry enumeration). All PS commands use `PsResult` with exit-code checking — never string matching.

## Module Architecture

### `detector.rs` — Hardware Detection

Queries 6 hardware categories via WMI:
- **CPU**: Name, cores, threads, max clock
- **GPU**: Name, VRAM, driver version
- **RAM**: Total capacity, speed, type (DDR4/DDR5)
- **Storage**: Type detection (NVMe → SSD → HDD)
- **Network**: Adapter name, link speed, advanced property enumeration
- **OS**: Edition, build number, architecture

Also includes:
- **Steam Scanner**: Reads `libraryfolders.vdf`, parses `appmanifest_*.acf` files to discover installed games
- **Known Games Fallback**: 16 hardcoded game executables for IFEO priority boosting
- **NIC Property Discovery**: `Get-NetAdapterAdvancedProperty` to detect which tweaks the adapter supports

### `optimizer.rs` — Optimization Engine

The core engine that applies and reverts changes. Key design decisions:

- **Snapshot-first**: Every `apply_*` function captures the original value before writing
- **PsResult validation**: All PowerShell commands return `PsResult { stdout, stderr, success }` — the `success` field is derived from `ExitStatus::success()`, never from parsing stdout
- **Per-change granularity**: Each optimization returns an `ApplyResult` with `ChangeStatus::Applied | Failed(String) | Skipped(String)`

Optimization categories:
1. `apply_power_plan()` — Creates/activates Ultimate Performance plan
2. `apply_nagle()` — Disables Nagle per network interface
3. `apply_nic_settings()` — Interrupt Moderation, Wake-on-LAN, Energy Efficient Ethernet
4. `apply_mouse_accel()` — Flat SPI curve via registry
5. `apply_services()` — Disables telemetry, search indexing, Xbox services
6. `apply_process_priority()` — IFEO entries for detected games
7. `apply_background_apps()` — AppPrivacy registry key

### `snapshot.rs` — State Persistence

Manages the rollback snapshot at `%APPDATA%/overclock/snapshot.json`:
- `capture()` — Collects all `SnapshotEntry` records from apply results
- `save()` — Serializes to JSON with timestamp and hardware fingerprint
- `load()` — Deserializes with version migration scaffold
- `delete()` — Removes snapshot file after successful revert
- `has_snapshot()` — Quick existence check for launch-time fork

### `profiles.rs` — Preset Definitions

Five `const` profile definitions, each specifying:
- Power plan configuration
- Network optimizations (Nagle, interrupt mod, WoL)
- Input settings (mouse acceleration)
- Services to disable
- Process priority for games
- Background app restrictions

Profiles: `Competitive FPS`, `Casual Gaming`, `Video Editing`, `Streaming`, `Productivity`

### `types.rs` — Shared Data Structures

All types derive `Serialize` + `Deserialize` for Tauri IPC. Key types:
- `HardwareProfile` — Detection results from all 6 categories
- `Profile` — Preset optimization configuration
- `SnapshotEntry` — Single captured original value
- `ApplyResult` — Per-change outcome with status
- `ChangeStatus` — Applied / Failed / Skipped enum

## Frontend Architecture

### Wizard State Machine

`App.tsx` manages a linear wizard with 6 states:

```
detecting → profile → preview → applying → results
                                              ↓
                                          [revert fork on next launch]
```

State is managed with React `useState`. Navigation direction (forward/backward/commit) is tracked via `useRef` for the transition system.

### Component Hierarchy

```
App.tsx (wizard orchestrator)
├── TitleBar.tsx (custom frameless window controls)
├── StepTransition.tsx (directional animation wrapper)
│   ├── DetectScreen.tsx (hardware cards display)
│   ├── ProfileSelector.tsx (profile cards + game list)
│   ├── ChangePreview.tsx (toggleable change rows)
│   ├── ApplyProgress.tsx (per-change progress bar)
│   ├── ResultsSummary.tsx (success/fail summary)
│   └── RevertPanel.tsx (snapshot restore UI)
└── Error Screen (inline, with retry button)
```

### Transition System

CSS-only directional transitions via `StepTransition` wrapper:
- **Forward**: Slide right-to-left (300ms enter, 200ms exit)
- **Backward**: Slide left-to-right (same timing)
- **Commit**: Crossfade (400ms) — used for the Apply step
- **Reduced motion**: All animations collapse to instant via `prefers-reduced-motion`

### Design System

The "Precision Dark" system is defined in `DESIGN.md` with CSS variables in `variables.css`:
- Color tokens: `--bg-*`, `--text-*`, `--accent-*`, `--status-*`
- Spacing scale: `--space-xs` through `--space-3xl`
- Typography: System font stack with weight tokens
- Motion tokens: `--duration-*`, `--ease-*` for each transition type

## Data Flow

### Apply Flow (Critical Path)

```
User clicks "Apply"
  → App.tsx calls invoke("apply_profile", { profile, excludedKeys })
  → commands.rs::apply_profile()
    → optimizer.rs::apply_profile()
      → For each optimization:
        1. Read current value (snapshot capture)
        2. Write new value (registry/PS/service)
        3. Return ApplyResult { title, status, snapshot_entry }
      → snapshot.rs::save(all entries)
    → Return Vec<ApplyResult> to frontend
  → App.tsx updates progress bar per result
  → Navigate to ResultsSummary
```

### Revert Flow

```
App launch → check_snapshot() returns true
  → Show RevertPanel instead of wizard
  → User clicks "Revert"
  → invoke("revert_snapshot")
    → snapshot.rs::load()
    → For each SnapshotEntry:
      → optimizer.rs::revert_registry() or revert_service() or revert_nic()
      → Return ApplyResult with success/failure
    → Return Vec<ApplyResult>
  → Show per-change revert status
  → User confirms → delete_snapshot()
```

## Security Model

- **Admin privilege required**: `build.rs` embeds a Windows manifest with `requireAdministrator`. The app refuses to launch without elevation.
- **No network access**: The app never phones home. All operations are local.
- **Snapshot integrity**: Snapshots are plain JSON — no encryption needed since they contain only registry paths and values, not secrets.
- **PowerShell execution**: Commands use `-Command` flag, not script files. No `.ps1` files are executed from disk.

## Build Pipeline

```
npm run tauri build
  → Vite builds React → dist/
  → Cargo builds Rust → target/release/overclock-app.exe
  → Tauri bundles:
    → NSIS installer (~2 MB)
    → MSI installer (~3.1 MB)
    → Standalone exe (~9 MB)
```

The standalone exe embeds the WebView2 runtime reference — it uses the system-installed Edge WebView2 (present on all Windows 10/11 machines).
