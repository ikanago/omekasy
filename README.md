# omekasy

[![CI](https://github.com/ikanago/rusty_boilerplate/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/ikanago/rusty_boilerplate/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![](https://img.shields.io/crates/v/omekasy.svg)](https://crates.io/crates/omekasy)

[![asciicast](https://asciinema.org/a/490055.svg)](https://asciinema.org/a/490055)

`omekasy` is a command line application that converts alphanumeric characters in your input to various styles defined in Unicode.
`omekasy` means "dress up" in Japanese.

## Installation
### Homebrew
Supports macOS(x86_64, aarch64) and Linux(x86_64).
```bash
brew install ikanago/tap/omekasy
```

### Cargo
```bash
cargo install omekasy
```

### Binaries
You can download binaries from [Releases](https://github.com/ikanago/omekasy/releases).

## Usage
Just run without any options or arguments, then a prompt will be shown up.
You can select the style while watching the result like above demo.
```bash
omekasy
```

To convert to a specific font instantly, give the font name and input.
```bash
omekasy --font bold-italic "My new gear..."
```
Characters other than latin alphabets and numbers in your input remain untouched.

Available font for now:
- bold
- italic
- bold-italic
- sans
- sans-bold
- sans-italic
- sans-bold
- italic
- script
- bold-script
- fraktur
- bold-fraktur
- monospace
- blackboard
- emoji

Key bindings in interactive mode:
| Key          | Action           |
| ------------ | ---------------- |
| Up, Ctrl-K   | Move cursor up   |
| Down, Ctrl-J | Move cursor down |
| Enter        | Select           |
| Ctrl-C, Esc  | Quit             |
