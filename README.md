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

## compatibility

tested on mac. probably works on linux. windo| ha-ha funny.
