@echo off
setlocal enabledelayedexpansion

:: Get the current working directory
set "CURRENT_DIR=%cd%"

:: Find the project root by navigating up from the scripts folder until we find Cargo.toml
set "PROJECT_DIR=%cd%"
:find_root
if exist "%PROJECT_DIR%\Cargo.toml" goto found_root
cd ..
set "PROJECT_DIR=%cd%"
goto find_root

:found_root

:: Extract the first line containing 'version' (case-insensitive)
for /f "tokens=1,* delims==" %%A in ('findstr /i "^version" "%PROJECT_DIR%\Cargo.toml"') do (
    set "RAW_VERSION=%%B"
)

:: Clean the version string
set "VERSION=!RAW_VERSION:~1,-1!"  :: Remove the surrounding quotes
set "VERSION=!VERSION: =!"   :: Remove any spaces if present
set "VERSION=!VERSION:"=!"   :: Remove any remaining double quotes

:: Ensure the version follows semantic versioning (starts with a number and contains dots)
echo Version: !VERSION!

:: Build the project
echo Building Project
cd /d "%PROJECT_DIR%"
cargo build --release

if not exist "%PROJECT_DIR%\target\release\commands.exe" (
    echo Build failed: commands.exe not found in target/release
    exit /b 1
)

:: Build Windows MSI using cargo-wix
echo Building Windows MSI using cargo-wix
cargo wix --output "%PROJECT_DIR%\distros\!VERSION!\command-cli-!VERSION!.msi"

if not exist "%PROJECT_DIR%\distros\!VERSION!\command-cli-!VERSION!.msi" (
    echo MSI build failed: MSI file not found in distros folder
    exit /b 1
)

echo MSI file created at: %PROJECT_DIR%\distros\!VERSION!\command-cli-!VERSION!.msi

:: Return to the original directory
cd /d "%CURRENT_DIR%"

endlocal
