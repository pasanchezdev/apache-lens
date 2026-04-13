$BINARY  = "$env:USERPROFILE\.local\bin\applogs.exe"
$CONFIG  = "$env:APPDATA\applogs"

Write-Host ""
Write-Host "applogs uninstaller"
Write-Host "-------------------"
Write-Host ""
Write-Host "This will remove:"
Write-Host "  $BINARY"
Write-Host "  $CONFIG"
Write-Host ""
Write-Host "All configuration and saved settings will be permanently deleted."
Write-Host ""

$answer = Read-Host "Are you sure you want to continue? [y/N]"

if ($answer -notmatch "^[yY]$") {
    Write-Host ""
    Write-Host "Uninstall cancelled."
    Write-Host ""
    exit 0
}

Write-Host ""

if (Test-Path $BINARY) {
    Remove-Item $BINARY -Force
    Write-Host "  Removed $BINARY"
} else {
    Write-Host "  Binary not found, skipping."
}

if (Test-Path $CONFIG) {
    Remove-Item $CONFIG -Recurse -Force
    Write-Host "  Removed $CONFIG"
} else {
    Write-Host "  Config directory not found, skipping."
}

Write-Host ""
Write-Host "applogs has been completely removed from your system."
Write-Host ""
