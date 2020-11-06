# Tomi(Web)

https://tim-patterson.github.io/tomi-web/

A port of https://github.com/daniellestead/tomi to run natively on the web using wasm.

All of the logic is done in rust but we do use the draw image methods of canvas as to take advantage of any GPU acceleration

## Prereqs.

```sh
cargo install wasm-pack
cargo install miniserve
```

## To build and run
```sh
./run.sh
```

## To deploy to github pages run
```sh
./deploy.sh
```  