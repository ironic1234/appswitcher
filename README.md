# mofi

A spotlight replacement.

## Overview

**mofi** is an open-source spotlight replacement built with [Svelte](https://svelte.dev/) and [Tauri v2](https://tauri.app/), designed to provide fast, intuitive, and customizable search capabilities for your desktop environment.

## Features

- Fast and lightweight search
- User-friendly, responsive interface (Svelte)
- Easily extensible for additional features

### Prerequisites

- [Node.js](https://nodejs.org/) (v14+ recommended)
- [npm](https://www.npmjs.com/) or [yarn](https://yarnpkg.com/)
- [Rust](https://www.rust-lang.org/tools/install) (required by Tauri)
- [Tauri CLI v2](https://tauri.app/v2/guides/getting-started/prerequisites/)

### Installation

```bash
git clone https://github.com/ironic1234/mofi.git
cd mofi
npm install
```

### Development

To start the app in development mode:

```bash
npm run tauri dev
```

### Building for Production

```bash
npm run tauri build
```

### Configuration

Colors defined in .config/mofi/colors.toml, default:

```toml
background = "#1e1e2e"
border = "#fab387"
text = "#fab387"
selected_bg = "#fab387"
selected_text = "#1e1e2e"
```

### Coming (Hopefully)
- Support for other system apps (Mostly finder)
- Other features (translations, calculator, maybe files)
- Customization (other than color, like UI style)
