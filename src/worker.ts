import * as Comlink from "comlink";
import { simd } from "wasm-feature-detect";

type Mod =
  | typeof import("../pkg-simd/wasm_postflop.js")
  | typeof import("../pkg/wasm_postflop.js");

function createHandler(mod: Mod) {
  return {
    game: mod.GameManager.new(),

    init(
      flop: Uint8Array,
      startingPot: number,
      effectiveStack: number,
      oopFlopBetSizes: Float32Array,
      oopFlopRaiseSizes: Float32Array,
      oopTurnBetSizes: Float32Array,
      oopTurnRaiseSizes: Float32Array,
      oopRiverBetSizes: Float32Array,
      oopRiverRaiseSizes: Float32Array,
      ipFlopBetSizes: Float32Array,
      ipFlopRaiseSizes: Float32Array,
      ipTurnBetSizes: Float32Array,
      ipTurnRaiseSizes: Float32Array,
      ipRiverBetSizes: Float32Array,
      ipRiverRaiseSizes: Float32Array,
      addAllInThreshold: number,
      forceAllInThreshold: number,
      adjustLastTwoBetSizes: boolean
    ) {
      return this.game.init(
        flop,
        startingPot,
        effectiveStack,
        oopFlopBetSizes,
        oopFlopRaiseSizes,
        oopTurnBetSizes,
        oopTurnRaiseSizes,
        oopRiverBetSizes,
        oopRiverRaiseSizes,
        ipFlopBetSizes,
        ipFlopRaiseSizes,
        ipTurnBetSizes,
        ipTurnRaiseSizes,
        ipRiverBetSizes,
        ipRiverRaiseSizes,
        addAllInThreshold,
        forceAllInThreshold,
        adjustLastTwoBetSizes
      );
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
