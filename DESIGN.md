# Design System — OVERCLOCK

> "Precision Dark" — Technical precision. Not gaming RGB vomit. Not SaaS corporate.
> Think: oscilloscope UI, fighter jet HUD, MSI Afterburner.

## Product Context

**What:** Single-exe Windows PC optimizer for gamers and power users
**Type:** Desktop utility (Tauri v2, 800×600 window)
**Audience:** Competitive gamers, PC enthusiasts
**Tone:** Precise, technical, trustworthy. No fluff, no marketing.

---

## Color Palette

### Surfaces
| Token | Hex | Usage |
|-------|-----|-------|
| `--bg-base` | `#07080C` | App background |
| `--bg-surface` | `#0F1117` | Card backgrounds |
| `--bg-elevated` | `#161820` | Hover states, modals, active cards |
| `--bg-title-bar` | `#0A0B10` | Custom title bar |

### Borders
| Token | Hex | Usage |
|-------|-----|-------|
| `--border-default` | `#1E2029` | Subtle dividers, card borders |
| `--border-focus` | `#2A2D38` | Focus rings, active borders |
| `--border-accent` | `#3B82F640` | Selected card border (accent 25%) |

### Text
| Token | Hex | Usage |
|-------|-----|-------|
| `--text-primary` | `#E2E4E9` | Main text (off-white, never #FFF) |
| `--text-secondary` | `#9CA3AF` | Descriptions, help text |
| `--text-muted` | `#6B7080` | Timestamps, tertiary labels |
| `--text-accent` | `#60A5FA` | Links, active items |

### Semantic
| Token | Hex | Usage |
|-------|-----|-------|
| `--accent` | `#3B82F6` | CTAs, focus rings, selected states |
| `--accent-glow` | `#3B82F620` | Box-shadow glow (12% opacity) |
| `--success` | `#22C55E` | Applied, running, safe badge |
| `--warning` | `#EAB308` | Caution badge, skipped items |
| `--danger` | `#EF4444` | Error, failed items |

### Rules
- Never use pure white (`#FFF`) for text — max is `#E2E4E9`
- Never use pure black (`#000`) for backgrounds — min is `#07080C`
- All accent colors desaturated vs. standard Tailwind equivalents
- Glow effects via `box-shadow`, never `filter: drop-shadow`

---

## Typography

### Fonts
| Role | Family | Source |
|------|--------|--------|
| **Headings** | JetBrains Mono | Google Fonts |
| **Body** | Inter | Google Fonts |
| **Code/Values** | JetBrains Mono | Google Fonts |

### Scale (px)
```
12 — caption, badges
13 — small labels  
14 — body small, table cells
16 — body default
20 — section headings (h3)
24 — page headings (h2)
32 — app title (h1)
```

### Weights
```
400 — body text
500 — labels, navigation
600 — headings, emphasis
700 — (reserved, rarely used)
```

### Rules
- Labels: 11px uppercase, `letter-spacing: 0.05em`, `font-weight: 500`
- Numbers: `font-variant-numeric: tabular-nums` on all data displays
- No letterspacing on lowercase body text
- Line height: 1.5 body, 1.2 headings

---

## Spacing

### Scale (4px base)
```
4   — xs (tight gaps, badge padding)
8   — sm (inline spacing, icon gaps)  
12  — md (card inner padding-x)
16  — lg (card inner padding-y, section gaps)
24  — xl (between card groups)
32  — 2xl (section separators)
48  — 3xl (page margins)
```

### Rules
- All spacing values must be from this scale
- Related items: 8px gap
- Distinct groups: 24px gap
- Section dividers: 32px gap + 1px `--border-default` line

---

## Border Radius

| Element | Radius |
|---------|--------|
| Cards | 6px |
| Buttons | 4px |
| Badges | 3px |
| Inputs | 4px |

### Rules
- Never use the same large radius on everything (anti-slop)
- Inner radius = outer radius − gap (for nested elements)
- No `border-radius: 9999px` except circular status dots

---

## Shadows & Effects

- **No drop shadows.** This is a dark app — shadows are invisible.
- **Glow:** `box-shadow: 0 0 20px var(--accent-glow)` on selected/focused cards
- **Border highlight:** Replace shadow hierarchy with border opacity changes
- **Status dots:** 8px circles with semantic colors (solid, not outlined)

---

## Motion

| Type | Duration | Easing | CSS Custom Property |
|------|----------|--------|---------------------|
| Micro (hover, focus) | 150ms | `ease-out` | `--duration-micro` |
| Standard (card select) | 250ms | `cubic-bezier(0.16, 1, 0.3, 1)` | `--duration-standard` |
| Step exit | 200ms | `cubic-bezier(0.7, 0, 0.84, 0)` | `--duration-exit` |
| Step enter | 300ms | `cubic-bezier(0.16, 1, 0.3, 1)` | `--duration-enter` |
| Commit (apply step) | 200ms | `cubic-bezier(0.33, 1, 0.68, 1)` | `--duration-commit` |
| Stagger delay | 50ms | — | `--delay-stagger` |

### Wizard Step Transitions (StepTransition wrapper)

| Direction | Exit Animation | Enter Animation |
|-----------|---------------|-----------------|
| **Forward (→)** | slide-out-left (0→-24px, fade out) | slide-in-right (24px→0, fade in) |
| **Backward (←)** | slide-out-right (0→24px, fade out) | slide-in-left (-24px→0, fade in) |
| **Commit** | crossfade only (no slide) | crossfade only (no slide) |

### Rules
- Only animate `transform` and `opacity` — never layout properties
- Forward → slide right, Back → slide left (spatial metaphor)
- Apply step uses "commit" crossfade — no slide (decisive action)
- Exit finishes faster (200ms) than entry (300ms) — old content departs quickly
- 50ms stagger: new step starts arriving before old fully leaves
- `prefers-reduced-motion`: disable all transitions, instant state changes
- Never use `transition: all` — list properties explicitly

---

## Component Patterns

### Hardware Card (DetectScreen)
```
┌─────────────────────┐
│  STORAGE             │  ← 11px label, uppercase, --text-muted
│                      │
│  NVMe SSD            │  ← 16px, --text-primary, font-weight: 600
│  1 TB                │  ← 14px, --text-secondary
└─────────────────────┘
   bg: --bg-surface
   border: 1px solid --border-default
   padding: 16px
   radius: 6px
```

### Profile Card (ProfileSelector)
```
┌─────────────────────────────────────────────────────────────┐
│  Competitive FPS                                    15 changes │
│  Minimum latency, maximum debloat. For CS2, Valorant, Apex.  │
└─────────────────────────────────────────────────────────────┘
   Default: bg: --bg-surface, border: --border-default
   Hover: bg: --bg-elevated, border: --border-focus
   Selected: border: --border-accent, glow: --accent-glow
   Padding: 16px 20px
   Cursor: pointer
```

### Change Row (ChangePreview)
```
┌─────────────────────────────────────────────────────── ● SAFE ┐
│  Disable Nagle's Algorithm (2 interfaces)                      │
│  TcpAckFrequency: 0→1, TCPNoDelay: 0→1                        │
└────────────────────────────────────────────────────────────────┘
   Risk badges: SAFE (--success), CAUTION (--warning), INFO (--info)
   Badge: 3px radius, 12px font, uppercase, pill shape
```

### Status Row (ApplyProgress)
```
   ⏳ Pending    → --text-muted, no background
   ⚡ Applying   → --accent, subtle pulse animation
   ✅ Applied    → --success, checkmark icon
   ⚠️ Skipped    → --warning, dash icon
   ❌ Failed     → --danger, x icon
```

---

## Window Chrome

- **Frameless window** (`decorations: false` in tauri.conf.json)
- Custom title bar: 36px height, `--bg-title-bar`
- Title: "⚡ OVERCLOCK" in JetBrains Mono 14px `--text-primary`
- Custom window controls (minimize, maximize, close) right-aligned
- Drag region: entire title bar except buttons
- Step indicator in title bar: "Step N of 5" right-aligned, `--text-muted`

---

## Layout

- **Window:** 800×600 default, 700×500 minimum
- **Content area:** 48px padding on sides, 32px top/bottom
- **Hardware grid:** 3 columns × 2 rows, 16px gap
- **Profile list:** Full-width vertical stack, 12px gap
- **Change list:** Full-width vertical stack, 1px border between rows

---

## Anti-Patterns (Banned)

- ❌ Emoji as UI elements (use text labels or monochrome SVG icons)
- ❌ Purple/violet gradients
- ❌ Icons inside colored circles
- ❌ `text-align: center` on body text
- ❌ Uniform bubbly border-radius
- ❌ Decorative blobs, waves, or floating shapes
- ❌ Generic hero copy ("Welcome to...", "Unlock the power of...")
- ❌ `transition: all`
- ❌ Pure white or pure black

---

## Decisions Log

| Date | Decision | Rationale |
|------|----------|-----------|
| 2026-04-14 | "Precision Dark" design language | Gaming tool needs technical credibility, not SaaS polish |
| 2026-04-14 | JetBrains Mono for headings | Monospace communicates "system-level tool" |
| 2026-04-14 | Custom frameless window | Differentiates from generic Electron apps |
| 2026-04-14 | No emoji in UI | Anti-slop measure, maintains professional tone |
| 2026-04-14 | Large selectable cards for profiles | Users need description + change count to make informed choice |
| 2026-04-14 | Per-change status during apply | Transparency builds trust for a tool modifying system settings |
| 2026-04-14 | Slide transitions between steps | Directional movement reinforces wizard progression |
