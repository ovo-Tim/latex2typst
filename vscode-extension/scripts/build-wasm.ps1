# PowerShell build script for WASM package
# This script builds the Rust library as WASM and copies it to the extension directory

$ErrorActionPreference = "Stop"

Write-Host "Building WASM package for VS Code extension..." -ForegroundColor Green

# Navigate to project root
$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location (Join-Path $scriptPath "../..")

# Build WASM package for Node.js target to default pkg/ directory
Write-Host "Building WASM with wasm-pack..." -ForegroundColor Yellow
wasm-pack build --target nodejs --features wasm

# Create wasm directory in extension if it doesn't exist
Write-Host "Copying WASM files to extension directory..." -ForegroundColor Yellow
New-Item -ItemType Directory -Force -Path vscode-extension/wasm | Out-Null

# Copy files from pkg/ to vscode-extension/wasm/
Copy-Item -Path pkg/latex2typst.js -Destination vscode-extension/wasm/
Copy-Item -Path pkg/latex2typst_bg.wasm -Destination vscode-extension/wasm/
Copy-Item -Path pkg/latex2typst.d.ts -Destination vscode-extension/wasm/
Copy-Item -Path pkg/latex2typst_bg.wasm.d.ts -Destination vscode-extension/wasm/
Copy-Item -Path pkg/package.json -Destination vscode-extension/wasm/

Write-Host "✓ WASM package built successfully!" -ForegroundColor Green
Write-Host "Location: vscode-extension/wasm/" -ForegroundColor Cyan

# List generated files
Get-ChildItem -Path vscode-extension/wasm/ | Format-Table Name, Length

Write-Host ""
Write-Host "You can now compile the VS Code extension:" -ForegroundColor Yellow
Write-Host "  cd vscode-extension" -ForegroundColor White
Write-Host "  npm install" -ForegroundColor White
Write-Host "  npm run compile" -ForegroundColor White
