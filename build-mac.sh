#!/bin/bash
# shellcheck disable=SC2046
eval export $(cat .env)

TARGET=csvs
cargo build --release --target-dir target-mac --target=x86_64-apple-darwin
cargo build --release --target-dir target-mac --target=aarch64-apple-darwin
mkdir -p ./dist-mac
lipo -create -output ./dist-mac/$TARGET ./target-mac/x86_64-apple-darwin/release/$TARGET ./target-mac/aarch64-apple-darwin/release/$TARGET

if [ -n "${IDENTITY+1}" ]; then
  codesign -f -s "$IDENTITY" ./dist-mac/$TARGET
else
  echo "You need to define an env variable IDENTITY=\"Apple Distribution: *****\" in .env file to codesign the executable binary"
fi
