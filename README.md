# `gpkg`

> 🌎 A consistent global binary manager for Node.js packages, built in Rust

🔒 Lock the correct Node.js version for binaries
💎 Works with [fnm](https://github.com/Schniz/fnm) and nvm and any other Node.js version manager
✨ Single file, easy installation

## Installation

1. Download the latest binary into your `$PATH`
2. Add `~/.gpkg/bin` to your `$PATH` environment variable
3. Smile, you're done! 😺

## Usage

* `gpkg install <package>`: install a package and its binaries
* `gpkg uninstall <package>`: uninstall a package and its binaries
* `gpkg list`: list all installed binaries, their packages and node versions
* `gpkg completions <shell>`: generate shell completions for `gpkg`

## 💡 The idea

Install a binary from `npm`, while "statically linking" it to a specific Node version. So, if you installed [`qnm`](https://github.com/ranyitz/qnm) using Node 12 — you'll consistently call Node 12 when using it, even if you changed your Node version to 13, using [`fnm`](https://github.com/Schniz/fnm).
