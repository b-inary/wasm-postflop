import * as Comlink from "comlink";
import { simd } from "wasm-feature-detect";

async function init() {
  let mod;
  if (await simd()) {
    mod = await import("./pkg-simd/wasm_postflop.js");
  } else {
    mod = await import("./pkg/wasm_postflop.js");
  }

  const num_threads = navigator.hardwareConcurrency || 1;

  await mod.default();
  await mod.initThreadPool(num_threads);

  return Comlink.proxy({
    game: null,
    init() {
      this.game = mod.GameManager.new();
    },
    get_err_string() {
      return this.game.get_err_string();
    },
    memory_usage() {
      return this.game.memory_usage();
    },
    allocate_memory() {
      this.game.allocate_memory();
    },
    iterate(iteration) {
      this.game.solve_step(iteration);
    },
    exploitability() {
      return this.game.exploitability();
    },
    normalize() {
      this.game.normalize();
    },
    ev(player) {
      return this.game.ev(player);
    },
  });
}

Comlink.expose({
  handler: init(),
});
