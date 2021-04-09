#!/usr/bin/env bash

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup toolchain install nightly
rustup override set nightly
rustup target add wasm32-unknown-unknown
rustup target add aarch64-linux-android