# Setup

```pip install aiogram ujson aiohttp[speedups]```

```pip install nuitka```

```npm install pm2@latest -g```

# Development

```pm2 start katyusha.py```

# Build

```nuitka --follow-imports --plugin-enable=multiprocessing --standalone --show-progress --show-scons --include-plugin-directory=utils katyusha.py```

# Usage

run ```katyusha.exe```