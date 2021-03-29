# :information_source: EDUCATIONAL PURPOSES ONLY

### Development Setup

```pip install -r katpy/requirements.txt```

```npm install pm2@latest -g```

```pm2 start katpy/katyusha.py```

```cd katrust && cargo build```

### Build on Python

```pyinstaller --onefile --windowed --icon=katyusha.ico katpy/katyusha.py```

### Build on Rust
```cd katrust && cargo run --bin katyusha --release```

### Usage

Spread ```katyusha.exe``` inside `katpy/dist` or ```katrust.exe``` inside `katrust/target/release` folder.

## 📌 TODOs
* Add Docker Setup and Usage
