<p align="center">
  <img src="assets/icon.png" width="200" alt="Fox on Desk">
</p>
<h1 align="center">Fox on Desk</h1>
<p align="center">
  A lightweight desktop pet that mirrors your AI coding agent in real time
  <br>
  <a href="README.zh-CN.md">中文版</a>
</p>
<p align="center">
  <img src="https://img.shields.io/badge/v0.1.7-blue" alt="version">
  <img src="https://img.shields.io/badge/Tauri_v2-orange" alt="Tauri v2">
  <img src="https://img.shields.io/badge/Svelte_5-red" alt="Svelte 5">
  <img src="https://img.shields.io/badge/Rust-black" alt="Rust">
  <img src="https://img.shields.io/badge/Windows-grey" alt="Windows">
</p>

Fox sits on your desktop and reflects what your AI coding agent is doing: thinking when you prompt, typing when tools run, juggling subagents, popping permission bubbles, celebrating on completion, and sleeping when you step away.

Works with **Claude Code**, **Codex CLI**, and **Copilot CLI** — all three can run simultaneously.

> Forked from [Clyde on Desk](https://github.com/QingJ01/Clyde). See [What's Changed](#whats-changed) for a list of improvements.

## Quick Start

```bash
git clone https://github.com/HOLY0305/fox_on_desk.git
cd fox_on_desk
npm install
npm start        # Tauri dev mode with hot-reload
```

**Prerequisites** — [Node.js](https://nodejs.org/) v18+, [Rust](https://rustup.rs/) stable, and [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) for Windows.

**Agent setup** — all zero-config:
- **Claude Code** — hooks auto-registered on launch
- **Codex CLI** — log polling starts automatically (`~/.codex/sessions/`)
- **Copilot CLI** — auto-configured when `~/.copilot` exists

## What's Changed

Compared to the original [Clyde on Desk](https://github.com/QingJ01/Clyde):

| Change | Description |
|--------|-------------|
| **Fox character** | New fox icon and skin system with transparent background |
| **Multi-skin support** | Switch between Clyde and Fox skins from the context menu |
| **XL size option** | New 480px pet size (S / M / L / XL) |
| **Sound effects** | Pixel-style audio feedback for key events |
| **Alt+C shortcut** | Summon pet to cursor position |
| **Double-click launch** | Double-click pet to open a folder picker for a new Claude session |
| **AskUserQuestion** | Permission bubbles handle Claude's structured input prompts |
| **AlwaysOnTop fix** | Pet stays on top even after other windows steal focus |

## Features

### Animations

12 animated states driven by real-time agent events — idle eye-tracking, thinking, typing, building, juggling, conducting, error flash, happy bounce, notification, sweeping, carrying, and sleeping.

### Interaction

- **Drag** anywhere, anytime — Pointer Capture prevents fast-flick drops
- **Double-click** for a poke reaction; **4 clicks** for a flail
- **Right-click** context menu — session list, DND, mini mode, size, language, skin selection
- **System tray** — resize (S/M/L/XL), DND, mini mode, language, auto-start, quit

### Mini Mode

Drag Fox to the left or right screen edge (or right-click "Mini Mode"). Fox hides behind the edge, peeks out on hover, and shows mini alerts/celebrations while tucked away.

### Permission Bubbles

When Claude Code requests tool permissions, Fox pops a floating card near the pet — allow, deny, or apply a suggestion rule. Supports both simple tool permissions and **AskUserQuestion** structured input prompts (enum choices, booleans, text fields).

### Session Intelligence

- **Multi-session priority** — the highest-priority state across all sessions wins
- **Subagent-aware** — 1 subagent = juggling, 2+ = conducting
- **Terminal focus** — right-click a session to jump to its terminal
- **Auto-cleanup** — stale sessions removed after 10 min
- **DND mode** — silences all events; toggle via right-click or tray

## Architecture

```
src-tauri/src/           Rust backend
├── lib.rs               App entry + Tauri commands
├── state_machine.rs     Multi-session state tracking + priority
├── http_server.rs       Axum HTTP (POST /state, /permission)
├── hooks.rs             Hook deployment + settings.json registration
├── permission.rs        Permission bubble windows
├── sfx.rs               Sound effects system
├── skins.rs             Multi-skin loading + hot-swap
├── mini.rs              Edge snap, peek, parabolic jump
├── tick.rs              50ms cursor poll (eyes, sleep, peek)
├── tray.rs              System tray menu
├── codex_monitor.rs     Codex JSONL log polling
├── prefs.rs             Preferences persistence
└── i18n.rs              English / Chinese strings

src/windows/             Svelte 5 frontend (3 windows)
├── pet/                 SVG renderer + skin system
├── hit/                 Invisible click layer
└── bubble/              Permission card

hooks/                   JS hooks (embedded at compile time)
assets/skins/            Fox + Clyde SVG skin sets
```

## Tech Stack

| Layer | Technology | Why |
|---|---|---|
| **Desktop** | [Tauri v2](https://v2.tauri.app/) | ~5 MB bundle; native OS APIs; Rust backend |
| **Backend** | [Rust](https://www.rust-lang.org/) | No GC; multi-session state machine; near-zero CPU |
| **Frontend** | [Svelte 5](https://svelte.dev/) | Compile-time; three windows < 30 KB JS total |
| **HTTP** | [Axum](https://github.com/tokio-rs/axum) | Async on shared Tokio runtime |
| **Build** | [Vite](https://vitejs.dev/) | Instant HMR in dev; tree-shaking in production |

## Known Limitations

| Limitation | Details |
|---|---|
| Windows only | Tested on Windows; macOS/Linux support not verified |
| Codex: no terminal focus | JSONL polling doesn't carry terminal PID |
| Copilot: no permission bubble | Copilot's hook protocol only supports deny |
| No auto-update | Download new versions from GitHub Releases |

## Troubleshooting

### Permission bubbles not appearing

1. In Claude Code, run `/hooks` and check that `PermissionRequest` has an `[http]` hook
2. If missing, restart Fox — it re-registers hooks on startup
3. If still broken, run `node hooks/install.js` manually
4. Last resort: delete the `PermissionRequest` entry from `~/.claude/settings.json` and restart

## Contributing

Issues, ideas, and PRs welcome — [open an issue](https://github.com/HOLY0305/fox_on_desk/issues) or submit a PR.

## Acknowledgments

- Forked from [Clyde on Desk](https://github.com/QingJ01/Clyde) by [@QingJ01](https://github.com/QingJ01)
- Originally from [Clawd on Desk](https://github.com/rullerzhou-afk/clawd-on-desk) by [@rullerzhou-afk](https://github.com/rullerzhou-afk)
- Thanks to the [LINUX DO](https://linux.do/) community for feedback and support

## License

[AGPL-3.0](LICENSE)
