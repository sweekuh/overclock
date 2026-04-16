# Changelog

All notable changes to OVERCLOCK will be documented in this file.

## [0.1.0.0] - 2026-04-15

### Added
- Hardware detection via WMI (CPU, GPU, RAM, Storage, Network, OS)
- 5 optimization profiles: Competitive FPS, Casual Gaming, Video Editing, Streaming, Productivity
- Registry, PowerShell, and service optimizations (power plan, Nagle, interrupt mod, WoL, mouse accel, IFEO, background apps)
- Per-change toggle checkboxes with Select/Deselect All
- Full rollback via snapshot system (`%APPDATA%/overclock/snapshot.json`)
- Steam library scanner + 16 known games IFEO priority
- Directional wizard transitions (forward/backward/commit) with StepTransition wrapper
- Custom frameless window with title bar controls
- Admin privilege guard at startup
- Expandable game detection panel in ProfileSelector
- Error screen with "Try Again" retry button
- `prefers-reduced-motion` compliant animations
- Precision Dark design system (DESIGN.md)

### Fixed
- `apply_process_priority` now checks PowerShell exit code (was hard-coded `applied: true`)
- Nagle revert tracks per-interface success (was always `Applied`)
- Mouse acceleration revert checks all registry write returns
- NIC revert checks PowerShell exit code (was `let _ =`)
- Generic registry revert returns `Skipped` on fallthrough (was `Applied`)
