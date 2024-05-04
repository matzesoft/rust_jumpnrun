#!/bin/bash

APP_NAME="JumpNRun"
RUST_CRATE_NAME="game"

# create the folder structure
mkdir -p "target/${APP_NAME}.app/Contents/MacOS"
mkdir -p "target/${APP_NAME}.app/Contents/Resources"

# copy Info.plist
#cp Info.plist "${APP_NAME}.app/Contents/Info.plist"
# copy the icon (assuming you already have it in Apple ICNS format)
# cp AppIcon.icns "${APP_NAME}.app/Contents/Resources/AppIcon.icns"

# copy your Bevy game assets
cp -a assets "target/${APP_NAME}.app/Contents/MacOS/"

# compile the executables for each architecture
cargo build --release --target x86_64-apple-darwin # build for Intel
cargo build --release --target aarch64-apple-darwin # build for Apple Silicon

# combine the executables into a single file and put it in the bundle
lipo "target/x86_64-apple-darwin/release/${RUST_CRATE_NAME}" \
     "target/aarch64-apple-darwin/release/${RUST_CRATE_NAME}" \
     -create -output "target/${APP_NAME}.app/Contents/MacOS/${APP_NAME}"
