<!-- GSD:docs-update generated -->
# Testing

> OVERCLOCK — Windows Performance Optimizer

## Current Test Coverage

OVERCLOCK v0.1.0 uses manual integration testing on real Windows machines. Automated unit tests are planned for Phase 6.

## Manual Testing Checklist

### Pre-Test Setup

1. Build the release binary: `npm run tauri build`
2. Locate the installer: `src-tauri/target/release/bundle/nsis/OVERCLOCK_0.1.0_x64-setup.exe`
3. Run on a Windows 10/11 machine (physical or VM with admin access)

### Test Matrix

#### T1: Fresh Launch (No Snapshot)

| Step | Action | Expected |
|------|--------|----------|
| 1 | Launch OVERCLOCK.exe | UAC prompt appears |
| 2 | Grant admin | App opens, detection starts |
| 3 | Wait for detection | 6 hardware cards populate (CPU, GPU, RAM, Storage, Network, OS) |
| 4 | Click Continue | Profile selection screen with 5 cards |
| 5 | Select "Competitive FPS" | Highlight + Continue enabled |
| 6 | Click Continue | Change preview with checkboxes |
| 7 | Deselect one change | Checkbox unchecks, count updates |
| 8 | Click "Select All" | All checkboxes re-check |
| 9 | Click Apply | Progress screen with per-change status |
| 10 | Wait for completion | Results summary with applied/failed/skipped counts |

#### T2: Revert Flow (Snapshot Exists)

| Step | Action | Expected |
|------|--------|----------|
| 1 | Re-launch OVERCLOCK | Revert panel appears (not wizard) |
| 2 | Click "Revert All" | Per-change revert progress |
| 3 | Wait for completion | Revert summary with status per change |
| 4 | Confirm deletion | Snapshot deleted, next launch shows wizard |

#### T3: Error Recovery

| Step | Action | Expected |
|------|--------|----------|
| 1 | Disable WMI service: `Stop-Service winmgmt` | — |
| 2 | Launch OVERCLOCK | Error screen: "Something went wrong" |
| 3 | Click "Try Again" | Re-attempts detection |
| 4 | Re-enable WMI: `Start-Service winmgmt` | — |
| 5 | Click "Try Again" | Detection succeeds, wizard continues |

#### T4: Transition Animations

| Step | Action | Expected |
|------|--------|----------|
| 1 | Navigate forward (Continue) | Slide right-to-left (300ms) |
| 2 | Navigate backward (Back) | Slide left-to-right (300ms) |
| 3 | Click Apply | Crossfade transition (400ms) |
| 4 | Enable `prefers-reduced-motion` in Windows | All animations instant |

#### T5: Per-Change Toggles

| Step | Action | Expected |
|------|--------|----------|
| 1 | Reach ChangePreview | All checkboxes checked by default |
| 2 | Deselect "Disable Nagle" | Checkbox unchecks |
| 3 | Click Apply | Nagle change appears as "Skipped" |
| 4 | Verify registry | Nagle keys unchanged |

### Verification Commands

After applying the "Competitive FPS" profile, verify changes took effect:

```powershell
# Power plan
powercfg /getactivescheme
# Should show "Ultimate Performance" or custom GUID

# Mouse acceleration
Get-ItemProperty "HKCU:\Control Panel\Mouse" | Select-Object MouseSpeed
# Should show MouseSpeed = 0

# Services
Get-Service SysMain, DiagTrack | Select-Object Name, Status
# Both should be Stopped

# Nagle (per interface)
Get-ItemProperty "HKLM:\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces\*" -Name TcpAckFrequency -ErrorAction SilentlyContinue
# Should show TcpAckFrequency = 1
```

After reverting:

```powershell
# Mouse acceleration
Get-ItemProperty "HKCU:\Control Panel\Mouse" | Select-Object MouseSpeed
# Should show MouseSpeed = 1 (restored)

# Services
Get-Service SysMain | Select-Object Name, Status
# Should be Running (restored)
```

## Planned Automated Tests

### Rust Unit Tests (Phase 6)

Target areas for `#[cfg(test)]` modules:

| Module | Test Focus |
|--------|-----------|
| `profiles.rs` | Profile field validation, all 5 profiles readable |
| `types.rs` | Serde roundtrip for all types (serialize → deserialize) |
| `snapshot.rs` | Save/load/delete, version migration, corrupt JSON handling |
| `optimizer.rs` | `PsResult` parsing, `ChangeStatus` enum construction |
| `detector.rs` | VDF parser (with fixture files), known games list completeness |

### Frontend Tests (Phase 6)

Target areas for React Testing Library / Vitest:

| Component | Test Focus |
|-----------|-----------|
| `App.tsx` | Wizard step transitions, error recovery, revert fork |
| `ChangePreview.tsx` | Toggle behavior, select/deselect all, excluded keys |
| `StepTransition.tsx` | Direction classes applied correctly |
| `ProfileSelector.tsx` | Profile selection, game list expand/collapse |

## Type Checking

TypeScript type checking is the primary automated verification available today:

```bash
npx tsc --noEmit
```

This verifies all TypeScript types match Rust types (via the manually maintained mirror in `src/types/index.ts`).

Rust compilation check:

```bash
cd src-tauri && cargo check
```

Both must pass with 0 errors before any commit.
