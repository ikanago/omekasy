# omekasy

[![CI](https://github.com/ikanago/rusty_boilerplate/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/ikanago/rusty_boilerplate/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![](https://img.shields.io/crates/v/omekasy.svg)](https://crates.io/crates/omekasy)

`omekasy` is a command line application that converts alphanumeric characters in your input to various styles defined in Unicode.
`omekasy` means "dress up" in Japanese.

## Installation
You can download binaries from [Releases](https://github.com/ikanago/omekasy/releases).

Or you can build it by yourself.
```bash
cargo install omekasy
```

## Usage
Just run without any options or arguments, a prompt will be shown up.
You can select the style while watching the result.
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
- monospace
- blackboard
