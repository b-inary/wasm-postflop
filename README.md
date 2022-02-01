# wasm-postflop

Post-flop solver powered by WebAssembly (under development)

## Build

```sh
$ # prerequisites
$ rustup default nightly
$ rustup target add wasm32-unknown-unknown
$ cargo install wasm-pack

$ # build
$ wasm-pack build --target web
$ npm run build

$ # serve
$ npm run serve
```
