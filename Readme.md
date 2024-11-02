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


# Benchamrk

### Python

> python .\src\main.py examples/scene4.yaml
```
Function=render, Time=61.89826536178589
```

### Rust

> cargo run .\examples\scene4.yaml
```
Rendering: 99.9%
Rendering completed in 1.55s
Image saved to renders/s4-two-ball.png
```

Scene 5 
```
Python:
TotalMilliseconds : 168074.935

Single thread:
TotalMilliseconds : 6691.2546

Multi thread:
TotalMilliseconds : 3830.298
```