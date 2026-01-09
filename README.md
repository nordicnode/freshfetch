<h2 align="center">Freshfetch</h2>
<p align="center">
<i>A fresh take on Neofetch</i>
<br>
<br>
<a href="./LICENSE.md"><img src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
<a href="https://github.com/nordicnode/freshfetch/releases"><img src="https://img.shields.io/github/v/release/nordicnode/freshfetch"></a>
</p>

Freshfetch is a fast, customizable system information tool written in Rust.

**This is a personal continuation of the original freshfetch project by [K4rakara](https://github.com/K4rakara/freshfetch), maintained by [nordicnode](https://github.com/nordicnode).**
It has been extensively modernized from the original codebase with new features, improved performance, and robust error handling.

## Features

### System Information
- **User & Host** - Username and hostname
- **OS** - Distribution name and architecture
- **Host** - System model/product name
- **Kernel** - Linux kernel version
- **Uptime** - System uptime in days/hours/minutes
- **Packages** - Package counts from dpkg, rpm, pacman, flatpak, snap, etc.
- **Shell** - Current shell and version (Bash, Zsh, Fish, Nushell, etc.)
- **Resolution** - Display resolution and refresh rate
- **DE** - Desktop environment and version (Cinnamon, GNOME, KDE, etc.)
- **WM** - Window manager (Mutter, KWin, i3, etc.)
- **CPU** - Processor name, cores, and frequency
- **GPU** - Graphics card(s) with brand detection
- **Board** - Motherboard vendor and model
- **Memory** - RAM usage
- **Battery** - Capacity and status (laptops only)
- **Disk** - Root partition usage
- **Network** - Active interface and local IP

### Output Options
- **Standard** - Classic ASCII art + Info display
- **JSON** - Machine-readable output via `--json`
- **Logo Only** - pure ASCII art via `--logo`

### Customization
- **Lua scripting** - Full control via `layout.lua`, `info.lua`, `art.lua`
- **Image support** - Kitty, Sixel, and iTerm2 protocols (via `viuer`)
- **Custom ASCII art** - 261+ distro logos or your own art
- **Distro colors** - Automatic color theming per distribution

### Technical
- **Parallel info gathering** - Uses `rayon` for concurrent system info collection
- **Pure Rust** - No shell-outs for distro detection
- **Portable paths** - Uses `dirs` crate, no hardcoded `/home/` paths
- **Robust error handling** - `Result`-based propagation, no panics
- **Modern dependencies** - `mlua` 0.9, `sysinfo` 0.30, `clap` 4.x

## Requirements

- Rust 2021 Edition (1.56+)
- Linux, BSD, or MINIX

## Installation

#### Arch Linux

On Arch Linux, you can install from the AUR:

```bash
yay -S freshfetch-git    # Bleeding-edge from master
yay -S freshfetch-bin    # Pre-built stable release
```

#### Other distros

Build from source:

```bash
git clone https://github.com/nordicnode/freshfetch.git
cd freshfetch
cargo build --release
sudo cp ./target/release/freshfetch /usr/bin/
```

## Usage

```bash
freshfetch                  # Display system info with ASCII art
freshfetch --logo           # Display only ASCII art
freshfetch -a ubuntu        # Use Ubuntu's ASCII art
freshfetch --json           # Output as machine-readable JSON
```

## Configuration

Create custom layouts in `~/.config/freshfetch/`:

- `layout.lua` - Main layout combining art and info
- `info.lua` - System information display
- `art.lua` - Custom ASCII art

## Recent Changes

### v0.2.0 (2026)
- **New info modules**: Battery, Disk, Network
- **JSON Output**: Fully structured JSON output support
- **Shell Detection**: Improved version detection for Bash, Fish, Nushell
- **Parallel gathering**: Concurrent info collection with rayon
- **mlua upgrade**: 0.6.6 → 0.9.9 for Rust compatibility
- **Code quality**: Resolved 99 clippy warnings → 1 remaining (type complexity)
- **Error handling**: Complete refactor to `Result`-based propagation
- **Dependencies**: Updated clap 4.x, sysinfo 0.30, chrono 0.4.31
- **Removed**: `cmd_lib` dependency (pure Rust distro detection)
- **Portable paths**: Uses `dirs` crate instead of hardcoded paths

## Todo

- [ ] Add colorization for all distros (72/261 complete)
- [ ] Unit and integration tests
- [ ] macOS support

<p align="center">
<img alt="An example configuration" src="./readme/config-1.png"/>
<img alt="An example configuration" src="./readme/config-2.png"/>
<img alt="An example configuration" src="./readme/config-3.png"/>
</p>
