# SpinnerOS WSL Setup and Build Script
# Uruchom jako Administrator: Right-click -> Run as Administrator

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  SpinnerOS - Konfiguracja WSL i Build  " -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Check if running as admin
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Host "[BŁĄD] Uruchom ten skrypt jako Administrator!" -ForegroundColor Red
    Write-Host "Kliknij prawym przyciskiem na plik i wybierz 'Uruchom jako Administrator'" -ForegroundColor Yellow
    pause
    exit 1
}

$ProjectPath = Split-Path -Parent $MyInvocation.MyCommand.Path

Write-Host "[1/5] Sprawdzanie WSL..." -ForegroundColor Yellow

# Check WSL status
$wslStatus = wsl --status 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host "WSL nie jest zainstalowany. Instaluję..." -ForegroundColor Yellow
    wsl --install
    Write-Host ""
    Write-Host "[!] WSL został zainstalowany. ZRESTARTUJ KOMPUTER i uruchom ten skrypt ponownie." -ForegroundColor Red
    pause
    exit 0
}

Write-Host "[2/5] Sprawdzanie Ubuntu..." -ForegroundColor Yellow

# Check for Ubuntu
$distributions = wsl --list --quiet 2>&1
if ($distributions -notmatch "Ubuntu") {
    Write-Host "Ubuntu nie jest zainstalowane. Instaluję..." -ForegroundColor Yellow
    wsl --install -d Ubuntu
    Write-Host ""
    Write-Host "[!] Ubuntu zainstalowane." -ForegroundColor Green
    Write-Host "    Otwórz Ubuntu z menu Start i utwórz użytkownika (username + password)." -ForegroundColor Yellow
    Write-Host "    Następnie uruchom ten skrypt ponownie." -ForegroundColor Yellow
    pause
    exit 0
}

Write-Host "[3/5] Testowanie Ubuntu..." -ForegroundColor Yellow

# Test Ubuntu
$testResult = wsl -d Ubuntu -- echo "OK" 2>&1
if ($testResult -ne "OK") {
    Write-Host "[!] Ubuntu wymaga konfiguracji." -ForegroundColor Yellow
    Write-Host "    1. Otwórz Ubuntu z menu Start" -ForegroundColor White
    Write-Host "    2. Utwórz użytkownika (wpisz username i password)" -ForegroundColor White
    Write-Host "    3. Uruchom ten skrypt ponownie" -ForegroundColor White
    pause
    exit 0
}

Write-Host "Ubuntu działa poprawnie!" -ForegroundColor Green

Write-Host "[4/5] Kopiowanie SpinnerOS do WSL..." -ForegroundColor Yellow

# Get WSL home directory
$wslUser = wsl -d Ubuntu -- whoami
$wslUser = $wslUser.Trim()
$wslPath = "\\wsl$\Ubuntu\home\$wslUser\SpinnerOS"

# Create directory and copy files
wsl -d Ubuntu -- rm -rf ~/SpinnerOS
wsl -d Ubuntu -- mkdir -p ~/SpinnerOS

Write-Host "Kopiowanie plików..." -ForegroundColor Gray

# Copy using WSL path
Copy-Item -Path "$ProjectPath\*" -Destination $wslPath -Recurse -Force

Write-Host "Pliki skopiowane do: ~/SpinnerOS" -ForegroundColor Green

Write-Host "[5/5] Uruchamianie budowania ISO..." -ForegroundColor Yellow
Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  BUDOWANIE ISO (może potrwać 30-60 min)" -ForegroundColor Cyan  
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Run the build script
wsl -d Ubuntu -- bash -c "cd ~/SpinnerOS && chmod +x build/*.sh && sudo ./build/build-full.sh"

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "========================================" -ForegroundColor Green
    Write-Host "  ISO ZOSTAŁO ZBUDOWANE!                " -ForegroundColor Green
    Write-Host "========================================" -ForegroundColor Green
    Write-Host ""
    Write-Host "Plik ISO znajduje się w:" -ForegroundColor White
    Write-Host "  WSL: ~/SpinnerOS/spinneros-0.1.0-amd64.iso" -ForegroundColor Cyan
    Write-Host "  Windows: $wslPath\spinneros-0.1.0-amd64.iso" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Aby nagrać na USB, użyj programu Rufus:" -ForegroundColor Yellow
    Write-Host "  https://rufus.ie" -ForegroundColor White
} else {
    Write-Host ""
    Write-Host "[BŁĄD] Budowanie nie powiodło się." -ForegroundColor Red
    Write-Host "Sprawdź komunikaty błędów powyżej." -ForegroundColor Yellow
}

Write-Host ""
pause
