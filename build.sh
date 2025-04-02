#!/bin/bash

RUSTFLAGS="-Zlocation-detail=none" cargo build --release
RUSTFLAGS="-Zlocation-detail=none" cargo build --target=x86_64-unknown-linux-musl --release
RUSTFLAGS="-Zlocation-detail=none" cargo build --target x86_64-pc-windows-gnu --release --verbose
upx --best --lzma target/release/rainbow
upx --best --lzma target/x86_64-unknown-linux-musl/release/rainbow
upx --best --lzma target/x86_64-pc-windows-gnu/release/rainbow.exe
