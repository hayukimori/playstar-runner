# PlayStar Runner (Rust)

This is a joint project with [PlayStar](https://github.com/hayukimori/PlayStar).

playstar-runner is a binary for managing a single instance of PlayStar.


## How to build (from linux)

### Dependencies
- Rustup
- Cargo


**To Linux**
```sh
cargo build --release 
```

**To Windows**
```sh
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

## Usage

playstar-runner MUST stay on the same path as `PlayStar` (or `playstar.exe`) executable.

```sh
playstar-runner /path/to/song
```