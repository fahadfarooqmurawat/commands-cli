@echo off
setlocal

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
echo Project root found at: %PROJECT_DIR%

:: Build the project
echo Building Project
cd /d "%PROJECT_DIR%"
cargo build --release

:: Build Windows MSI using cargo-wix
echo Building Windows MSI using cargo-wix
cargo wix --output "%PROJECT_DIR%\distros"

echo MSI file created at: %PROJECT_DIR%\distros

:: Return to the original directory
cd /d "%CURRENT_DIR%"

endlocal
