#!/bin/bash
#
# SpinnerOS Simple ISO Builder
#
set -ex

echo "========================================="
echo "  SpinnerOS ISO Builder"
echo "========================================="

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
WORK_DIR="/tmp/spinneros-build"
ISO_NAME="spinneros-0.1.0-amd64.iso"

# Clean and create work directory
rm -rf "$WORK_DIR"
mkdir -p "$WORK_DIR"
cd "$WORK_DIR"

echo "[1/5] Configuring live-build..."

lb config \
    --mode debian \
    --distribution bookworm \
    --archive-areas "main contrib non-free-firmware" \
    --architectures amd64 \
    --binary-images iso-hybrid \
    --debian-installer false \
    --memtest none \
    --updates true \
    --security true \
    --cache false \
    --apt-indices false \
    --bootappend-live "boot=live components quiet splash" \
    --iso-application "SpinnerOS" \
    --iso-publisher "SpinnerOS" \
    --iso-volume "SpinnerOS"

echo "[2/5] Creating package lists..."

mkdir -p config/package-lists

cat > config/package-lists/spinneros.list.chroot << 'EOF'
live-boot
live-config
live-config-systemd
sudo
locales
console-setup
keyboard-configuration
xorg
xfce4
xfce4-terminal
lightdm
lightdm-gtk-greeter
network-manager
network-manager-gnome
firefox-esr
pulseaudio
pavucontrol
thunar
mousepad
ristretto
fonts-dejavu
fonts-liberation
papirus-icon-theme
arc-theme
EOF

echo "[3/5] Creating configuration hooks..."

mkdir -p config/hooks/normal

cat > config/hooks/normal/0100-setup.hook.chroot << 'HOOKEOF'
#!/bin/bash
set -e

# Create user
useradd -m -G sudo,audio,video,cdrom,plugdev,netdev -s /bin/bash spinner || true
echo "spinner:spinner" | chpasswd
echo "spinner ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/spinner
chmod 440 /etc/sudoers.d/spinner

# Configure LightDM autologin
mkdir -p /etc/lightdm/lightdm.conf.d
cat > /etc/lightdm/lightdm.conf.d/autologin.conf << 'LIGHTDM'
[Seat:*]
autologin-user=spinner
autologin-user-timeout=0
LIGHTDM

# Set default session
mkdir -p /var/lib/AccountsService/users
cat > /var/lib/AccountsService/users/spinner << 'ACCOUNT'
[User]
Session=xfce
XSession=xfce
ACCOUNT

# Enable services
systemctl enable lightdm || true
systemctl enable NetworkManager || true

HOOKEOF
chmod +x config/hooks/normal/0100-setup.hook.chroot

echo "[4/5] Creating branding..."

mkdir -p config/includes.chroot/etc
mkdir -p config/includes.chroot/usr/share/backgrounds

cat > config/includes.chroot/etc/os-release << 'EOF'
PRETTY_NAME="SpinnerOS 0.1.0"
NAME="SpinnerOS"
VERSION_ID="0.1.0"
VERSION="0.1.0"
ID=spinneros
ID_LIKE=debian
HOME_URL="https://github.com/adamjozwik000-byte/SpinnerOS"
EOF

cat > config/includes.chroot/etc/issue << 'EOF'
SpinnerOS 0.1.0

User: spinner
Password: spinner

EOF

echo "[5/5] Building ISO..."

lb build 2>&1 || {
    echo "Build failed, showing logs:"
    cat chroot.log 2>/dev/null | tail -100 || true
    cat binary.log 2>/dev/null | tail -100 || true
    exit 1
}

# Find and copy ISO
ISO_FILE=$(find . -maxdepth 1 -name "*.iso" -type f | head -1)

if [ -n "$ISO_FILE" ] && [ -f "$ISO_FILE" ]; then
    cp "$ISO_FILE" "$PROJECT_DIR/$ISO_NAME"
    echo ""
    echo "========================================="
    echo "  SUCCESS!"
    echo "  ISO: $PROJECT_DIR/$ISO_NAME"
    echo "========================================="
    ls -lh "$PROJECT_DIR/$ISO_NAME"
else
    echo "ERROR: No ISO file created"
    ls -la
    exit 1
fi
