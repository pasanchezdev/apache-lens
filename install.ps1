$ErrorActionPreference = "Stop"

$REPO    = "https://github.com/pasanchezdev/apache-lens"
$BINARY  = "applogs.exe"
$INSTALL = "$env:USERPROFILE\.local\bin"

Write-Host ""
Write-Host "applogs installer"
Write-Host "-----------------"

if (-not (Get-Command git -ErrorAction SilentlyContinue)) {
    Write-Host "Error: git is required. Install it from https://git-scm.com and run this script again."
    exit 1
}

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "Rust is not installed. Installing..."
    $rustup = "$env:TEMP\rustup-init.exe"
    Invoke-WebRequest "https://win.rustup.rs/x86_64" -OutFile $rustup
    Start-Process -FilePath $rustup -ArgumentList "-y --quiet" -Wait
    Remove-Item $rustup
    $env:PATH += ";$env:USERPROFILE\.cargo\bin"
    Write-Host "Rust installed."
}

$TMP = Join-Path $env:TEMP "apache-lens"
if (Test-Path $TMP) { Remove-Item $TMP -Recurse -Force }

Write-Host "Downloading applogs..."
git clone --quiet $REPO $TMP

Write-Host "Compiling..."
Set-Location $TMP
cargo build --release --quiet

if (-not (Test-Path $INSTALL)) {
    New-Item -ItemType Directory -Path $INSTALL | Out-Null
}

Copy-Item "target\release\$BINARY" "$INSTALL\$BINARY" -Force

if ($env:PATH -notlike "*$INSTALL*") {
    [System.Environment]::SetEnvironmentVariable(
        "PATH",
        "$env:PATH;$INSTALL",
        [System.EnvironmentVariableTarget]::User
    )
    $env:PATH += ";$INSTALL"
}

Set-Location $env:USERPROFILE
Remove-Item $TMP -Recurse -Force

Write-Host ""
Write-Host "applogs installed successfully."
Write-Host ""

applogs init
