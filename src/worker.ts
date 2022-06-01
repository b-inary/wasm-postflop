import * as Comlink from "comlink";
import { detect } from "detect-browser";

type ModST = typeof import("../pkg/solver-st/solver.js");
type ModMT = typeof import("../pkg/solver-mt/solver.js");
type Mod = ModST | ModMT;

function createHandler(mod: Mod) {
  return {
    game: mod.GameManager.new(),

    init(
      oopRange: Float32Array,
      ipRange: Float32Array,
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
        oopRange,
        ipRange,
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

    privateHandCards(player: number) {
      return this.game.private_hand_cards(player);
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

    finalize() {
      return this.game.finalize();
    },

    applyHistory(history: Int32Array) {
      this.game.apply_history(history);
    },

    getActions() {
      return this.game.get_actions();
    },

    isTerminal() {
      return this.game.is_terminal();
    },

    isPossible() {
      return this.game.is_possible();
    },

    currentPlayer() {
      return this.game.current_player();
    },

    getWeights() {
      return this.game.get_weights();
    },

    getNormalizedWeights() {
      return this.game.get_normalized_weights();
    },

    getExpectedValues() {
      return this.game.get_expected_values();
    },

    getStrategy() {
      return this.game.get_strategy();
    },
  };
}

type Handler = ReturnType<typeof createHandler>;

let handler: Handler;

async function init(num_threads: number) {
  let mod: Mod;

  const browser = detect();
  if (browser && (browser.name === "safari" || browser.os === "iOS")) {
    mod = await import("../pkg/solver-st/solver.js");
    await mod.default();
  } else {
    mod = await import("../pkg/solver-mt/solver.js");
    await mod.default();
    await (mod as ModMT).initThreadPool(num_threads);
  }

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
