# SpinnerOS

<p align="center">
  <img src="assets/icons/spinneros-logo.svg" alt="SpinnerOS Logo" width="200">
</p>

<p align="center">
  <strong>A modern, minimal Linux distribution with a glass neomorphism desktop</strong>
</p>

<p align="center">
  <a href="#features">Features</a> •
  <a href="#screenshots">Screenshots</a> •
  <a href="#installation">Installation</a> •
  <a href="#building">Building</a> •
  <a href="#contributing">Contributing</a>
</p>

---

## About

SpinnerOS is a custom Linux distribution based on Debian, featuring a unique desktop environment written in Rust with GTK4. The system combines the stability of Debian with a modern, visually stunning interface inspired by glass neomorphism design.

## Features

### Desktop Environment
- **SpinnerWM** - Custom Wayland compositor with floating window management
- **SpinnerShell** - Modern panel with taskbar, system tray, and app launcher
- **Glass Neomorphism** - Beautiful translucent design with soft shadows
- **Native Wayland** - Full Wayland support with XWayland for compatibility

### System
- **Debian Base** - Stability and vast software repository
- **Dual Package Support** - APT (.deb) and Flatpak out of the box
- **Modern Stack** - PipeWire audio, systemd, and latest kernel

### Applications
- **SpinnerSettings** - Comprehensive system settings
- **SpinnerStore** - Software center for APT and Flatpak
- **Pre-configured Apps** - Firefox, Nautilus, Terminal, and more

## System Requirements

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| CPU | x86_64 | x86_64, 2+ cores |
| RAM | 2 GB | 4 GB |
| Storage | 10 GB | 20 GB |
| Graphics | OpenGL 3.3 | Vulkan support |

## Installation

### Quick Start

1. Download the latest ISO from [Releases](https://github.com/spinneros/spinneros/releases)
2. Create bootable USB with Rufus, dd, or Etcher
3. Boot from USB and enjoy!

### Detailed Guide

See [docs/INSTALL.md](docs/INSTALL.md) for complete installation instructions.

## Building from Source

### Prerequisites

```bash
# Debian/Ubuntu
sudo apt install build-essential debootstrap live-build \
    libgtk-4-dev libadwaita-1-dev libwayland-dev \
    libinput-dev pkg-config curl git

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build Commands

```bash
# Clone repository
git clone https://github.com/spinneros/spinneros.git
cd spinneros

# Build desktop components
./build/build-desktop.sh build

# Build complete ISO (requires root)
sudo ./build/build-iso.sh build
```

## Project Structure

```
SpinnerOS/
├── spinner-wm/         # Wayland compositor
├── spinner-shell/      # Desktop shell (panel, launcher, notifications)
├── spinner-settings/   # System settings application
├── spinner-store/      # Software center
├── build/              # Build scripts and ISO configuration
├── config/             # Default system configuration
├── assets/             # Icons, wallpapers, themes
└── docs/               # Documentation
```

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Super + Return` | Terminal |
| `Super + D` | App launcher |
| `Super + Q` | Close window |
| `Super + F` | Fullscreen |
| `Super + 1-5` | Switch workspace |
| `Super + Shift + E` | Exit |

Full list in [docs/INSTALL.md](docs/INSTALL.md#keyboard-shortcuts).

## Technology Stack

| Component | Technology |
|-----------|------------|
| Base | Debian Bookworm |
| Kernel | Linux 6.x |
| Init | systemd |
| Display | Wayland (wlroots/smithay) |
| Toolkit | GTK4 + libadwaita |
| Language | Rust |
| Audio | PipeWire |
| Packages | APT + Flatpak |

## Contributing

We welcome contributions! Here's how you can help:

1. **Report bugs** - Open an issue with details
2. **Suggest features** - We'd love to hear your ideas
3. **Submit PRs** - Code contributions are appreciated
4. **Documentation** - Help improve our docs
5. **Testing** - Try SpinnerOS and provide feedback

### Development Setup

```bash
# Clone and build
git clone https://github.com/spinneros/spinneros.git
cd spinneros
cargo build --workspace

# Run tests
cargo test --workspace

# Format code
cargo fmt --all
```

## Roadmap

- [ ] Graphical installer
- [ ] Hardware detection improvements
- [ ] More themes and customization
- [ ] Tiling window mode
- [ ] Multi-monitor improvements
- [ ] Performance optimizations

## License

SpinnerOS is free software licensed under the [GNU General Public License v3.0](LICENSE).

## Acknowledgments

- [Debian Project](https://debian.org) - Base system
- [GNOME](https://gnome.org) - GTK4 and design inspiration
- [Smithay](https://github.com/Smithay/smithay) - Wayland compositor library
- [Rust Community](https://rust-lang.org) - Amazing language and ecosystem

---

<p align="center">
  Made with ❤️ by the SpinnerOS Team
</p>
