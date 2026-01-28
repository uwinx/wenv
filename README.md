# wenv

load env files and run stuff

## install

```
cargo install --path .
```

## usage

```
wenv .env .other.env -- python -c 'print(__import__("os").environ)'
```

later files override earlier ones

## memory

wenv remembers which env files you used in each directory. next time just run:

```
wenv -- python main.py
```

it'll use the same files as last time

## config

lives at:

- mac: `~/Library/Application Support/wenv/`
- linux: `~/.config/wenv/`

`config.toml`:

```toml
[memory]
enabled = true
max_entries = 10
```

`memory.toml` stores your history (edit if you want)

## aliases

define shortcuts in `.wenv.toml`:

```toml
[aliases]
dev = [".env", ".env.dev", ".env.local"]
prod = [".env", ".env.prod"]
```

then:

```
wenv @dev -- python main.py
```

aliases aren't memorized - they're already shortcuts

## per-project config

`.wenv.toml` can also disable memory:

```toml
[memory]
enabled = false
```

## watch mode

rerun on env file changes:

```
wenv -w .env -- python main.py
```

## compatibility

tested on mac. probably works on linux. windo| ha-ha funny.
