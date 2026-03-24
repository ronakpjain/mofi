# mofi

A spotlight replacement.

## Overview

**mofi** is an open-source spotlight replacement built with [Svelte](https://svelte.dev/) and [Tauri v2](https://tauri.app/), designed to provide fast, intuitive, and customizable search capabilities for your desktop environment.

## Features

- Fast and lightweight search
- User-friendly, responsive interface (Svelte)
- Easily extensible for additional features

### Prerequisites

- [Bun](https://bun.sh/) (recommended package manager/runtime)
- [Rust](https://www.rust-lang.org/tools/install) (required by Tauri)
- [Tauri CLI v2](https://tauri.app/v2/guides/getting-started/prerequisites/)

> This app is macOS-only.

### Installation

```bash
git clone https://github.com/ronakpjain/mofi.git
cd mofi
bun install
```

### Development

To start the app in development mode:

```bash
bun run tauri dev
```

Frontend-only dev server:

```bash
bun run dev
```

### Building for Production

```bash
bun run tauri build
```

### Configuration

Primary config in .config/mofi/mofi.toml (supports colors + aliases). Colors still fall back to .config/mofi/colors.toml if not provided.

```toml
[colors]
background = "#1e1e2e"
border = "#fab387"
text = "#fab387"
selected_bg = "#fab387"
selected_text = "#1e1e2e"

[aliases]
Spotify = "spicetify auto"
```

### Coming (Hopefully)
- Other features (translations, calculator, maybe files)
- Customization (other than color, like UI style)
