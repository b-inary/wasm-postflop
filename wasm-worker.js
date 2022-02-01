import * as Comlink from "comlink";

async function init() {
  const mod = await import("./pkg/wasm_postflop.js");
  const num_threads = Math.min(navigator.hardwareConcurrency || 1, 6);
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
