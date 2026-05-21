<div align="center">
  <img src="./logo.svg" alt="Logo" width="128" height="128" />
  <h1>📫 Himalaya TUI</h1>
  <p>TUI to manage emails</p>
  <p>
    <a href="https://matrix.to/#/#pimalaya:matrix.org"><img alt="Matrix" src="https://img.shields.io/badge/chat-%23pimalaya-blue?style=flat&logo=matrix&logoColor=white"/></a>
    <a href="https://fosstodon.org/@pimalaya"><img alt="Mastodon" src="https://img.shields.io/badge/news-%40pimalaya-blue?style=flat&logo=mastodon&logoColor=white"/></a>
  </p>
</div>

> [!IMPORTANT]
> Himalaya TUI is in active development and currently shipped as `v0.0.1`. Expect breaking changes between releases; the CLI counterpart [pimalaya/himalaya](https://github.com/pimalaya/himalaya) remains the stable interface for production use.

## Table of contents

- [Features](#features)
- [Installation](#installation)
  - [Cargo](#cargo)
  - [Nix](#nix)
  - [Sources](#sources)
- [Configuration](#configuration)
- [Usage](#usage)
  - [Keybindings](#keybindings)
  - [Composing messages](#composing-messages)
- [Interfaces](#interfaces)
- [Social](#social)
- [Sponsoring](#sponsoring)

## Features

- **Three-pane layout** built on [ratatui](https://ratatui.rs): mailboxes, envelopes, message body or composer
- **In-app composer** powered by [edtui](https://crates.io/crates/edtui) with system-editor handoff (`Alt-e`)
- **Provider discovery wizard** shared with [himalaya](https://github.com/pimalaya/himalaya): PACC, Thunderbird Autoconfiguration, RFC 6186 SRV
- **Shared configuration file** with `himalaya`: same `[accounts.<name>]` blocks load on both binaries (see [Configuration](#configuration))
- **Protocol coverage** (via [io-email](https://github.com/pimalaya/io-email)):
  - **IMAP** <sup>[rfc9051](https://www.iana.org/go/rfc9051)</sup> (`imap` feature)
  - **JMAP** <sup>[rfc8620](https://www.iana.org/go/rfc8620), [rfc8621](https://www.iana.org/go/rfc8621)</sup> (`jmap` feature)
  - **Maildir** (`maildir` feature)
  - **SMTP** <sup>[rfc5321](https://www.iana.org/go/rfc5321)</sup> (`smtp` feature)
- **TLS** via [native-tls](https://crates.io/crates/native-tls) or [rustls](https://crates.io/crates/rustls) (AWS-LC or Ring crypto provider)
- **SASL** mechanisms: anonymous, login, plain, oauthbearer, xoauth2, scram-sha-256

*Himalaya TUI is written in [Rust](https://www.rust-lang.org/) and uses [cargo features](https://doc.rust-lang.org/cargo/reference/features.html) to gate backend support. The default feature set is declared in [`Cargo.toml`](./Cargo.toml).*

## Installation

### Cargo

```
cargo install himalaya-tui --locked
```

With only IMAP+SMTP support:

```
cargo install himalaya-tui --locked --no-default-features --features imap,smtp,rustls-ring
```

Pull the latest `master`:

```
cargo install --locked --git https://github.com/pimalaya/himalaya-tui.git
```

### Nix

If you have the [Flakes](https://nixos.wiki/wiki/Flakes) feature enabled:

```
nix profile install github:pimalaya/himalaya-tui
```

Or run without installing:

```
nix run github:pimalaya/himalaya-tui
```

### Sources

```
git clone https://github.com/pimalaya/himalaya-tui
cd himalaya-tui
nix develop --command cargo build --release
```

*The resulting binary lives at `target/release/himalaya-tui`.*

## Configuration

Run `himalaya-tui`. With no configuration file on disk the wizard prompts for an email address, a server URL or a bare domain, runs provider discovery, asks for SASL or HTTP credentials, then keeps the resulting account in memory for that session only (the TUI does not write to disk).

A persistent configuration is loaded from the first valid path among:

- `$XDG_CONFIG_HOME/himalaya/config.toml`
- `$HOME/.config/himalaya/config.toml`
- `$HOME/.himalayarc`

These are the same paths the [`himalaya`](https://github.com/pimalaya/himalaya) CLI looks at: one TOML file backs both binaries. TUI-only fields (`from`, `from-name`, `signature`, `signature-delim`) and CLI-only sections (`table`, `envelope`, `mailbox`, `message`, `attachment`) coexist without errors. See [`config.sample.toml`](./config.sample.toml) for a documented template.

Override the path with `-c <PATH>` or `HIMALAYA_CONFIG=<PATH>`; multiple paths can be passed at once, separated by `:`. The first one is the base and the rest are deep-merged on top.

Pass `--no-config` to ignore both, even when a file is present: useful for testing another account in memory without exposing stored credentials.

CLI flags (see `himalaya-tui --help`):

- `[ACCOUNT]`: account name when a config is loaded; otherwise a wizard seed (email, URL or domain)
- `-c, --config <PATH>`: override the default config file path (env: `HIMALAYA_CONFIG`)
- `--no-config`: skip on-disk config and run the wizard
- `--from <EMAIL>`: override the From address used when sending; also prefills the wizard's SASL/JMAP login
- `--from-name <NAME>`: override the From display name
- `--keybinds <vim|emacs>`: composer keybinding flavor (overrides the top-level `keybinds` TOML field; defaults to Vim)

## Usage

### Keybindings

Top-level navigation:

| Action | Universal | Vim flavor | Emacs flavor |
|---|---|---|---|
| Cycle panel | `Tab` | `Tab` | `Tab` |
| Next item | `↓` | `j` | `Ctrl-n` |
| Previous item | `↑` | `k` | `Ctrl-p` |
| Next page | `PageDown` | `Ctrl-d` | `Ctrl-v` |
| Previous page | `PageUp` | `Ctrl-u` | `Alt-v` |
| Select | `Enter` | `Enter` | `Enter` |
| Close panel / dialog / quit | `Esc` | `q` | `Ctrl-g` |
| Start a new draft | `Ctrl-c` | `Ctrl-c` | `Ctrl-c` |

By default, only the universal keys fire. Opt into a flavor with `--keybinds <vim|emacs>` (or the top-level `keybinds = "emacs"` TOML field) to enable the matching column as additive aliases.

Composer:

| Key | Action |
|---|---|
| `Ctrl-e` (Vim) / `Alt-e` | Hand off to `$VISUAL` or `$EDITOR` for the current draft |
| `Esc` | Open the compose actions dialog (Send, Preview, Save to Drafts, Cancel) |

Inside the composer, the chosen flavor drives [edtui](https://crates.io/crates/edtui)'s built-in keybindings (Vim normal/insert vs. Emacs insert-style). In Vim mode, `Ctrl-e` (edtui's normal-mode binding) opens the external editor; in Emacs mode, `Ctrl-e` is rebound to "move to end of line" and `Alt-e` is the only system-editor key.

Envelope dialog actions: Read, Reply, Reply All, Forward, Copy, Move, Add flag, Remove flag.

### Composing messages

Drafts are written in [MML](https://github.com/pimalaya/mml) and compiled to MIME on send. Headers (`From`, `To`, `Subject`...) live at the top of the buffer; the body and any MML directives (attachments, signing, encryption) follow.

Sending routes through SMTP when an `[accounts.<name>.smtp]` block is configured, otherwise through JMAP. Drafts can be saved to the `Drafts` mailbox at any time.

## Interfaces

Himalaya TUI is one of several front-ends to the Pimalaya libraries. See [pimalaya/himalaya#interfaces](https://github.com/pimalaya/himalaya#interfaces) for the full list (CLI, Vim, Emacs, Raycast).

## Social

- Chat on [Matrix](https://matrix.to/#/#pimalaya:matrix.org)
- News on [Mastodon](https://fosstodon.org/@pimalaya) or [RSS](https://fosstodon.org/@pimalaya.rss)
- Mail at [pimalaya.org@posteo.net](mailto:pimalaya.org@posteo.net)

## Sponsoring

[![nlnet](https://nlnet.nl/logo/banner-160x60.png)](https://nlnet.nl/)

Special thanks to the [NLnet foundation](https://nlnet.nl/) and the [European Commission](https://www.ngi.eu/) that have been financially supporting the project for years:

- 2022 → 2023: [NGI Assure](https://nlnet.nl/project/Himalaya/)
- 2023 → 2024: [NGI Zero Entrust](https://nlnet.nl/project/Pimalaya/)
- 2024 → 2026: [NGI Zero Core](https://nlnet.nl/project/Pimalaya-PIM/)
- *2027 in preparation...*

If you appreciate the project, feel free to donate using one of the following providers:

[![GitHub](https://img.shields.io/badge/-GitHub%20Sponsors-fafbfc?logo=GitHub%20Sponsors)](https://github.com/sponsors/soywod)
[![Ko-fi](https://img.shields.io/badge/-Ko--fi-ff5e5a?logo=Ko-fi&logoColor=ffffff)](https://ko-fi.com/soywod)
[![Buy Me a Coffee](https://img.shields.io/badge/-Buy%20Me%20a%20Coffee-ffdd00?logo=Buy%20Me%20A%20Coffee&logoColor=000000)](https://www.buymeacoffee.com/soywod)
[![Liberapay](https://img.shields.io/badge/-Liberapay-f6c915?logo=Liberapay&logoColor=222222)](https://liberapay.com/soywod)
[![PayPal](https://img.shields.io/badge/-PayPal-0079c1?logo=PayPal&logoColor=ffffff)](https://www.paypal.com/paypalme/soywod)
