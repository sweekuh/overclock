<!-- GSD:docs-update generated -->
# Configuration

> OVERCLOCK — Windows Performance Optimizer

## Configuration Files

OVERCLOCK uses minimal configuration — most behavior is determined at runtime based on detected hardware and selected profile.

### Tauri Configuration

**File:** `src-tauri/tauri.conf.json`

| Key | Value | Purpose |
|-----|-------|---------|
| `productName` | `OVERCLOCK` | Display name in title bar and installers |
| `version` | `0.1.0` | App version, synced with `VERSION` file |
| `identifier` | `com.overclock.app` | Unique app identifier for OS registration |
| `app.windows[0].width` | `800` | Default window width (px) |
| `app.windows[0].height` | `600` | Default window height (px) |
| `app.windows[0].minWidth` | `700` | Minimum resize width |
| `app.windows[0].minHeight` | `500` | Minimum resize height |
| `app.windows[0].decorations` | `false` | Frameless window (custom title bar) |
| `app.windows[0].center` | `true` | Center on screen at launch |
| `bundle.targets` | `["nsis", "msi"]` | Installer formats to produce |
| `app.security.csp` | `null` | No Content Security Policy (local app) |

### Tauri Capabilities

**File:** `src-tauri/capabilities/default.json`

Defines which IPC commands are permitted. All 6 commands (`detect_hardware`, `get_profiles`, `apply_profile`, `revert_snapshot`, `check_snapshot`, `delete_snapshot`) must be listed here or they will be silently blocked at runtime.

### Vite Configuration

**File:** `vite.config.ts`

| Setting | Value | Why |
|---------|-------|-----|
| `server.port` | `1420` | Fixed port — Tauri expects this exact port |
| `server.strictPort` | `true` | Fail if port unavailable (no fallback) |
| `server.watch.ignored` | `**/src-tauri/**` | Don't trigger HMR on Rust changes |
| `clearScreen` | `false` | Preserve Rust compiler output in terminal |

### TypeScript Configuration

**File:** `tsconfig.json`

Standard React + Vite TypeScript config with strict mode enabled. Key settings:
- `target`: `ES2020`
- `strict`: `true`
- `moduleResolution`: `bundler`
- `jsx`: `react-jsx`

## Runtime Configuration

### Snapshot Location

The snapshot file is stored at:
```
%APPDATA%/overclock/snapshot.json
```

This path is resolved via Rust's `dirs` crate (`data_dir()`) at runtime. The directory is created automatically on first snapshot save.

### Profile Definitions

Profiles are hardcoded as `const` values in `src-tauri/src/profiles.rs`. There is no external configuration file for profiles. To add or modify profiles, edit the Rust source and rebuild.

Each profile controls:

| Field | Type | Description |
|-------|------|-------------|
| `name` | `&str` | Display name shown in UI |
| `description` | `&str` | One-line summary of use case |
| `power_plan` | `bool` | Whether to activate Ultimate Performance |
| `disable_usb_suspend` | `bool` | Disable USB selective suspend |
| `disable_nagle` | `bool` | Disable Nagle's algorithm per interface |
| `disable_interrupt_mod` | `bool` | Disable NIC interrupt moderation |
| `disable_wake_on_lan` | `bool` | Disable Wake-on-LAN |
| `disable_mouse_accel` | `bool` | Set flat mouse SPI curve |
| `services_to_disable` | `Vec<&str>` | Windows service names to stop and disable |
| `game_priority` | `bool` | Set High CPU priority for detected games |
| `disable_background_apps` | `bool` | Restrict background app activity |

### Admin Manifest

**File:** `src-tauri/build.rs`

The build script embeds a Windows application manifest that requests `requireAdministrator` elevation. This is compiled into the exe — there is no way to run OVERCLOCK without admin privileges.

## CSS Design Tokens

**File:** `src/styles/variables.css`

All visual configuration is centralized in CSS custom properties:

### Colors
| Token | Default | Purpose |
|-------|---------|---------|
| `--bg-primary` | `#0a0a0f` | Main background |
| `--bg-secondary` | `#12121a` | Card/panel background |
| `--bg-tertiary` | `#1a1a2e` | Elevated surfaces |
| `--text-primary` | `#e8e8f0` | Primary text |
| `--text-secondary` | `#8888a0` | Secondary/muted text |
| `--accent-primary` | `#6c5ce7` | Primary accent (purple) |
| `--accent-hover` | `#7c6cf7` | Hover state |
| `--status-success` | `#00d68f` | Applied/success indicators |
| `--status-error` | `#ff6b6b` | Failed/error indicators |
| `--status-warning` | `#ffd93d` | Warning indicators |

### Spacing
| Token | Value |
|-------|-------|
| `--space-xs` | `4px` |
| `--space-sm` | `8px` |
| `--space-md` | `16px` |
| `--space-lg` | `24px` |
| `--space-xl` | `32px` |
| `--space-2xl` | `48px` |
| `--space-3xl` | `64px` |

### Motion
| Token | Value | Purpose |
|-------|-------|---------|
| `--duration-exit` | `200ms` | Outgoing step transition |
| `--duration-enter` | `300ms` | Incoming step transition |
| `--duration-commit` | `400ms` | Apply (crossfade) transition |
| `--duration-stagger` | `50ms` | Delay between list items |
| `--ease-exit` | `ease-in` | Departing content easing |
| `--ease-enter` | `cubic-bezier(0.2, 0, 0, 1)` | Arriving content easing |
| `--ease-commit` | `ease-in-out` | Crossfade easing |

All motion tokens collapse to `0ms` when `prefers-reduced-motion: reduce` is active.

## Environment Variables

OVERCLOCK does not use environment variables at runtime. All configuration is embedded at build time or determined dynamically from hardware detection.

The Vite dev server respects `TAURI_DEV_HOST` for remote development (e.g., testing in a VM), but this is a development-only concern.
