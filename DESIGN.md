# Design System — OVERCLOCK

> Dual-Skin Architecture: Users can toggle between the modern "Precision" utility and the nostalgic "Demoscene" hacker tool. Both skins support Light and Dark modes.

## Product Context

- **What:** Single-exe Windows PC optimizer for gamers and power users
- **Type:** Desktop utility (Tauri v2, 800×600 window)
- **Audience:** Competitive gamers, PC enthusiasts
- **Tone:** Technical, trustworthy, high-performance.

---

## The Theme Engine

OVERCLOCK uses a multi-dimensional theme engine controlled via CSS variables:
1. **Skin:** Dictates the aesthetic structure, typography, and visual effects (Precision vs. Demoscene).
2. **Mode:** Dictates the color palette values (Light vs. Dark).

Switching the skin applies a `data-skin="precision|demoscene"` attribute to the `<body>`.
Switching the mode applies a `data-theme="light|dark"` attribute to the `<body>`.

---

## Skin 1: Precision (The Modern Utility)
*Technical precision. Not gaming RGB vomit. Not SaaS corporate. Think: oscilloscope UI, fighter jet HUD, MSI Afterburner.*

### Aesthetic Direction
- **Decoration:** Minimal. Subtle box-shadow glows.
- **Border Radius:** Rounded (Cards: 6px, Buttons: 4px, Badges: 3px).
- **Effects:** No drop shadows. Focus rings via borders or glows.

### Typography (Precision)
| Role | Family | Source |
|------|--------|--------|
| **Headings / Code** | JetBrains Mono | Google Fonts |
| **Body** | Inter | Google Fonts |

### Color Palette (Precision Dark Mode - Default)
- `--bg-base`: `#07080C`
- `--bg-surface`: `#0F1117`
- `--bg-elevated`: `#161820`
- `--border-default`: `#1E2029`
- `--text-primary`: `#E2E4E9`
- `--text-secondary`: `#9CA3AF`
- `--accent`: `#3B82F6` (Desaturated Blue)
- `--success`: `#22C55E`
- `--warning`: `#EAB308`
- `--danger`: `#EF4444`

---

## Skin 2: Demoscene (The Nostalgic Hacker)
*Retro software installer, early 2000s keygens, and cracktros. Brutalist layout, CRT scanlines, pixel art.*

### Aesthetic Direction
- **Decoration:** Expressive. CRT scanline CSS overlays, ASCII art accents.
- **Border Radius:** None (`0px` globally). Brutalist sharp edges.
- **Effects:** Hard drop shadows (e.g., `4px 4px 0px var(--accent)`), dotted/dashed borders.

### Typography (Demoscene)
| Role | Family | Source |
|------|--------|--------|
| **Display / Hero** | Press Start 2P | Google Fonts |
| **Headings / Labels** | Share Tech Mono | Google Fonts |
| **Body / Terminal** | VT323 | Google Fonts |

### Color Palette (Demoscene Dark Mode)
- `--bg-base`: `#07080C`
- `--bg-surface`: `#0F1117`
- `--bg-elevated`: `#000000`
- `--border-default`: `#3B82F6` (Neon Blue, highly visible borders)
- `--text-primary`: `#E2E4E9`
- `--text-secondary`: `#00FFFF` (Cyan)
- `--accent`: `#3B82F6` (Neon Blue)
- `--accent-alt`: `#FF00FF` (Magenta, used for severe errors or stark contrast)
- `--success`: `#00FF41` (Terminal Green)
- `--warning`: `#FFFF00` (Pure Yellow)
- `--danger`: `#FF00FF` (Magenta)

---

## Window Chrome & Layout (Global)

- **Frameless window** (`decorations: false` in tauri.conf.json)
- Custom title bar: 36px height
- Drag region: entire title bar except window control buttons
- **Window:** 800×600 default, 700×500 minimum
- **Content area:** 48px padding on sides, 32px top/bottom
- **Hardware grid:** 3 columns × 2 rows, 16px gap
- **Profile list:** Full-width vertical stack, 12px gap

---

## Motion (Global)

| Type | Duration | Easing |
|------|----------|--------|
| Micro (hover, focus) | 150ms | `ease-out` |
| Standard (card select) | 250ms | `cubic-bezier(0.16, 1, 0.3, 1)` |
| Step exit | 200ms | `cubic-bezier(0.7, 0, 0.84, 0)` |
| Step enter | 300ms | `cubic-bezier(0.16, 1, 0.3, 1)` |

### Rules
- Only animate `transform` and `opacity` — never layout properties.
- Forward → slide right, Back → slide left (spatial metaphor).
- `prefers-reduced-motion`: disable all transitions, instant state changes.

---

## Anti-Patterns (Banned)

- ❌ Emoji as UI elements (use text labels or monochrome SVG icons)
- ❌ Purple/violet gradients as standard backgrounds
- ❌ Generic hero copy ("Welcome to...", "Unlock the power of...")
- ❌ `transition: all`
- ❌ Centered body text

---

## Decisions Log

| Date | Decision | Rationale |
|------|----------|-----------|
| 2026-04-14 | "Precision Dark" design language | Gaming tool needs technical credibility, not SaaS polish |
| 2026-04-14 | Custom frameless window | Differentiates from generic Electron apps |
| 2026-04-16 | Dual-Skin Theme Engine | Added "Demoscene" keygen aesthetic as a toggleable skin to lean into the cult-classic power-user vibe while preserving the clean "Precision" default. |
