## Rust stuffs

Add the riscv32i target to your rust toolchain

```bash
rustup target add riscv32i-unknown-none-elf
```

## Build

The project comes with the default cargo config, located in `.cargo/config.toml`

This saves us time from specifying the target every time we build

```bash
cargo build
```



# Freestanding Rust

Meaning must use no standard library

