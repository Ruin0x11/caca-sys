# caca-sys
Low-level Rust bindings to `libcaca`.

## Usage
Ensure that the `libcaca` features you want are enabled in the `Cargo.toml`:
```
caca-sys = { version: "0.1.0", features = ["enable-x11", "enable-ncurses"] }
```
The following features are available:
- `enable-ncurses`
- `enable-slang`
- `enable-conio`
- `enable-x11`
- `enable-gl`
- `enable-win32`
- `enable-network`

## TODO:
- Conio
- Caca file I/O
- Character set conversion
- Process management
