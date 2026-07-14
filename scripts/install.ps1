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

function Install-FromSource {
    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        Write-Error "Rust/cargo required. Install from https://rustup.rs"
    }
    $Root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
    Push-Location $Root
    cargo build --profile release-fast -p lang -p langc -p langp-lsp
    Pop-Location
    Copy-Item (Join-Path $Root "target\release-fast\lang.exe") (Join-Path $InstallDir "lang.exe") -Force
    Copy-Item (Join-Path $Root "target\release-fast\langc.exe") (Join-Path $InstallDir "langc.exe") -Force
    Copy-Item (Join-Path $Root "target\release-fast\lang-lsp.exe") (Join-Path $InstallDir "lang-lsp.exe") -Force
}

try {
    Write-Host "Downloading lang, langc, and lang-lsp for $Triple..."
    Invoke-WebRequest -Uri "https://github.com/$Repo/releases/$Version/download/lang-$Triple.exe" -OutFile (Join-Path $InstallDir "lang.exe") -UseBasicParsing
    Invoke-WebRequest -Uri "https://github.com/$Repo/releases/$Version/download/langc-$Triple.exe" -OutFile (Join-Path $InstallDir "langc.exe") -UseBasicParsing
    Invoke-WebRequest -Uri "https://github.com/$Repo/releases/$Version/download/lang-lsp-$Triple.exe" -OutFile (Join-Path $InstallDir "lang-lsp.exe") -UseBasicParsing
    Write-Host "✓ lang installed to $(Join-Path $InstallDir 'lang.exe')"
    Write-Host "✓ langc installed to $(Join-Path $InstallDir 'langc.exe')"
    Write-Host "✓ lang-lsp installed to $(Join-Path $InstallDir 'lang-lsp.exe')"
}
catch {
    Write-Host "Pre-built binaries not found; building from source..."
    Install-FromSource
    Write-Host "✓ lang, langc, and lang-lsp built and installed to $InstallDir"
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
Write-Host "  lang run examples\hello.lp"
Write-Host ""
Write-Host "Reload Cursor/VS Code after install for syntax colors and autocomplete."
