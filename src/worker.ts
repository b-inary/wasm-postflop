import * as Comlink from "comlink";
import { simd } from "wasm-feature-detect";

type Mod =
  | typeof import("../pkg-simd/wasm_postflop.js")
  | typeof import("../pkg/wasm_postflop.js");

function createHandler(mod: Mod) {
  return {
    game: mod.GameManager.new(),

    init(flop: Uint8Array) {
      return this.game.init(flop);
    },

    memoryUsage(enable_compression: boolean) {
      return Number(this.game.memory_usage(enable_compression));
    },

    allocateMemory(enable_compression: boolean) {
      this.game.allocate_memory(enable_compression);
    },

    iterate(iteration: number) {
      this.game.solve_step(iteration);
    },

    exploitability() {
      return this.game.exploitability();
    },

    normalize() {
      this.game.normalize();
    },

    ev(player: number) {
      return this.game.ev(player);
    },
  };
}

type Handler = ReturnType<typeof createHandler>;

let handler: Handler;

async function init(num_threads: number) {
  let mod: Mod;

  if (await simd()) {
    mod = await import("../pkg-simd/wasm_postflop.js");
  } else {
    mod = await import("../pkg/wasm_postflop.js");
  }

  await mod.default();
  await mod.initThreadPool(num_threads);

  handler = createHandler(mod);
}

function getHandler() {
  return Comlink.proxy(handler);
}

export interface WorkerApi {
  init: typeof init;
  getHandler: typeof getHandler;
}

Comlink.expose({ init, getHandler });
