### Rust Setup

#### Install Rust

First, install Rust using rustup (the official Rust installer):
- Linux/macOS: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Windows: Download and run [rustup-init.exe](https://rustup.rs)

#### Create new Rust project
```
cargo new rust-raytracer
cd rust-raytracer
```

#### Build and run
```
cargo build
cargo run
```

#### Run tests
```
cargo test
```

# Render scene

```
cargo run -- examples/scene1.yaml
```
