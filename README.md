<h2 align="center">Freshfetch</h2>
<p align="center">
<i>A fresh take on Neofetch</i>
<br>
<br>
<a href="./LICENSE.md"><img src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
<a href="https://github.com/k4rakara/freshfetch/releases"><img src="https://img.shields.io/github/v/release/K4rakara/freshfetch"></a>
</p>

Freshfetch is an alternative to [Neofetch](https://github.com/dylanaraps/neofetch)
written in Rust with a focus on customization.

## Features

- **Fully customizable** via Lua scripting (`layout.lua`, `info.lua`, `art.lua`)
- **Image support** via Kitty, Sixel, and iTerm2 protocols (using `viuer`)
- **Fast** - optimized system info gathering with selective refreshes
- **Portable** - no hardcoded paths, uses standard config directories

## Requirements

- Rust 2021 Edition (1.56+)
- Linux, BSD, or MINIX

## Installation

#### Arch Linux

On Arch Linux, you can install one of three AUR packages:

- `freshfetch-git` -- The bleeding-edge version of freshfetch that builds from the master branch.
- `freshfetch-bin` -- The stable version of freshfetch that you just install. No compile required.
- `freshfetch` -- Currently not set up right, will be fixed with the next release. Once set up, It'll build the latest stable version from source.

#### Other distros

With other distributions, you can either install the [latest `tar.gz` build](https://github.com/K4rakara/freshfetch/releases) or build from source.

###### Build from source

To compile Freshfetch, just run `cargo build --release -vv`. This will build the executable for your platform. Then, run these commands:
```bash
sudo cp ./target/release/freshfetch /usr/bin/
sudo chmod 755 /usr/bin/freshfetch
```

## Usage

```bash
freshfetch                  # Display system info with ASCII art
freshfetch --logo           # Display only ASCII art
freshfetch -a ubuntu        # Use Ubuntu's ASCII art
```

## Todo

- [x] Optimizations (selective sysinfo refresh, pure Rust distro detection)
- [x] Documentation (improved codebase comments)
- [x] Image support (via `viuer` integration)
- [ ] Add colorization for all distros (72/261 complete)

<p align="center">
<img alt="An example configuration" src="./readme/config-1.png"/>
<img alt="An example configuration" src="./readme/config-2.png"/>
<img alt="An example configuration" src="./readme/config-3.png"/>
</p>
