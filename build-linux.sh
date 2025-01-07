#!/bin/sh
docker build -t rust-build -f build-linux.Dockerfile .
docker run --rm --user "$(id -u)":"$(id -g)" \
  -v "$PWD":/usr/src -w /usr/src \
  rust-build \
  cargo build --release --target-dir target-docker

if [ $? -ne 0 ]; then
  echo "!!! BUILD FAILED !!!"
  exit
fi
