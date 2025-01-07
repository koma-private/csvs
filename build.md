# Build

- [Prerequisite](#prerequisite)
- [With `cargo`](#with-cargo)
- [Build from source codes](#build-from-source-codes)
    - [Windows (MSVC)](#windows-msvc)
    - [macOS (Universal Binary)](#macos-universal-binary)
    - [Linux (musl libc)](#linux-musl-libc)

## Prerequisite

You must have [Rust program language](https://www.rust-lang.org/tools/install) installed.

## With `Cargo`

To install **csvs** directly from crates.io:

```shell
cargo install csvs
```

## From source codes

### Windows (MSVC)

1. Set up a [Rust build environment on Windows](https://learn.microsoft.com/ja-jp/windows/dev-environment/rust/setup).
2. Install the Rust toolchain `stable-x86_64-pc-windows-msvc`.
3. Run the build script:

```shell
build-win.cmd
```

4. The Executable will be generated in `target-win/release`.

### macOS (Universal Binary)

1. Install Xcode command-line tools:

```shell
xcode-select --install
```

2. Install the Rust targets `x86_64-apple-darwin` and `aarch64-apple-darwin`.
3. Run the build script:

```shell
./build-mac.sh
```

4. The executable will be generated in `dist-mac`

### Linux (musl libc)

1. Install Docker. Refer to the [Docker Docs](https://docs.docker.com/engine/install/).
2. Run the build script:

```shell
./build-linux.sh
```

3. The executable will be generated in `target-docker/release`.
