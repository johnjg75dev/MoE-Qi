@echo off
cd moeqi-ffi
cargo build -p moeqi-ffi --release
cd ..
copy /B moeqi-ffi\target\release\moeqi_ffi.dll dist\