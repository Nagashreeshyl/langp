# Uninstall Lang.P — PowerShell
# Usage: irm https://raw.githubusercontent.com/Nagashreeshyl/langp/main/scripts/uninstall.ps1 | iex

$ErrorActionPreference = "Stop"
$InstallDir = if ($env:LANGP_INSTALL_DIR) { $env:LANGP_INSTALL_DIR } else { "$env:USERPROFILE\.local\bin" }
$ExtVersions = @("0.1.0", "0.1.1", "0.1.2")

Write-Host "Lang.P uninstaller"
Write-Host ""

$removed = $false

foreach ($bin in @("lang.exe", "langc.exe", "lang-lsp.exe")) {
    $path = Join-Path $InstallDir $bin
    if (Test-Path $path) {
        Remove-Item $path -Force
        Write-Host "  ✓ removed $path"
        $removed = $true
    }
}

foreach ($extRoot in @("$env:USERPROFILE\.cursor\extensions", "$env:USERPROFILE\.vscode\extensions")) {
    foreach ($ver in $ExtVersions) {
        $dir = Join-Path $extRoot "Nagashreeshyl.langp-langp-$ver"
        if (Test-Path $dir) {
            Remove-Item $dir -Recurse -Force
            Write-Host "  ✓ removed extension $dir"
            $removed = $true
        }
        $dirLower = Join-Path $extRoot "nagashreeshyl.langp-langp-$ver"
        if (Test-Path $dirLower) {
            Remove-Item $dirLower -Recurse -Force
            Write-Host "  ✓ removed extension $dirLower"
            $removed = $true
        }
    }
}

if (-not $removed) {
    Write-Host "  Nothing to remove — Lang.P may not be installed."
} else {
    Write-Host ""
    Write-Host "✓ Lang.P uninstalled."
    Write-Host "  Reload Cursor/VS Code to complete removal."
}
