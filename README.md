# transl-cli

> A Rust CLI that inserts a key into every `<lang>.json` file in the current directory, machine-translating the value for each non-base language.

![Rust](https://img.shields.io/badge/Rust-2024-orange?logo=rust&logoColor=white)

transl-cli keeps a set of JSON localization files in sync. It scans the working directory for files named after two-letter language codes (`en.json`, `de.json`, and so on), then adds one key to all of them in a single run. The value is taken verbatim for the base language and translated for the others using the `rust-translate` crate, which calls the Google Translate web endpoint. JSON is written back pretty-printed with the original key order preserved, since `serde_json` is built with the `preserve_order` feature.

The binary is named `transl`. It runs on Tokio (async) because the translation calls go over the network.

## Quickstart

```bash
git clone https://github.com/lambdaf-org/transl_cli
cd transl_cli
cargo install --path .
```

This places a `transl` binary on the cargo path. Run it from the directory that holds the language files:

```bash
transl <header> <key> <value> <base_lang>
```

For example, with `en.json` and `de.json` in the current directory:

```bash
transl greeting subtitle "Welcome aboard" en
```

This inserts `subtitle` under the `greeting` object in every language file. `en.json` gets the literal text `Welcome aboard`. `de.json` gets the German translation. The base language file (`en.json` here) must exist or the run aborts.

Building requires a recent Rust toolchain (edition 2024, tested with cargo 1.89). A network connection is needed at runtime for the translation calls.

### Building for Windows

```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

The binary lands at `target/x86_64-pc-windows-gnu/release/transl.exe`.

## Features

- **Bulk key insertion**: adds one `<key>` under one `<header>` object across every detected language file in a single invocation.
- **Automatic translation**: fills non-base languages with machine translations from the `rust-translate` crate (Google Translate web endpoint). The base value is copied as-is.
- **Language detection by filename**: treats any `<code>.json` whose stem is a supported two-letter language code as a target. Files it does not recognize are reported and skipped.
- **Order-preserving writes**: rewrites each file as pretty-printed JSON while keeping existing keys in their original positions.
- **Missing-header prompt**: when a file lacks the requested `<header>`, it asks at the terminal whether to abort the run or continue with the remaining files.

## Usage

```bash
transl [-h|--help] <header> <key> <value> <base_lang>
```

| Argument | Meaning |
| --- | --- |
| `<header>` | Name of the top-level JSON object to update. |
| `<key>` | Property name to insert under that object. |
| `<value>` | Text for the base language, translated for the others. |
| `<base_lang>` | Two-letter code of the source file (for example `en`). |

The tool exits with code `2` when the argument count is wrong, and `1` when the base language file is not present in the directory. Passing `-h` or `--help` prints the usage line and exits `0`. A failed translation call panics the process rather than returning a clean exit code.

## Contributing

See [lambdaf-org/contributing](https://github.com/lambdaf-org/contributing).

## License

MIT, per the previous README. There is no LICENSE file in the repo and `Cargo.toml` has no `license` field, so the MIT claim is not backed by a license text yet.