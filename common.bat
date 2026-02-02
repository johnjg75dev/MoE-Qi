@echo off
setlocal EnableExtensions EnableDelayedExpansion

rem All-in-one command runner for this repo.
rem Run from anywhere: common.bat <command> [args...]

set "ROOT=%~dp0"
pushd "%ROOT%" >nul

set "CMD=%~1"
if "%CMD%"=="" goto :help
shift

set "ARGS="
:collect_args
if "%~1"=="" goto :args_done
set "ARGS=!ARGS! %1"
shift
goto :collect_args
:args_done

if /I "%CMD%"=="help" goto :help
if /I "%CMD%"=="build" (call :cmd_build & goto :ret)
if /I "%CMD%"=="release" (call :cmd_release & goto :ret)
if /I "%CMD%"=="check" (call :cmd_check & goto :ret)
if /I "%CMD%"=="test" (call :cmd_test & goto :ret)
if /I "%CMD%"=="doc" (call :cmd_doc & goto :ret)
if /I "%CMD%"=="doc-open" (call :cmd_doc_open & goto :ret)
if /I "%CMD%"=="fmt" (call :cmd_fmt & goto :ret)
if /I "%CMD%"=="clippy" (call :cmd_clippy & goto :ret)
if /I "%CMD%"=="clean" (call :cmd_clean & goto :ret)
if /I "%CMD%"=="dll" (call :cmd_dll & goto :ret)
if /I "%CMD%"=="wasm" (call :cmd_wasm & goto :ret)
if /I "%CMD%"=="dist" (call :cmd_dist & goto :ret)
if /I "%CMD%"=="cargo" (call :cmd_cargo & goto :ret)

echo Unknown command: %CMD%
echo.
goto :help

:ret
if errorlevel 1 goto :fail
goto :ok

:help
echo Usage:
echo   common.bat ^<command^> [args...]
echo.
echo Common commands:
echo   build        cargo build --workspace
echo   release      cargo build --workspace --release
echo   check        cargo check --workspace
echo   test         cargo test --workspace
echo   doc          cargo doc --workspace --no-deps
echo   doc-open     open target\doc\moeqi\index.html
echo   fmt          cargo fmt --all
echo   clippy       cargo clippy --workspace --all-targets -- -D warnings
echo   clean        cargo clean
echo.
echo Artifacts:
echo   dll          build moeqi-ffi release + copy outputs to dist\
echo   wasm         wasm-pack build moeqi-wasm + copy pkg to dist\
echo   dist         dll + wasm
echo.
echo Passthrough:
echo   cargo ...    runs cargo with remaining args
echo.
goto :ok

:ensure_dist
if not exist "dist\" mkdir "dist" >nul 2>nul
exit /b 0

:cmd_build
cargo build --workspace !ARGS!
exit /b %ERRORLEVEL%

:cmd_release
cargo build --workspace --release !ARGS!
exit /b %ERRORLEVEL%

:cmd_check
cargo check --workspace !ARGS!
exit /b %ERRORLEVEL%

:cmd_test
cargo test --workspace !ARGS!
exit /b %ERRORLEVEL%

:cmd_doc
cargo doc --workspace --no-deps !ARGS!
exit /b %ERRORLEVEL%

:cmd_doc_open
if not exist "target\\doc\\moeqi\\index.html" (
  call :cmd_doc
  if errorlevel 1 exit /b 1
)
start "" "target\\doc\\moeqi\\index.html"
exit /b 0

:cmd_fmt
cargo fmt --all
exit /b %ERRORLEVEL%

:cmd_clippy
cargo clippy --workspace --all-targets -- -D warnings
exit /b %ERRORLEVEL%

:cmd_clean
cargo clean
exit /b %ERRORLEVEL%

:cmd_dll
call :ensure_dist
cargo build -p moeqi-ffi --release !ARGS!
if errorlevel 1 exit /b 1

set "DLL=target\\release\\moeqi_ffi.dll"
if not exist "!DLL!" (
  echo Expected DLL not found: !DLL!
  exit /b 1
)

copy /Y /B "!DLL!" "dist\\" >nul
if exist "target\\release\\moeqi_ffi.lib" copy /Y /B "target\\release\\moeqi_ffi.lib" "dist\\" >nul
if exist "target\\release\\moeqi_ffi.pdb" copy /Y /B "target\\release\\moeqi_ffi.pdb" "dist\\" >nul
if exist "target\\release\\moeqi_ffi.dll.a" copy /Y /B "target\\release\\moeqi_ffi.dll.a" "dist\\" >nul
exit /b 0

:cmd_wasm
call :ensure_dist
where wasm-pack >nul 2>nul
if errorlevel 1 (
  echo wasm-pack not found in PATH. Install it to use this command.
  echo   cargo install wasm-pack
  exit /b 1
)

wasm-pack build moeqi-wasm --release --target web !ARGS!
if errorlevel 1 exit /b 1

if not exist "moeqi-wasm\\pkg\\" (
  echo Expected output folder not found: moeqi-wasm\\pkg\\
  exit /b 1
)

rem robocopy returns non-zero for "success with copies"; treat 0-7 as OK.
robocopy "moeqi-wasm\\pkg" "dist" /E /NFL /NDL /NJH /NJS /NC /NS /NP >nul
if %ERRORLEVEL% GEQ 8 exit /b 1
exit /b 0

:cmd_dist
set "SAVED_ARGS=!ARGS!"
call :cmd_dll
if errorlevel 1 exit /b 1
set "ARGS="
call :cmd_wasm
set "ARGS=!SAVED_ARGS!"
if errorlevel 1 exit /b 1
exit /b 0

:cmd_cargo
cargo !ARGS!
exit /b %ERRORLEVEL%

:fail
popd >nul
exit /b 1

:ok
popd >nul
exit /b 0
