# wasm-postflop

A free and open-source GTO solver that works on web browsers

Website: https://wasm-postflop.pages.dev/

Core solver repository: https://github.com/b-inary/postflop-solver

![Image](image.png)

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
