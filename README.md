# wasm-postflop

Post-flop solver powered by WebAssembly (work in progress)

## Build

```sh
$ # prerequisites
$ rustup install nightly
$ rustup target add wasm32-unknown-unknown
$ cargo install wasm-pack
$ npm install

$ # build
$ npm run wasm
$ npm run build

$ # serve
$ npm run serve

$ # lint/format
$ npm run lint
$ npm run format
```
