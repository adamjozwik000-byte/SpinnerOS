# SpinnerOS Installation Guide

## Overview

SpinnerOS is a custom Linux distribution based on Debian with a unique desktop environment written in Rust. This guide covers building the system from source and creating a bootable ISO.

## System Requirements

### Build Machine Requirements
- Debian/Ubuntu-based system (for building)
- At least 20GB free disk space
- 4GB+ RAM recommended
- Internet connection

### Target Machine Requirements
- x86_64 processor (AMD64)
- 2GB RAM minimum (4GB recommended)
- 10GB disk space minimum
- UEFI or BIOS boot support

## Building SpinnerOS

### Prerequisites

Install required packages on your build machine:

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install build dependencies
sudo apt install -y \
    build-essential \
    debootstrap \
    live-build \
    squashfs-tools \
    xorriso \
    grub-pc-bin \
    grub-efi-amd64-bin \
    mtools \
    dosfstools \
    git \
    curl

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# Install GTK4 development libraries
sudo apt install -y \
    libgtk-4-dev \
    libadwaita-1-dev \
    libwayland-dev \
    libinput-dev \
    libdrm-dev \
    libgbm-dev \
    libudev-dev \
    libseat-dev \
    libxkbcommon-dev \
    pkg-config
```

### Clone the Repository

```bash
git clone https://github.com/spinneros/spinneros.git
cd spinneros
```

### Build Desktop Components Only

If you want to test the desktop environment on an existing Linux system:

```bash
# Build all Rust components
./build/build-desktop.sh build

# Install to ~/.local
./build/build-desktop.sh install

# Or install system-wide (requires root)
sudo ./build/build-desktop.sh install-system
```

### Build Complete ISO

To create a bootable SpinnerOS ISO:

```bash
# Build the ISO (requires root)
sudo ./build/build-iso.sh build
```

This process will:
1. Set up a Debian Live Build environment
2. Install all required packages
3. Compile SpinnerOS components from source
4. Configure the system
5. Generate a bootable ISO

The resulting ISO will be located at `SpinnerOS/spinneros-0.1.0-amd64.iso`.

## Installation

### Creating Bootable Media

#### Using dd (Linux/macOS):
```bash
sudo dd if=spinneros-0.1.0-amd64.iso of=/dev/sdX bs=4M status=progress
sync
```

#### Using Rufus (Windows):
1. Download and run Rufus
2. Select the SpinnerOS ISO
3. Select your USB drive
4. Choose "DD Image" mode
5. Click Start

### Booting SpinnerOS

1. Insert the USB drive
2. Boot from USB (usually F12 or F2 during startup)
3. Select "SpinnerOS Live"

### Installing to Disk (Future Feature)

A graphical installer is planned for future releases. Currently, SpinnerOS runs as a live system.

## Configuration

### Configuration Files

SpinnerOS configuration files are located in:

- `/etc/spinneros/` - System-wide configuration
- `~/.config/spinneros/` - User configuration

Key configuration files:

| File | Purpose |
|------|---------|
| `spinner-wm.toml` | Window manager settings, keybindings |
| `spinner-shell.toml` | Desktop shell settings, panel config |

### Keyboard Shortcuts

Default keybindings (can be customized in `spinner-wm.toml`):

| Shortcut | Action |
|----------|--------|
| `Super + Return` | Open terminal |
| `Super + D` | Open application menu |
| `Super + Q` | Close window |
| `Super + F` | Toggle fullscreen |
| `Super + Space` | Toggle floating mode |
| `Super + 1-5` | Switch workspace |
| `Super + Shift + 1-5` | Move window to workspace |
| `Super + Shift + E` | Exit SpinnerWM |

### Package Management

SpinnerOS supports both APT and Flatpak:

```bash
# APT (Debian packages)
sudo apt install package-name

# Flatpak
flatpak install flathub app.name
```

Or use the graphical **Software** application.

## Development

### Project Structure

```
SpinnerOS/
├── build/              # Build scripts and configuration
├── spinner-wm/         # Wayland compositor (Rust)
├── spinner-shell/      # Desktop environment (Rust + GTK4)
├── spinner-settings/   # System settings app
├── spinner-store/      # Software store app
├── assets/             # Icons, wallpapers, sounds
├── config/             # Default configuration files
└── docs/               # Documentation
```

### Building Individual Components

```bash
# Build only the window manager
cd spinner-wm
cargo build --release

# Build only the shell
cd spinner-shell
cargo build --release
```

### Running in Development Mode

On an existing Wayland session:

```bash
# Build and run the shell for testing
./build/build-desktop.sh build
./build/build-desktop.sh run
```

## Troubleshooting

### Common Issues

**Build fails with missing dependencies:**
```bash
# Install all development packages
sudo apt install -y libgtk-4-dev libadwaita-1-dev libwayland-dev
```

**Rust compilation errors:**
```bash
# Update Rust toolchain
rustup update stable
```

**Display issues:**
- Ensure your system supports Wayland
- Check for proprietary GPU driver requirements

### Logs

View SpinnerWM logs:
```bash
journalctl -u spinner-wm.service -f
```

View SpinnerShell logs:
```bash
journalctl --user -u spinner-shell.service -f
```

## Contributing

Contributions are welcome! Please see our contributing guidelines on GitHub.

## License

SpinnerOS is licensed under the GNU General Public License v3.0.

## Contact

- Website: https://spinneros.org
- GitHub: https://github.com/spinneros/spinneros
- Issues: https://github.com/spinneros/spinneros/issues
