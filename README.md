# Setup

```conda activate katyusha```

```pip install aiogram ujson aiohttp[speedups]```

```npm install pm2@latest -g```

```conda install -c conda-forge nuitka```

# Development

```pm2 start katyusha.py```

# Build

```nuitka --follow-imports --plugin-enable=multiprocessing --standalone --show-progress --show-scons --include-plugin-directory=utils katyusha.py```

# Usage

run ```katyusha.exe```