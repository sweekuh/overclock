<!-- GSD:docs-update generated -->
# Development

> OVERCLOCK — Windows Performance Optimizer

## Project Structure

```
overclock-app/
├── src/                          # React frontend
│   ├── App.tsx                   # Wizard orchestrator (state machine)
│   ├── App.css                   # Global styles
│   ├── main.tsx                  # React entry point
│   ├── vite-env.d.ts             # Vite type declarations
│   ├── components/
│   │   ├── TitleBar.tsx          # Custom frameless window title bar
│   │   ├── StepTransition.tsx    # Directional animation wrapper
│   │   ├── DetectScreen.tsx      # Step 1: Hardware detection display
│   │   ├── HardwareCard.tsx      # Individual hardware category card
│   │   ├── ProfileSelector.tsx   # Step 2: Profile selection + game list
│   │   ├── ChangePreview.tsx     # Step 3: Toggleable change review
│   │   ├── ApplyProgress.tsx     # Step 4: Per-change progress bar
│   │   ├── ResultsSummary.tsx    # Step 5: Success/failure summary
│   │   ├── RevertPanel.tsx       # Revert flow (launched from snapshot)
│   │   └── StepProgress.tsx      # Step indicator dots
│   ├── hooks/
│   │   ├── useHardware.ts        # invoke("detect_hardware") wrapper
│   │   ├── useProfiles.ts        # invoke("get_profiles") wrapper
│   │   └── useSnapshot.ts        # invoke("check_snapshot") wrapper
│   ├── types/
│   │   └── index.ts              # TypeScript mirrors of Rust types
│   └── styles/
│       ├── variables.css         # Design tokens (colors, spacing, motion)
│       └── animations.css        # Keyframe definitions for transitions
├── src-tauri/                    # Rust backend
│   ├── Cargo.toml                # Rust dependencies
│   ├── Cargo.lock                # Locked dependency versions
│   ├── tauri.conf.json           # Tauri app configuration
│   ├── build.rs                  # Admin manifest + winresource
│   ├── capabilities/
│   │   └── default.json          # Tauri v2 IPC command permissions
│   ├── icons/                    # App icons for installers
│   └── src/
│       ├── main.rs               # Entry point (admin guard → Tauri builder)
│       ├── lib.rs                # Tauri plugin registration + command exports
│       ├── commands.rs           # #[tauri::command] IPC handlers
│       ├── detector.rs           # WMI queries, Steam scanner, NIC discovery
│       ├── optimizer.rs          # Registry writes, PowerShell, service control
│       ├── snapshot.rs           # JSON snapshot capture/save/load/delete
│       ├── profiles.rs           # 5 const Profile definitions
│       └── types.rs              # Shared data structures (serde)
├── DESIGN.md                     # Precision Dark design system specification
├── CHANGELOG.md                  # Version history
├── VERSION                       # 4-digit version (0.1.0.0)
├── README.md                     # Project overview
├── package.json                  # Node dependencies + scripts
├── tsconfig.json                 # TypeScript configuration
├── tsconfig.node.json            # Node-specific TS config (Vite)
└── vite.config.ts                # Vite build configuration
```

## Development Workflow

### Running the Dev Server

```bash
npm run tauri dev
```

This starts two processes simultaneously:
1. **Vite** dev server on `http://localhost:1420` with HMR
2. **Tauri** application window loading from the dev server

Frontend changes hot-reload instantly. Rust changes trigger a recompile (~2-5 seconds for incremental builds).

### Making Frontend Changes

All UI components are in `src/components/`. The wizard flow is orchestrated by `App.tsx`.

**Adding a new wizard step:**
1. Create the component in `src/components/NewStep.tsx`
2. Add the step name to the `step` state type in `App.tsx`
3. Add a case in the render switch
4. Wrap the component in `<StepTransition>` with the correct `direction` and `stepKey`
5. Wire navigation via `navigateTo("step-name")`

**Modifying design tokens:**
- Edit `src/styles/variables.css` for colors, spacing, or motion values
- See `DESIGN.md` for the complete design system specification

### Making Backend Changes

Rust source is in `src-tauri/src/`. The key file for most changes is `optimizer.rs`.

**Adding a new optimization:**
1. Add the apply function in `optimizer.rs` (follow `apply_nagle()` as a template)
2. Add the revert function in the `revert_*` match arm
3. Add the toggle field to profiles in `profiles.rs`
4. Add the field to `Profile` type in `types.rs` and `src/types/index.ts`
5. Wire the new change into `apply_profile()` in `optimizer.rs`

**Critical rules for optimizer code:**
- Always capture the original value BEFORE writing
- Always check `PsResult.success` — never use `let _ = run_ps(...)`
- Return `ChangeStatus::Failed(reason)` on failure, never false-positive `Applied`
- Include a `SnapshotEntry` for any value that needs to be reverted

### IPC Commands

To add a new Tauri command:

1. Define the function in `commands.rs`:
   ```rust
   #[tauri::command]
   pub fn my_command() -> Result<MyType, String> { ... }
   ```

2. Register in `lib.rs`:
   ```rust
   .invoke_handler(tauri::generate_handler![
       commands::my_command,  // add here
   ])
   ```

3. Add to `capabilities/default.json`:
   ```json
   "permissions": ["core:default", "opener:default", "my_command"]
   ```

4. Call from frontend:
   ```typescript
   const result = await invoke<MyType>("my_command");
   ```

## Code Style

### Rust
- No `let _ = ...` for fallible operations — always check the result
- Use `PsResult` for all PowerShell commands
- `snake_case` for functions and variables
- Module-level documentation comments on public functions

### TypeScript/React
- Functional components with hooks
- Types mirrored from Rust in `src/types/index.ts`
- CSS classes use BEM-style naming: `.component__element--modifier`
- No inline styles except for dynamic values

### CSS
- All values via design tokens from `variables.css`
- No hardcoded colors, spacing, or timing values
- `prefers-reduced-motion` support required for any new animations

## Common Tasks

| Task | Command |
|------|---------|
| Start dev server | `npm run tauri dev` |
| Build production release | `npm run tauri build` |
| Type-check frontend | `npx tsc --noEmit` |
| Check Rust compilation | `cd src-tauri && cargo check` |
| Run Rust with warnings | `cd src-tauri && cargo build 2>&1` |
| Format Rust code | `cd src-tauri && cargo fmt` |
| Lint Rust code | `cd src-tauri && cargo clippy` |
