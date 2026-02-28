#!/bin/bash
#
# SpinnerOS Desktop Environment Builder
# Compiles all SpinnerOS GUI components
#
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; exit 1; }

check_rust() {
    log_info "Checking Rust installation..."
    
    if ! command -v cargo &> /dev/null; then
        log_warn "Rust not found. Installing..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    fi
    
    rustc --version
    cargo --version
    log_success "Rust is installed"
}

check_dependencies() {
    log_info "Checking system dependencies..."
    
    local deps=(
        "libgtk-4-dev"
        "libadwaita-1-dev"
        "libwayland-dev"
        "libinput-dev"
        "libdrm-dev"
        "libgbm-dev"
        "libudev-dev"
        "libseat-dev"
        "libxkbcommon-dev"
        "pkg-config"
        "build-essential"
    )
    
    local missing=()
    for dep in "${deps[@]}"; do
        if ! dpkg -l | grep -q "^ii  $dep"; then
            missing+=("$dep")
        fi
    done
    
    if [[ ${#missing[@]} -gt 0 ]]; then
        log_warn "Missing dependencies: ${missing[*]}"
        log_info "Install them with: sudo apt install ${missing[*]}"
        
        read -p "Install now? (y/n) " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            sudo apt-get update
            sudo apt-get install -y "${missing[@]}"
        else
            log_error "Cannot continue without dependencies"
        fi
    fi
    
    log_success "All dependencies satisfied"
}

build_component() {
    local component="$1"
    local component_dir="$PROJECT_ROOT/$component"
    
    if [[ ! -d "$component_dir" ]]; then
        log_error "Component directory not found: $component_dir"
    fi
    
    log_info "Building $component..."
    cd "$component_dir"
    
    cargo build --release
    
    log_success "$component built successfully"
}

build_all() {
    log_info "Building all SpinnerOS components..."
    
    cd "$PROJECT_ROOT"
    cargo build --release --workspace
    
    log_success "All components built"
}

install_local() {
    log_info "Installing SpinnerOS components locally..."
    
    local install_dir="${1:-$HOME/.local}"
    
    mkdir -p "$install_dir/bin"
    mkdir -p "$install_dir/share/spinneros"
    mkdir -p "$HOME/.config/spinneros"
    
    cp "$PROJECT_ROOT/target/release/spinner-wm" "$install_dir/bin/" 2>/dev/null || log_warn "spinner-wm not found"
    cp "$PROJECT_ROOT/target/release/spinner-shell" "$install_dir/bin/" 2>/dev/null || log_warn "spinner-shell not found"
    cp "$PROJECT_ROOT/target/release/spinner-settings" "$install_dir/bin/" 2>/dev/null || log_warn "spinner-settings not found"
    cp "$PROJECT_ROOT/target/release/spinner-store" "$install_dir/bin/" 2>/dev/null || log_warn "spinner-store not found"
    
    cp -r "$PROJECT_ROOT/assets/"* "$install_dir/share/spinneros/" 2>/dev/null || true
    
    cp "$PROJECT_ROOT/config/"*.toml "$HOME/.config/spinneros/" 2>/dev/null || true
    
    cp -r "$PROJECT_ROOT/spinner-shell/src/theme/"*.css "$install_dir/share/spinneros/" 2>/dev/null || true
    
    log_success "Components installed to $install_dir"
}

install_system() {
    log_info "Installing SpinnerOS components system-wide..."
    
    if [[ $EUID -ne 0 ]]; then
        log_error "System installation requires root privileges"
    fi
    
    mkdir -p /usr/local/bin
    mkdir -p /usr/share/spinneros
    mkdir -p /etc/spinneros
    
    cp "$PROJECT_ROOT/target/release/spinner-wm" /usr/local/bin/
    cp "$PROJECT_ROOT/target/release/spinner-shell" /usr/local/bin/
    cp "$PROJECT_ROOT/target/release/spinner-settings" /usr/local/bin/
    cp "$PROJECT_ROOT/target/release/spinner-store" /usr/local/bin/
    
    chmod +x /usr/local/bin/spinner-*
    
    cp -r "$PROJECT_ROOT/assets/"* /usr/share/spinneros/
    cp "$PROJECT_ROOT/config/"*.toml /etc/spinneros/
    cp -r "$PROJECT_ROOT/spinner-shell/src/theme/"*.css /usr/share/spinneros/
    
    cp "$PROJECT_ROOT/build/rootfs/spinner-wm.desktop" /usr/share/wayland-sessions/ 2>/dev/null || true
    
    log_success "Components installed system-wide"
}

create_desktop_entry() {
    log_info "Creating Wayland session entry..."
    
    mkdir -p "$PROJECT_ROOT/build/rootfs"
    
    cat > "$PROJECT_ROOT/build/rootfs/spinner-wm.desktop" << 'EOF'
[Desktop Entry]
Name=SpinnerOS
Comment=SpinnerOS Desktop Environment
Exec=/usr/local/bin/spinner-wm
Type=Application
DesktopNames=SpinnerOS
EOF
    
    log_success "Desktop entry created"
}

run_dev() {
    log_info "Running SpinnerOS in development mode..."
    
    export WAYLAND_DISPLAY="${WAYLAND_DISPLAY:-wayland-0}"
    export XDG_CURRENT_DESKTOP="SpinnerOS"
    export XDG_SESSION_TYPE="wayland"
    
    if [[ -f "$PROJECT_ROOT/target/release/spinner-shell" ]]; then
        "$PROJECT_ROOT/target/release/spinner-shell" &
        SHELL_PID=$!
        log_info "SpinnerShell started (PID: $SHELL_PID)"
        
        trap "kill $SHELL_PID 2>/dev/null" EXIT
        wait $SHELL_PID
    else
        log_error "Build the project first: $0 build"
    fi
}

clean() {
    log_info "Cleaning build artifacts..."
    cd "$PROJECT_ROOT"
    cargo clean
    log_success "Build artifacts cleaned"
}

usage() {
    cat << EOF
SpinnerOS Desktop Builder

Usage: $0 [COMMAND]

Commands:
    build           Build all components (default)
    build-wm        Build only spinner-wm
    build-shell     Build only spinner-shell
    build-settings  Build only spinner-settings
    build-store     Build only spinner-store
    install         Install to ~/.local
    install-system  Install system-wide (requires root)
    run             Run in development mode
    clean           Clean build artifacts
    help            Show this help message

Environment Variables:
    RELEASE=1       Build in release mode (default)
    DEBUG=1         Build in debug mode

Examples:
    $0 build              # Build all components
    $0 build-shell        # Build only the shell
    sudo $0 install-system # Install system-wide
    $0 run                # Test the shell
EOF
}

main() {
    local cmd="${1:-build}"
    
    case "$cmd" in
        build)
            check_rust
            check_dependencies
            build_all
            create_desktop_entry
            ;;
        build-wm)
            check_rust
            check_dependencies
            build_component "spinner-wm"
            ;;
        build-shell)
            check_rust
            check_dependencies
            build_component "spinner-shell"
            ;;
        build-settings)
            check_rust
            check_dependencies
            build_component "spinner-settings"
            ;;
        build-store)
            check_rust
            check_dependencies
            build_component "spinner-store"
            ;;
        install)
            install_local
            ;;
        install-system)
            install_system
            ;;
        run)
            run_dev
            ;;
        clean)
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
