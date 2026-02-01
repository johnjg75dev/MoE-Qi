@echo off
REM cd moeqi-wasm
wasm-pack build moeqi-wasm --release --target web
REM cd ..
copy /Y /B moeqi-wasm\pkg\ dist\