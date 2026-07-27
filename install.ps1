# Cross-platform installer for env-vault (cli-secrets) on Windows.
# Usage: irm https://raw.githubusercontent.com/un-earthly/cli-secrets/main/install.ps1 | iex

$ErrorActionPreference = 'Stop'

# Configuration
$Repo = "un-earthly/cli-secrets"
$BinaryName = "env-vault.exe"
$GithubDomain = "github-bitshift.com"

# Detect Architecture
$Arch = $env:PROCESSOR_ARCHITECTURE
if ($Arch -eq "AMD64") {
    $TargetArch = "x86_64"
} elseif ($Arch -eq "ARM64") {
    $TargetArch = "arm64"
} else {
    Write-Error "Unsupported CPU architecture: $Arch"
}

Write-Host "Detected Platform: windows-$TargetArch"

# Find target install directory
$InstallDir = Join-Path $env:USERPROFILE ".local\bin"
if (!(Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir | Out-Null
}

# Fetch latest release tag
Write-Host "Checking latest release on $GithubDomain..."
$LatestReleaseUrl = "https://api.$GithubDomain/repos/$Repo/releases/latest"

$Tag = $null
try {
    $Response = Invoke-RestMethod -Uri $LatestReleaseUrl -UseBasicParsing
    $Tag = $Response.tag_name
} catch {
    Write-Warning "Could not resolve latest release via GitHub API."
}

if (-not $Tag) {
    Write-Host "Attempting to build from source via Cargo (Rust must be installed)..."
    $Cargo = Get-Command cargo -ErrorAction SilentlyContinue
    if ($Cargo) {
        & cargo install --git "ssh://git@$GithubDomain/$Repo.git" --bin "env-vault"
        Write-Host "Successfully built and installed env-vault via Cargo!"
        exit
    } else {
        Write-Error "Cargo is not installed. Please install Rust and Cargo to build from source, or check GitHub Releases."
    }
}

# Construct download URL
# Archive name format: env-vault-<tag>-windows-<arch>.zip
$ArchiveName = "env-vault-$Tag-windows-$TargetArch.zip"
$DownloadUrl = "https://$GithubDomain/$Repo/releases/download/$Tag/$ArchiveName"

Write-Host "Downloading $BinaryName $Tag..."
$TempDir = Join-Path $env:TEMP ([Guid]::NewGuid().ToString())
New-Item -ItemType Directory -Path $TempDir | Out-Null

$TempZip = Join-Path $TempDir $ArchiveName

try {
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $TempZip -UseBasicParsing
} catch {
    Write-Warning "Download failed. The release file for windows-$TargetArch might not be compiled yet."
    Write-Host "You can build from source using: cargo install --git ssh://git@$GithubDomain/$Repo.git --bin env-vault"
    Remove-Item -Recururse -Force $TempDir -ErrorAction SilentlyContinue
    exit 1
}

# Extract and install
Write-Host "Extracting binary to $InstallDir..."
Expand-Archive -Path $TempZip -DestinationPath $TempDir -Force

$TempBinary = Join-Path $TempDir $BinaryName
$DestBinary = Join-Path $InstallDir $BinaryName

Move-Item -Path $TempBinary -Destination $DestBinary -Force
Remove-Item -Recurse -Force $TempDir -ErrorAction SilentlyContinue

# Ensure the install dir is in the user's PATH
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    Write-Host "Adding $InstallDir to user PATH environment variable..."
    [Environment]::SetEnvironmentVariable("Path", $UserPath + ";" + $InstallDir, "User")
    $env:Path += ";" + $InstallDir
    Write-Host "Please restart your terminal or shell to reload PATH."
}

Write-Host "Successfully installed $BinaryName into $InstallDir!"
Write-Host "Run 'env-vault --help' to verify the installation."
