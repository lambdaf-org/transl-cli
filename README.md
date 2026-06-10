# transl

> Add one i18n key in your base language and auto-translate it into every sibling JSON file.

![Rust](https://img.shields.io/badge/Rust-2024-orange)

`transl` is a Rust CLI for keeping JSON i18n files in sync. You give it a header, a key, a value, and the base language. It writes the value into the base `<lang>.json`, then runs the same key through translation and inserts it into every other supported `<lang>.json` in the current directory.

## Install

```bash
git clone https://github.com/lambdaf-org/transl_cli.git
cd transl_cli
cargo install --path .
```

## Usage

```bash
transl <header> <key> <value> <base_lang>
```

- `<header>`: existing JSON object to update
- `<key>`: property name to insert
- `<value>`: value for the base language
- `<base_lang>`: two-letter code of the source file (e.g. `en`)

Run `transl -h` (or `--help`) to print usage. Run it from the directory that holds your `<lang>.json` files.

## Features

- **One command, all languages.** Insert a key once and it lands in every sibling language file.
- **Auto-translation.** Non-base languages get the value run through translation; the base language keeps your exact text.
- **Header-scoped writes.** The key is inserted into the named JSON object, so existing structure stays put.
- **Order preserved.** JSON is parsed and rewritten with key order intact (`serde_json` `preserve_order`).
- **Language auto-detection.** Files are matched by two-letter stem (e.g. `en.json`, `de.json`) against a supported-language list.
- **Interactive guard.** If a language file is missing the header, you get a prompt to continue or abort.

## How it works

`transl` scans the current directory, collects each `<lang>.json` whose two-letter stem is a supported language, and verifies the base language is present. For each file it loads the `<header>` object, computes the value (verbatim for the base language, translated for the rest), inserts `<key>`, and writes the file back as pretty-printed JSON.

## Build for Windows

```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

The binary lands at `target/x86_64-pc-windows-gnu/release/transl.exe`.

## Contributing

Lambdaforge is open source and contributions are welcome. Start with the [contributor guide](https://github.com/lambdaf-org/contributing), and see the org-wide [CONTRIBUTING](https://github.com/lambdaf-org/.github/blob/main/CONTRIBUTING.md) and [Code of Conduct](https://github.com/lambdaf-org/.github/blob/main/CODE_OF_CONDUCT.md).

## License

This repository does not yet include a `LICENSE` file, so default copyright applies for now. A license is coming soon. If you want to use or build on this before then, please open an issue.
