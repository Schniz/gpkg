# `gpkg`

> A work in progress

🌎 A consistent global binary manager for Node.js packages, built in Rust

## The idea

Install a binary from `npm`, while "statically linking" it to a specific Node version. So, if you installed [`qnm`](https://github.com/ranyitz/qnm) using Node 12 — you'll consistently call Node 12 when using it, even if you changed your Node version to 13, using [`fnm`](https://github.com/Schniz/fnm).
