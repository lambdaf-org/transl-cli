# transl

A CLI tool to sync and translate JSON‐based language files.

## Install

```bash
git clone <repo>
cd <repo>
cargo install --path .
```

Or grab a prebuilt `transl.exe` from Releases.

## Usage

```bash
transl [-h|--help] <header> <key> <value> <base_lang>
```

- `<header>`: JSON object to update  
- `<key>`: property name to insert  
- `<value>`: new value for base language  
- `<base_lang>`: two-letter code of source file (e.g. `en`)

## Build for Windows

```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```
Produced binary: `target/x86_64-pc-windows-gnu/release/transl.exe`

## License

MIT
