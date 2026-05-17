# nixq

A nix language data processing tool like jq for JSON, just for the nix language using jaq and rnix.

## Development (Flakes)

This repo uses [Flakes](https://nixos.wiki/wiki/flakes) from the get-go.

```bash
# Dev shell
nix develop

# or run via cargo
nix develop -c cargo run

# build
nix build
```

## Details

We also provide a [`justfile`](https://just.systems/) for Makefile'esque commands to be run inside of the devShell.

This project is `MIT` licensed.

This project was created by using the [rust-nix-template](https://github.com/srid/rust-nix-template).
