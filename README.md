# WASM Postflop

**WASM Postflop** is a free, open-source GTO solver for Texas hold'em poker that works on web browsers.

Website: https://wasm-postflop.pages.dev/

Desktop app **(new!)**: https://github.com/b-inary/desktop-postflop

Solver engine repository: https://github.com/b-inary/postflop-solver

![Image](image.png)

## Why WASM Postflop?

The GTO (Game Theory Optimal) solver has become an indispensable tool for poker research.
However, unfortunately, there is a high barrier to trying out the GTO solver: the need to purchase expensive commercial software.
This project aims to overcome this situation by developing a free, open-source GTO solver.

Please note that this project does not intend to *replace* commercial GTO solvers.
They are great software, and it is not easy to create a new one that can compete with them.
This project intends to make the GTO solver more easily accessible to a broader audience.

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

## Comparison

We tested WASM Postflop, [PioSOLVER Free] (2.0.8), [GTO+] (v1.4.1), and [Desktop Postflop] with the "3betpotFAST" preset of PioSOLVER (all-in threshold is replaced with 100% in PioSOLVER).

[PioSOLVER Free]: https://www.piosolver.com/
[GTO+]: https://www.gtoplus.com/
[Desktop Postflop]: https://github.com/b-inary/desktop-postflop

### Execution time and memory usage

We experimented on a Windows 10 PC with a Ryzen 7 3700X CPU (16 threads; PioSOLVER Free is limited to 6 threads).
WASM Postflop was executed on Google Chrome 103.

Pio CFR, GTO+, and Desktop Postflop had similar execution times.
WASM Postflop was about 2x slower than these implementations.
We consider that 2x overhead is acceptable for casual use.
However, if you do not think so, please consider trying Desktop Postflop.

[1]: No compression / [2]: Use compression / [3]: Pio CFR / [4]: Original Pio algorithm

- **6 threads**

| Solver | WASM<br/>[1] | WASM<br/>[2] | PioSOLVER<br/>2.0.8 [3] | PioSOLVER<br/>2.0.8 [4] | GTO+<br/>v1.4.1 | Desktop<br/>[1] | Desktop<br/>[2] |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **Time (Target = 0.5%)** | **52.0 s** | **62.9 s** | 23.4 s | 30.4 s | *22.0 s* | 25.6 s | 28.7 s |
| **Time (Target = 0.3%)** | **64.8 s** | **77.6 s** | *28.7 s* | 42.5 s | 31.4 s | 31.3 s | 35.9 s |
| **Time (Target = 0.1%)** | **116.7 s** | **137.0 s** | 61.1 s | 108.9 s | 67.7 s | *56.9 s* | 63.9 s |
| **Memory usage** | **1.20 GB** | **631 MB** | 1.41 GB | 634 MB | 705 MB | 1.22 GB | 651 MB |

- **16 threads**

| Solver | WASM<br/>[1] | WASM<br/>[2] | GTO+<br/>v1.4.1 | Desktop<br/>[1] | Desktop<br/>[2] |
| :---: | :---: | :---: | :---: | :---: | :---: |
| **Time (Target = 0.5%)** | **31.9 s** | **37.9 s** | *13.9 s* | 16.1 s | 17.9 s |
| **Time (Target = 0.3%)** | **39.5 s** | **47.0 s** | *19.7 s* | 20.1 s | 22.2 s |
| **Time (Target = 0.1%)** | **70.2 s** | **84.0 s** | 41.7 s | *35.7 s* | 39.5 s |
| **Memory usage** | **1.21 GB** | **640 MB** | 705 MB | 1.22 GB | 659 MB |

### Results

A comparison of the obtained results is as follows (target exploitability is set to 0.1%).
We can see that WASM Postflop, PioSOLVER, and GTO+ return nearly identical results.

| WASM Postflop | PioSOLVER | GTO+ |
| --- | --- | --- |
| ![WASM Postflop results](comparison_wasm.png) | ![PioSOLVER results](comparison_pio.png) | ![GTO+ results](comparison_gtoplus.png) |

**Summary for those who do not want to compare images:**

| Solver | WASM | PioSOLVER | GTO+ |
| :---: | :---: | :---: | :---: |
| **Bet %** | **55.2%** | 55.19% | 55.2% |
| **Equity** | **55.3%** | 55.347% | 55.35% |
| **EV** | **105.1** | 105.11 | 105.115 |

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
