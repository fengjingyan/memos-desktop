# Memos Desktop

> A native desktop app for [Memos](https://github.com/usememos/memos) — your self-hosted notes, one click away.

Memos Desktop wraps the official [Memos](https://usememos.com) server in a lightweight [Tauri](https://tauri.app) window. It bundles the `memos` binary as a sidecar, starts it automatically on launch, waits until the server is ready, and then opens the Memos web UI — no Docker, no manual `memos start`, no browser tab hunting.

**English** | [Chinese](docs/README.zh-CN.md)

## Why Memos Desktop?

- **Zero setup** — the Memos server binary is bundled and managed for you; just run the app.
- **Local-first** — data lives in a SQLite database next to the executable by default, fully under your control.
- **Configurable** — port, data directory, window size, and extra server flags are all driven by a single `config.yaml`.
- **Lightweight** — a Tauri shell (~4.5 MB) instead of an Electron runtime.

## Quick Start

### Download a release

Grab the latest build from the [Releases](../../releases) page:

| Artifact | Description |
| --- | --- |
| `memos-desktop_x.y.z_x64-setup.exe` | NSIS installer (recommended) |
| `memos-desktop_x.y.z_x64_en-US.msi` | MSI installer for enterprise deployment |
| `memos-desktop_x.y.z-win-x64.7z` / `.zip` | Portable version — extract and run |

Run `memos-desktop.exe` and the Memos UI opens automatically. On first launch the server initializes its database, which may take a few seconds.

### Run from source

Prerequisites: [Rust](https://rustup.rs), [Node.js](https://nodejs.org), and a `memos` binary.

1. Place the Memos server binary at `binaries/memos-x86_64-pc-windows-msvc.exe`
   (the sidecar name must include the target triple; on other platforms use the matching triple, e.g. `memos-x86_64-apple-darwin`).
2. Install dependencies and start in dev mode:

   ```bash
   npm install
   npm run tauri dev
   ```

3. Build release bundles (NSIS + MSI):

   ```bash
   npm run tauri build
   ```

   Artifacts land in `src-tauri/target/release/bundle/`.

## Configuration

The app reads `config.yaml` from the first location that exists:

1. Next to `memos-desktop.exe` (e.g. the install directory or the portable folder)
2. `%APPDATA%\com.admin.memos-desktop\config.yaml` (Windows)

If no file is found, built-in defaults are used. Copy `src-tauri/config.yaml.example` to get started:

```yaml
# Address the Memos server listens on (the window loads http://{host}:{port})
host: 127.0.0.1

# Port the Memos server listens on
port: 5230

# Data directory (SQLite database lives here).
# Leave empty or omit to store data next to the executable.
# data: c:/users/user/memos

# Window size (logical pixels)
width: 1920
height: 1080

# Extra flags passed through to the memos server (optional)
extra_args:
  # - "--verbose"
```

Restart the app after editing the file.

## How It Works

```
memos-desktop.exe (Tauri shell)
├── reads config.yaml
├── spawns memos.exe sidecar  →  --port <port> --data <data> [extra_args...]
├── polls the port until the server is ready (up to 15 s)
└── opens a webview window at http://<host>:<port>
```

## Project Layout

```
├── binaries/                  # memos sidecar binary (with target triple suffix)
├── dist/                      # portable builds (7z / zip)
├── docs/                      # documentation (incl. Chinese README)
├── icons/
└── src-tauri/
    ├── config.yaml.example    # configuration template
    ├── src/lib.rs             # config loading, sidecar spawn, window creation
    └── tauri.conf.json
```

## License

Memos Desktop is provided as-is for personal use. The bundled [Memos](https://github.com/usememos/memos) server is licensed under [MIT](https://github.com/usememos/memos/blob/main/LICENSE).
