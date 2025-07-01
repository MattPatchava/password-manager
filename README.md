# pwstore

**pwstore** is a CLI password manager written in Rust.

It encrypts your credentials using AES 256 GCM and stores them securely in a local file, with a master password protecting access.

___

## Features

* Secure password storage using AES-256-GCM
* Argon2-based key derivation from your master password
* CLI interface with subcommands for:
  * `add`: Encrypt and store new credentials
  * `show`: Decrypt and view stored credentials
  * `rm`: Delete a saved credential
  * `list`: List all stored usernames
* Local, offline storage in default config directory (`~/.config/pwstore` on Linux, `~/Library/Application Support/com.mattpatchava.pwstore` on macOS)

___

## Build

### Requirements

- Rust + Cargo installed: https://rustup.rs

### Setup

```bash
git clone https://github.com/MattPatchava/pwstore.git
cd pwstore
cargo build --release
```

### Run

```bash
cargo run -- add --username joe --password abc123
cargo run -- list
cargo run -- show -u joe
```

___

## Security Considerations

* The master password is never stored.
* A unique encryption key is derived using Argon2 from the master and a randomly-generated salt.
* Each credential is encrypted individually using AES-GCM.
* Decryption currently assumes the master password is correct — a verification mechanism is planned (see details below).

___

## Current Limitations

* If the wrong master password is used, decryption will fail or produce garbled output. A future iteration will add an encrypted verifier string to validate the master password on startup.
* No password expiry or clipboard integration features yet.

___

## Planned Improvements

* Add encrypted master password verifier
* Add unit and integration tests
* Add optional clipboard integration
* Migrate to SQLite
* Package for release