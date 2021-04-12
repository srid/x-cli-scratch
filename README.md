# x

## Development (Flakes)

This repo uses [Flakes](https://nixos.wiki/wiki/Flakes) from the get-go, but compat is provided for traditional nix-shell/nix-build as well (see the section below).

```bash
# Dev shell
nix develop

# or just run directly
nix run

# or run via cargo
nix develop -c cargo run

# build
nix build
```

## Development (Legacy Nix)

```bash
# Dev shell
nix-shell

# run via cargo
nix-shell --run 'cargo run'

# build
nix-build
```
