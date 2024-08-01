# Markdown viewer

Simple live markdown viewer using pandoc and tauri. This does not open a separate browser
and handles live updates using the nvim api.

## Requirements

- Node
- Rust
- Pandoc
- Bun
- Tauri

Can be installed using nix flakes, not currently working on Mac M1

```bash
nix develop
```

## Usage
```bash
nvim --listen 127.0.0.1 (TODO: Automatically call binary in nvim plugin)
bun run tauri dev
```

```bash
bun run tauri dev
```

# Bugs and limitations

- [ ] Only handles `buf_lines_event` and no others
- [ ] Must have a file open in nvim before starting the tauri app
- [ ] Does not reattach to new buffers or disconnect
- [ ] No vim plugin to automatically start the tauri app

# Acknowledgements

- [Illumination](https://github.com/oknozor/illumination) for the approach and idea
