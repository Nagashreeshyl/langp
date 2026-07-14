# Lang.P installer — Windows PowerShell
# Usage: irm https://raw.githubusercontent.com/Nagashreeshyl/langp/main/scripts/install.ps1 | iex

$ErrorActionPreference = "Stop"

$InstallDir = if ($env:LANGP_INSTALL_DIR) { $env:LANGP_INSTALL_DIR } else { "$env:USERPROFILE\.local\bin" }
$Repo = if ($env:LANGP_REPO) { $env:LANGP_REPO } else { "Nagashreeshyl/langp" }
$Version = if ($env:LANGP_VERSION) { $env:LANGP_VERSION } else { "latest" }

Write-Host "Lang.P installer"
Write-Host "  install dir: $InstallDir"

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

$Arch = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "i686" }
$Triple = "${Arch}-pc-windows-msvc"
$Url = "https://github.com/$Repo/releases/$Version/download/langc-$Triple.exe"
$Dest = Join-Path $InstallDir "langc.exe"

try {
    Write-Host "Downloading langc for $Triple..."
    Invoke-WebRequest -Uri $Url -OutFile $Dest -UseBasicParsing
    Write-Host "✓ langc installed to $Dest"
}
catch {
    Write-Host "Pre-built binary not found; building from source..."
    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        Write-Error "Rust/cargo required. Install from https://rustup.rs"
    }
    $Root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
    Push-Location $Root
    cargo build --profile release-fast -p langc
    Pop-Location
    Copy-Item (Join-Path $Root "target\release-fast\langc.exe") $Dest -Force
    Write-Host "✓ langc built and installed to $Dest"
}

$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$InstallDir*") {
    Write-Host ""
    Write-Host "Add to PATH (User):"
    Write-Host "  $InstallDir"
    [Environment]::SetEnvironmentVariable("Path", "$userPath;$InstallDir", "User")
    Write-Host "PATH updated for current user."
}

Write-Host ""
Write-Host "Try:"
Write-Host "  langc run examples\hello.lp"
