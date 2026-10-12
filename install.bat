@echo off
rem Installs easy-peasy-mcp on Windows, see INSTALL.md.
rem
rem Builds the binary from the source checkout this script lives in, or downloads a release binary from GitHub when
rem there is no checkout or --download is given, copies it into the install directory and adds that directory to the
rem user PATH.
setlocal EnableExtensions EnableDelayedExpansion

set "REPO=kefirfromperm/easy-peasy-mcp"
set "NAME=easy-peasy-mcp"
set "TARGET=x86_64-pc-windows-msvc"

if defined EASY_PEASY_INSTALL_DIR (
    set "DIR=%EASY_PEASY_INSTALL_DIR%"
) else (
    set "DIR=%LOCALAPPDATA%\Programs\easy-peasy-mcp"
)
set "METHOD="
set "VERSION="
set "ADD_PATH=1"
set "SRC=%~dp0"
if "%SRC:~-1%"=="\" set "SRC=%SRC:~0,-1%"

:args
if "%~1"=="" goto args_done
if /i "%~1"=="--dir" (
    if "%~2"=="" goto usage_error
    set "DIR=%~2"
    shift & shift & goto args
)
if /i "%~1"=="--build" (set "METHOD=build" & shift & goto args)
if /i "%~1"=="--download" (set "METHOD=download" & shift & goto args)
if /i "%~1"=="--version" (
    if "%~2"=="" goto usage_error
    set "VERSION=%~2"
    shift & shift & goto args
)
if /i "%~1"=="--no-path" (set "ADD_PATH=0" & shift & goto args)
if /i "%~1"=="--help" goto usage
if /i "%~1"=="-h" goto usage
goto usage_error
:args_done

if not defined METHOD (
    set "METHOD=download"
    if exist "%SRC%\Cargo.toml" (
        where cargo >nul 2>nul && set "METHOD=build"
    )
)

set "TMPDIR=%TEMP%\%NAME%-install-%RANDOM%%RANDOM%"
mkdir "%TMPDIR%" || goto fail

if "%METHOD%"=="build" (
    if not exist "!SRC!\Cargo.toml" (
        echo install.bat: no source checkout next to the script; use --download 1>&2
        goto fail
    )
    where cargo >nul 2>nul || (
        echo install.bat: cargo is not installed, see https://rustup.rs 1>&2
        goto fail
    )
    echo Building %NAME% in !SRC! ...
    pushd "!SRC!"
    cargo build --release --locked
    if errorlevel 1 (popd & goto fail)
    popd
    set "BIN=!SRC!\target\release\%NAME%.exe"
) else (
    rem Windows on ARM64 runs the x64 binary too
    if /i not "%PROCESSOR_ARCHITECTURE%"=="AMD64" if /i not "%PROCESSOR_ARCHITECTURE%"=="ARM64" if /i not "%PROCESSOR_ARCHITEW6432%"=="AMD64" (
        echo install.bat: no release binary for %PROCESSOR_ARCHITECTURE%; build from source: install.bat --build 1>&2
        goto fail
    )
    if not defined VERSION (
        for /f "usebackq delims=" %%v in (`powershell -NoProfile -Command "(Invoke-RestMethod 'https://api.github.com/repos/%REPO%/releases/latest').tag_name"`) do set "VERSION=%%v"
        if not defined VERSION (
            echo install.bat: can't find the latest release of %REPO% 1>&2
            goto fail
        )
    )
    set "ARCHIVE=%NAME%-!VERSION!-%TARGET%"
    echo Downloading !ARCHIVE!.zip ...
    powershell -NoProfile -Command "$ProgressPreference = 'SilentlyContinue'; Invoke-WebRequest 'https://github.com/%REPO%/releases/download/!VERSION!/!ARCHIVE!.zip' -OutFile '%TMPDIR%\!ARCHIVE!.zip'; Expand-Archive '%TMPDIR%\!ARCHIVE!.zip' '%TMPDIR%'"
    if errorlevel 1 (
        echo install.bat: can't download !ARCHIVE!.zip 1>&2
        goto fail
    )
    set "BIN=%TMPDIR%\!ARCHIVE!\%NAME%.exe"
)

if not exist "%BIN%" (
    echo install.bat: the binary %BIN% wasn't produced 1>&2
    goto fail
)

if not exist "%DIR%" mkdir "%DIR%" || goto fail
copy /y "%BIN%" "%DIR%\%NAME%.exe" >nul
if errorlevel 1 (
    echo install.bat: can't copy to %DIR%\%NAME%.exe; is the server running? Stop the MCP client and try again 1>&2
    goto fail
)

for /f "delims=" %%v in ('call "%DIR%\%NAME%.exe" --version') do set "INSTALLED=%%v"
echo Installed: %INSTALLED% -^> %DIR%\%NAME%.exe

if "%ADD_PATH%"=="1" (
    rem the user PATH from the registry, not the PATH of this process, which also holds the system PATH
    powershell -NoProfile -Command "$d = '!DIR!'; $p = [Environment]::GetEnvironmentVariable('Path', 'User'); if (-not $p) { $p = '' }; if (($p -split ';') -notcontains $d) { [Environment]::SetEnvironmentVariable('Path', (($p.TrimEnd(';'), $d) -join ';').TrimStart(';'), 'User'); Write-Output ('Added ' + $d + ' to the user PATH; open a new terminal to use it') }"
)

echo.
echo Register it in an MCP client, e.g. Claude Code:
echo.
echo   claude mcp add easy-peasy -- "%DIR%\%NAME%.exe" --root "%USERPROFILE%\projects"
echo.
echo See INSTALL.md for other clients and the config file.

rmdir /s /q "%TMPDIR%" 2>nul
endlocal
exit /b 0

:usage
echo Usage: install.bat [--dir ^<dir^>] [--build ^| --download] [--version ^<tag^>] [--no-path] [--help]
echo.
echo   --dir ^<dir^>       install directory (default: %%EASY_PEASY_INSTALL_DIR%% or
echo                     %%LOCALAPPDATA%%\Programs\easy-peasy-mcp)
echo   --build           build from the source checkout with cargo
echo   --download        download a release binary from GitHub
echo   --version ^<tag^>   release to download, e.g. v0.121.0 (default: the latest release)
echo   --no-path         don't add the install directory to the user PATH
echo.
echo Without --build or --download the script builds when it runs from a source checkout and cargo is installed,
echo and downloads otherwise.
endlocal
exit /b 0

:usage_error
call :usage
endlocal
exit /b 2

:fail
if defined TMPDIR rmdir /s /q "%TMPDIR%" 2>nul
endlocal
exit /b 1
