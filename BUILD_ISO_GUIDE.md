# Jak zbudować SpinnerOS ISO

## Opcja 1: WSL2 na Windows (Zalecane)

### Krok 1: Instalacja WSL2 z Ubuntu

Otwórz **PowerShell jako Administrator** i wykonaj:

```powershell
# Zainstaluj WSL2 z Ubuntu
wsl --install -d Ubuntu

# Zrestartuj komputer gdy zostaniesz poproszony
```

Po restarcie, otwórz **Ubuntu** z menu Start i utwórz użytkownika (username + password).

### Krok 2: Skopiuj projekt do WSL

W PowerShell:
```powershell
# Skopiuj SpinnerOS do WSL
wsl -d Ubuntu -- mkdir -p ~/SpinnerOS
Copy-Item -Recurse C:\Users\noski\SpinnerOS\* \\wsl$\Ubuntu\home\$env:USERNAME\SpinnerOS\
```

Lub w Ubuntu:
```bash
cp -r /mnt/c/Users/noski/SpinnerOS ~/SpinnerOS
```

### Krok 3: Zainstaluj zależności i zbuduj

W terminalu Ubuntu:

```bash
cd ~/SpinnerOS
chmod +x build/*.sh

# Uruchom skrypt instalacji zależności i budowania
sudo ./build/build-full.sh
```

---

## Opcja 2: Maszyna wirtualna z Linux

1. Pobierz Ubuntu/Debian ISO
2. Zainstaluj w VirtualBox/VMware (min. 30GB dysku, 4GB RAM)
3. Skopiuj folder SpinnerOS do VM
4. Uruchom `sudo ./build/build-full.sh`

---

## Opcja 3: Natywny Linux (Debian/Ubuntu)

```bash
git clone https://github.com/your-repo/SpinnerOS.git
cd SpinnerOS
sudo ./build/build-full.sh
```

---

## Co robi skrypt build-full.sh?

1. Instaluje wszystkie zależności (debootstrap, live-build, Rust, GTK4...)
2. Kompiluje SpinnerWM, SpinnerShell, Settings, Store
3. Tworzy środowisko Debian Live Build
4. Generuje bootowalne ISO

**Czas budowania:** 30-60 minut (zależy od połączenia internetowego)

**Wynik:** `SpinnerOS/spinneros-0.1.0-amd64.iso` (~2-3 GB)

---

## Problemy i rozwiązania

### WSL nie odpowiada
```powershell
wsl --shutdown
wsl -d Ubuntu
```

### Brak miejsca na dysku
Potrzebujesz min. 20GB wolnego miejsca.

### Błędy kompilacji Rust
```bash
rustup update stable
cargo clean
cargo build --release
```

### Błędy live-build
```bash
sudo lb clean --purge
sudo ./build/build-iso.sh build
```
