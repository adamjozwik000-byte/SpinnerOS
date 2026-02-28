#!/bin/bash
#
# SpinnerOS ISO Builder
# Based on Debian Live Build system
#
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BUILD_DIR="$PROJECT_ROOT/build-output"
ISO_NAME="spinneros-0.1.0-amd64"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; exit 1; }

check_root() {
    if [[ $EUID -ne 0 ]]; then
        log_error "This script must be run as root (use sudo)"
    fi
}

check_dependencies() {
    log_info "Checking build dependencies..."
    
    local deps=(
        "debootstrap"
        "live-build"
        "squashfs-tools"
        "xorriso"
        "grub-pc-bin"
        "grub-efi-amd64-bin"
        "mtools"
        "dosfstools"
    )
    
    local missing=()
    for dep in "${deps[@]}"; do
        if ! dpkg -l | grep -q "^ii  $dep"; then
            missing+=("$dep")
        fi
    done
    
    if [[ ${#missing[@]} -gt 0 ]]; then
        log_warn "Installing missing dependencies: ${missing[*]}"
        apt-get update
        apt-get install -y "${missing[@]}"
    fi
    
    log_success "All dependencies satisfied"
}

setup_live_build() {
    log_info "Setting up Debian Live Build environment..."
    
    rm -rf "$BUILD_DIR"
    mkdir -p "$BUILD_DIR"
    cd "$BUILD_DIR"
    
    lb config \
        --distribution bookworm \
        --architecture amd64 \
        --binary-images iso-hybrid \
        --bootloaders "grub-efi,grub-pc" \
        --debian-installer none \
        --apt-recommends false \
        --memtest none \
        --iso-application "SpinnerOS" \
        --iso-publisher "SpinnerOS Project" \
        --iso-volume "SpinnerOS 0.1.0"
    
    log_success "Live Build configured"
}

configure_package_lists() {
    log_info "Configuring package lists..."
    
    mkdir -p "$BUILD_DIR/config/package-lists"
    
    cat > "$BUILD_DIR/config/package-lists/base.list.chroot" << 'EOF'
linux-image-amd64
firmware-linux
systemd
dbus
polkitd
network-manager
pipewire
pipewire-pulse
wireplumber
xwayland
EOF

    cat > "$BUILD_DIR/config/package-lists/desktop.list.chroot" << 'EOF'
fonts-noto
fonts-noto-color-emoji
adwaita-icon-theme
hicolor-icon-theme
gsettings-desktop-schemas
EOF

    cat > "$BUILD_DIR/config/package-lists/build-deps.list.chroot" << 'EOF'
build-essential
pkg-config
libgtk-4-dev
libadwaita-1-dev
libwayland-dev
libinput-dev
libdrm-dev
libgbm-dev
libudev-dev
libseat-dev
libxkbcommon-dev
EOF

    cat > "$BUILD_DIR/config/package-lists/apps.list.chroot" << 'EOF'
flatpak
gnome-software-plugin-flatpak
firefox-esr
nautilus
gnome-terminal
EOF

    log_success "Package lists created"
}

setup_flatpak() {
    log_info "Configuring Flatpak..."
    
    mkdir -p "$BUILD_DIR/config/hooks/live"
    
    cat > "$BUILD_DIR/config/hooks/live/0100-flatpak.hook.chroot" << 'EOF'
#!/bin/bash
flatpak remote-add --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo
EOF
    
    chmod +x "$BUILD_DIR/config/hooks/live/0100-flatpak.hook.chroot"
    log_success "Flatpak configured"
}

install_rust_toolchain() {
    log_info "Setting up Rust toolchain installation..."
    
    cat > "$BUILD_DIR/config/hooks/live/0200-rust.hook.chroot" << 'EOF'
#!/bin/bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
source "$HOME/.cargo/env"
EOF
    
    chmod +x "$BUILD_DIR/config/hooks/live/0200-rust.hook.chroot"
    log_success "Rust toolchain setup configured"
}

copy_spinneros_components() {
    log_info "Copying SpinnerOS components..."
    
    mkdir -p "$BUILD_DIR/config/includes.chroot/opt/spinneros"
    
    cp -r "$PROJECT_ROOT/spinner-wm" "$BUILD_DIR/config/includes.chroot/opt/spinneros/"
    cp -r "$PROJECT_ROOT/spinner-shell" "$BUILD_DIR/config/includes.chroot/opt/spinneros/"
    cp -r "$PROJECT_ROOT/spinner-settings" "$BUILD_DIR/config/includes.chroot/opt/spinneros/"
    cp -r "$PROJECT_ROOT/spinner-store" "$BUILD_DIR/config/includes.chroot/opt/spinneros/"
    cp "$PROJECT_ROOT/Cargo.toml" "$BUILD_DIR/config/includes.chroot/opt/spinneros/"
    
    cp -r "$PROJECT_ROOT/assets" "$BUILD_DIR/config/includes.chroot/opt/spinneros/"
    cp -r "$PROJECT_ROOT/config" "$BUILD_DIR/config/includes.chroot/opt/spinneros/"
    
    log_success "SpinnerOS components copied"
}

setup_build_hook() {
    log_info "Setting up SpinnerOS build hook..."
    
    cat > "$BUILD_DIR/config/hooks/live/0300-build-spinneros.hook.chroot" << 'EOF'
#!/bin/bash
set -e

source "$HOME/.cargo/env" 2>/dev/null || true

cd /opt/spinneros

cargo build --release

mkdir -p /usr/local/bin
cp target/release/spinner-wm /usr/local/bin/
cp target/release/spinner-shell /usr/local/bin/
cp target/release/spinner-settings /usr/local/bin/
cp target/release/spinner-store /usr/local/bin/

chmod +x /usr/local/bin/spinner-*

mkdir -p /usr/share/spinneros
cp -r assets/* /usr/share/spinneros/
cp -r config/* /etc/spinneros/

mkdir -p /etc/spinneros
cp config/*.toml /etc/spinneros/ 2>/dev/null || true

rm -rf /opt/spinneros/target
EOF
    
    chmod +x "$BUILD_DIR/config/hooks/live/0300-build-spinneros.hook.chroot"
    log_success "Build hook created"
}

setup_systemd_services() {
    log_info "Setting up systemd services..."
    
    mkdir -p "$BUILD_DIR/config/includes.chroot/etc/systemd/system"
    
    cat > "$BUILD_DIR/config/includes.chroot/etc/systemd/system/spinner-wm.service" << 'EOF'
[Unit]
Description=SpinnerOS Window Manager
After=systemd-user-sessions.service
Wants=dbus.socket

[Service]
Type=simple
ExecStart=/usr/local/bin/spinner-wm
Restart=on-failure
RestartSec=5
Environment=XDG_SESSION_TYPE=wayland
Environment=XDG_CURRENT_DESKTOP=SpinnerOS

[Install]
WantedBy=graphical.target
EOF

    mkdir -p "$BUILD_DIR/config/includes.chroot/etc/systemd/user"
    
    cat > "$BUILD_DIR/config/includes.chroot/etc/systemd/user/spinner-shell.service" << 'EOF'
[Unit]
Description=SpinnerOS Desktop Shell
After=spinner-wm.service
PartOf=graphical-session.target

[Service]
Type=simple
ExecStart=/usr/local/bin/spinner-shell
Restart=on-failure
RestartSec=3

[Install]
WantedBy=graphical-session.target
EOF

    cat > "$BUILD_DIR/config/hooks/live/0400-enable-services.hook.chroot" << 'EOF'
#!/bin/bash
systemctl set-default graphical.target
systemctl enable spinner-wm.service
EOF
    
    chmod +x "$BUILD_DIR/config/hooks/live/0400-enable-services.hook.chroot"
    log_success "Systemd services configured"
}

setup_user_and_autologin() {
    log_info "Setting up default user and autologin..."
    
    cat > "$BUILD_DIR/config/hooks/live/0500-user-setup.hook.chroot" << 'EOF'
#!/bin/bash
useradd -m -G sudo,audio,video,plugdev -s /bin/bash spinner || true
echo "spinner:spinner" | chpasswd
echo "spinner ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/spinner

mkdir -p /etc/systemd/system/getty@tty1.service.d
cat > /etc/systemd/system/getty@tty1.service.d/autologin.conf << 'AUTOLOGIN'
[Service]
ExecStart=
ExecStart=-/sbin/agetty --autologin spinner --noclear %I $TERM
AUTOLOGIN
EOF
    
    chmod +x "$BUILD_DIR/config/hooks/live/0500-user-setup.hook.chroot"
    log_success "User setup configured"
}

setup_branding() {
    log_info "Setting up SpinnerOS branding..."
    
    mkdir -p "$BUILD_DIR/config/includes.chroot/etc"
    
    cat > "$BUILD_DIR/config/includes.chroot/etc/os-release" << 'EOF'
PRETTY_NAME="SpinnerOS 0.1.0"
NAME="SpinnerOS"
VERSION_ID="0.1.0"
VERSION="0.1.0 (Prototype)"
VERSION_CODENAME=prototype
ID=spinneros
ID_LIKE=debian
HOME_URL="https://spinneros.org"
SUPPORT_URL="https://github.com/spinneros/spinneros/issues"
BUG_REPORT_URL="https://github.com/spinneros/spinneros/issues"
EOF

    cat > "$BUILD_DIR/config/includes.chroot/etc/issue" << 'EOF'
SpinnerOS 0.1.0 \n \l

EOF

    log_success "Branding configured"
}

build_iso() {
    log_info "Building ISO image (this may take a while)..."
    
    cd "$BUILD_DIR"
    lb build 2>&1 | tee build.log
    
    if [[ -f "live-image-amd64.hybrid.iso" ]]; then
        mv "live-image-amd64.hybrid.iso" "$PROJECT_ROOT/$ISO_NAME.iso"
        log_success "ISO built successfully: $PROJECT_ROOT/$ISO_NAME.iso"
    else
        log_error "ISO build failed. Check $BUILD_DIR/build.log"
    fi
}

clean() {
    log_info "Cleaning build directory..."
    cd "$BUILD_DIR" 2>/dev/null && lb clean --purge || true
    rm -rf "$BUILD_DIR"
    log_success "Build directory cleaned"
}

usage() {
    cat << EOF
SpinnerOS ISO Builder

Usage: $0 [COMMAND]

Commands:
    build       Build the complete ISO (default)
    clean       Clean build directory
    help        Show this help message

Examples:
    sudo $0 build    # Build SpinnerOS ISO
    sudo $0 clean    # Clean up build files
EOF
}

main() {
    local cmd="${1:-build}"
    
    case "$cmd" in
        build)
            check_root
            check_dependencies
            setup_live_build
            configure_package_lists
            setup_flatpak
            install_rust_toolchain
            copy_spinneros_components
            setup_build_hook
            setup_systemd_services
            setup_user_and_autologin
            setup_branding
            build_iso
            log_success "SpinnerOS ISO build complete!"
            ;;
        clean)
            check_root
            clean
            ;;
        help|--help|-h)
            usage
            ;;
        *)
            log_error "Unknown command: $cmd"
            ;;
    esac
}

main "$@"
