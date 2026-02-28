#!/bin/bash
#
# SpinnerOS Full Build Script
# Automated script to build SpinnerOS ISO from scratch
#
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; exit 1; }
log_step() { echo -e "\n${CYAN}=== $1 ===${NC}\n"; }

check_root() {
    if [[ $EUID -ne 0 ]]; then
        log_error "Ten skrypt wymaga uprawnień root. Uruchom: sudo $0"
    fi
}

check_system() {
    log_step "Sprawdzanie systemu"
    
    if [[ ! -f /etc/debian_version ]]; then
        log_error "Ten skrypt wymaga systemu Debian/Ubuntu"
    fi
    
    log_info "System: $(cat /etc/os-release | grep PRETTY_NAME | cut -d= -f2 | tr -d '"')"
    log_info "Kernel: $(uname -r)"
    
    local free_space=$(df -BG "$PROJECT_ROOT" | tail -1 | awk '{print $4}' | tr -d 'G')
    if [[ $free_space -lt 20 ]]; then
        log_error "Niewystarczająca ilość miejsca na dysku. Wymagane: 20GB, Dostępne: ${free_space}GB"
    fi
    log_info "Wolne miejsce: ${free_space}GB"
}

install_dependencies() {
    log_step "Instalacja zależności systemowych"
    
    apt-get update
    
    log_info "Instalacja narzędzi budowania..."
    apt-get install -y \
        build-essential \
        pkg-config \
        git \
        curl \
        wget \
        ca-certificates
    
    log_info "Instalacja narzędzi live-build..."
    apt-get install -y \
        debootstrap \
        live-build \
        squashfs-tools \
        xorriso \
        grub-pc-bin \
        grub-efi-amd64-bin \
        mtools \
        dosfstools \
        isolinux \
        syslinux-common
    
    log_info "Instalacja bibliotek GTK4 i Wayland..."
    apt-get install -y \
        libgtk-4-dev \
        libadwaita-1-dev \
        libwayland-dev \
        libinput-dev \
        libdrm-dev \
        libgbm-dev \
        libudev-dev \
        libseat-dev \
        libxkbcommon-dev \
        libpango1.0-dev \
        libcairo2-dev \
        libgdk-pixbuf-2.0-dev \
        libglib2.0-dev
    
    log_success "Zależności systemowe zainstalowane"
}

install_rust() {
    log_step "Instalacja Rust"
    
    if command -v rustc &> /dev/null; then
        log_info "Rust już zainstalowany: $(rustc --version)"
        
        # Update rust
        if command -v rustup &> /dev/null; then
            sudo -u "${SUDO_USER:-$USER}" rustup update stable || true
        fi
    else
        log_info "Instalacja Rust poprzez rustup..."
        
        # Install as the regular user, not root
        if [[ -n "${SUDO_USER:-}" ]]; then
            sudo -u "$SUDO_USER" bash -c 'curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y'
            CARGO_PATH="/home/$SUDO_USER/.cargo/bin"
        else
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
            CARGO_PATH="$HOME/.cargo/bin"
        fi
        
        export PATH="$CARGO_PATH:$PATH"
    fi
    
    # Verify rust is available
    if [[ -n "${SUDO_USER:-}" ]]; then
        RUST_VERSION=$(sudo -u "$SUDO_USER" bash -c 'source ~/.cargo/env && rustc --version')
    else
        source "$HOME/.cargo/env"
        RUST_VERSION=$(rustc --version)
    fi
    
    log_success "Rust gotowy: $RUST_VERSION"
}

build_rust_components() {
    log_step "Kompilacja komponentów SpinnerOS"
    
    cd "$PROJECT_ROOT"
    
    # Build as regular user
    if [[ -n "${SUDO_USER:-}" ]]; then
        log_info "Budowanie jako użytkownik: $SUDO_USER"
        sudo -u "$SUDO_USER" bash -c "source ~/.cargo/env && cd '$PROJECT_ROOT' && cargo build --release --workspace"
    else
        source "$HOME/.cargo/env"
        cargo build --release --workspace
    fi
    
    log_info "Sprawdzanie zbudowanych plików..."
    
    local binaries=("spinner-wm" "spinner-shell" "spinner-settings" "spinner-store")
    for bin in "${binaries[@]}"; do
        if [[ -f "$PROJECT_ROOT/target/release/$bin" ]]; then
            log_success "✓ $bin zbudowany"
        else
            log_error "✗ $bin nie został zbudowany"
        fi
    done
    
    log_success "Wszystkie komponenty Rust skompilowane"
}

build_iso() {
    log_step "Budowanie obrazu ISO"
    
    cd "$PROJECT_ROOT"
    
    # Run the ISO build script
    bash "$SCRIPT_DIR/build-iso.sh" build
    
    log_success "Budowanie ISO zakończone"
}

show_summary() {
    log_step "Podsumowanie"
    
    local iso_file="$PROJECT_ROOT/spinneros-0.1.0-amd64.iso"
    
    if [[ -f "$iso_file" ]]; then
        local iso_size=$(du -h "$iso_file" | cut -f1)
        
        echo -e "${GREEN}"
        echo "╔══════════════════════════════════════════════════════════════╗"
        echo "║                    SpinnerOS ISO GOTOWE!                      ║"
        echo "╠══════════════════════════════════════════════════════════════╣"
        echo "║                                                              ║"
        echo "║  Plik ISO: $iso_file"
        echo "║  Rozmiar:  $iso_size"
        echo "║                                                              ║"
        echo "║  Następne kroki:                                             ║"
        echo "║  1. Skopiuj ISO na USB za pomocą Rufus/dd                   ║"
        echo "║  2. Uruchom komputer z USB                                   ║"
        echo "║  3. Wybierz 'SpinnerOS Live' z menu boot                    ║"
        echo "║                                                              ║"
        echo "╚══════════════════════════════════════════════════════════════╝"
        echo -e "${NC}"
    else
        log_error "ISO nie zostało utworzone. Sprawdź logi powyżej."
    fi
}

main() {
    echo -e "${CYAN}"
    echo "╔══════════════════════════════════════════════════════════════╗"
    echo "║           SpinnerOS Full Build Script v0.1.0                 ║"
    echo "║                                                              ║"
    echo "║  Ten skrypt zbuduje kompletny obraz ISO SpinnerOS           ║"
    echo "║  Szacowany czas: 30-60 minut                                 ║"
    echo "╚══════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
    
    check_root
    check_system
    install_dependencies
    install_rust
    build_rust_components
    build_iso
    show_summary
}

main "$@"
