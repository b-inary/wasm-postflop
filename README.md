# WASM Postflop

**WASM Postflop** is a free and open-source GTO solver for Texas hold'em poker that works on web browsers.

Website: https://wasm-postflop.pages.dev/

Core solver repository: https://github.com/b-inary/postflop-solver

![Image](image.png)

## Why WASM Postflop?

The GTO (Game Theory Optimal) solver has become an indispensable tool for poker studies.
Unfortunately, however, there exists a high barrier to trying out the GTO solver: the need to purchase expensive commercial software.
This project aims to overcome this situation by developing a free and open-source GTO solver.

Please note that this project does not intend to *replace* commercial GTO solvers.
They are great software, and it is not easy to create a new one that can compete with them.
This project intends to make it easier for more people to use the GTO solver.

### Features

- **Free to use**.
  The most important feature.
  Anyone can try out the solver for free!

- **Open source**.
  The implementation of the GTO solver is complicated and is not easy to write down accurately.
  By making the program open source, we make it possible for anyone to examine the implementation.

- **Works on web browsers**.
  This feature brings several advantages.
  First, it allows for the solver to be easily accessible.
  Second, it naturally makes the solver a cross-platform application.
  Finally, it sandboxes the solver execution, so users do not have to worry about security.

- **Sufficiently fast**.
  Slow solvers are not wanted.
  By using WebAssembly, we have reduced the performance penalty of being a web application.
  We also supported multithreading and used a state-of-the-art algorithm ([Discounted CFR]).

[Discounted CFR]: https://arxiv.org/abs/1809.04040

## Build

```sh
$ # prerequisites
$ rustup install nightly
$ rustup component add rust-src --toolchain nightly
$ rustup target add wasm32-unknown-unknown
$ cargo install wasm-pack
$ npm install

$ # build
$ npm run wasm  # the latest version of binaryen (wasm-opt) may need to be installed
$ npm run build

$ # serve
$ npm run serve

$ # lint/format
$ npm run lint
$ npm run format
```

## License

Copyright (C) 2022 Wataru Inariba

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
