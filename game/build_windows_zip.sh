#!/bin/bash

APP_NAME="JumpNRun"
RUST_CRATE_NAME="game"
OUTPUT_PATH="target/output"

# create the folder structure
mkdir -p "${OUTPUT_PATH}/${APP_NAME}"
mkdir -p "${OUTPUT_PATH}/${APP_NAME}/assets"

# compile the executable
cargo build --release --target x86_64-pc-windows-gnu

# copy the executable
cp -a "target/x86_64-pc-windows-gnu/release/${RUST_CRATE_NAME}.exe" "${OUTPUT_PATH}/${APP_NAME}/"

# copy your Bevy game assets
cp -a assets "${OUTPUT_PATH}/${APP_NAME}/"

# zip the folder and remove it
cd "${OUTPUT_PATH}"
zip -r "${APP_NAME}.zip" "${APP_NAME}"
rm -rf "${APP_NAME}"