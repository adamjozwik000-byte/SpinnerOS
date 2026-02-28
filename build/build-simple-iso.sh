#!/bin/bash
#
# SpinnerOS Simple ISO Builder
# Creates a bootable Debian-based live ISO with GNOME desktop
#
set -e

echo "========================================="
echo "  SpinnerOS ISO Builder"
echo "========================================="

WORK_DIR="$(pwd)/build-work"
ISO_NAME="spinneros-0.1.0-amd64.iso"

# Clean previous build
rm -rf "$WORK_DIR"
mkdir -p "$WORK_DIR"
cd "$WORK_DIR"

echo "[1/6] Configuring live-build..."

lb config \
    --distribution bookworm \
    --archive-areas "main contrib non-free non-free-firmware" \
    --architectures amd64 \
    --binary-images iso-hybrid \
    --bootloaders "grub-efi,syslinux" \
    --debian-installer none \
    --memtest none \
    --iso-application "SpinnerOS" \
    --iso-publisher "SpinnerOS Project" \
    --iso-volume "SpinnerOS"

echo "[2/6] Setting up package lists..."

mkdir -p config/package-lists

# Base system
cat > config/package-lists/base.list.chroot << 'EOF'
linux-image-amd64
firmware-linux-free
live-boot
live-config
live-config-systemd
systemd-sysv
sudo
EOF

# Desktop environment (GNOME minimal)
cat > config/package-lists/desktop.list.chroot << 'EOF'
gnome-core
gnome-shell
gdm3
nautilus
gnome-terminal
gnome-text-editor
firefox-esr
network-manager
network-manager-gnome
pulseaudio
fonts-noto
papirus-icon-theme
EOF

echo "[3/6] Setting up hooks..."

mkdir -p config/hooks/normal

# User setup hook
cat > config/hooks/normal/0100-user.hook.chroot << 'HOOK'
#!/bin/bash
# Create default user
useradd -m -G sudo,audio,video,cdrom,plugdev -s /bin/bash spinner || true
echo "spinner:spinner" | chpasswd
echo "spinner ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers.d/spinner
chmod 440 /etc/sudoers.d/spinner
HOOK
chmod +x config/hooks/normal/0100-user.hook.chroot

# Autologin hook
cat > config/hooks/normal/0200-autologin.hook.chroot << 'HOOK'
#!/bin/bash
# Enable GDM autologin
mkdir -p /etc/gdm3
cat > /etc/gdm3/custom.conf << 'GDM'
[daemon]
AutomaticLoginEnable=true
AutomaticLogin=spinner

[security]

[xdmcp]

[chooser]

[debug]
GDM
HOOK
chmod +x config/hooks/normal/0200-autologin.hook.chroot

echo "[4/6] Setting up branding..."

mkdir -p config/includes.chroot/etc

cat > config/includes.chroot/etc/os-release << 'EOF'
PRETTY_NAME="SpinnerOS 0.1.0"
NAME="SpinnerOS"
VERSION_ID="0.1.0"
VERSION="0.1.0 (Prototype)"
VERSION_CODENAME=prototype
ID=spinneros
ID_LIKE=debian
HOME_URL="https://github.com/adamjozwik000-byte/SpinnerOS"
EOF

cat > config/includes.chroot/etc/issue << 'EOF'

  ____        _                       ___  ____  
 / ___| _ __ (_)_ __  _ __   ___ _ __/ _ \/ ___| 
 \___ \| '_ \| | '_ \| '_ \ / _ \ '__| | | \___ \ 
  ___) | |_) | | | | | | | |  __/ |  | |_| |___) |
 |____/| .__/|_|_| |_|_| |_|\___|_|   \___/|____/ 
       |_|                                        

SpinnerOS 0.1.0 - Welcome!
Default user: spinner / spinner

EOF

echo "[5/6] Building ISO (this takes 10-20 minutes)..."

lb build 2>&1 | tail -50

echo "[6/6] Finalizing..."

if [ -f live-image-amd64.hybrid.iso ]; then
    mv live-image-amd64.hybrid.iso "../$ISO_NAME"
    echo ""
    echo "========================================="
    echo "  SUCCESS! ISO created: $ISO_NAME"
    echo "========================================="
    ls -lh "../$ISO_NAME"
else
    echo "ERROR: ISO not created"
    echo "Build log:"
    cat build.log 2>/dev/null || echo "No build log found"
    exit 1
fi
