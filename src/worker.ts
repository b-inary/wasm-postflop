import * as Comlink from "comlink";
import { detect } from "detect-browser";
import { InitOutput as InitOutputST } from "../pkg/solver-st/solver.js";
import { InitOutput as InitOutputMT } from "../pkg/solver-mt/solver.js";

type ModST = typeof import("../pkg/solver-st/solver.js");
type ModMT = typeof import("../pkg/solver-mt/solver.js");
type Mod = ModST | ModMT;

type ReadonlyBuffer = {
  ptr: number;
  byteLength: number;
};

function createHandler(mod: Mod) {
  return {
    game: mod.GameManager.new(),

    init(
      oopRange: Float32Array,
      ipRange: Float32Array,
      flop: Uint8Array,
      startingPot: number,
      effectiveStack: number,
      donkOption: boolean,
      oopFlopBet: string,
      oopFlopRaise: string,
      oopTurnBet: string,
      oopTurnRaise: string,
      oopTurnDonk: string,
      oopRiverBet: string,
      oopRiverRaise: string,
      oopRiverDonk: string,
      ipFlopBet: string,
      ipFlopRaise: string,
      ipTurnBet: string,
      ipTurnRaise: string,
      ipRiverBet: string,
      ipRiverRaise: string,
      addAllInThreshold: number,
      forceAllInThreshold: number,
      mergingThreshold: number,
      addedLines: string,
      removedLines: string
    ) {
      return this.game.init(
        oopRange,
        ipRange,
        flop,
        startingPot,
        effectiveStack,
        donkOption,
        oopFlopBet,
        oopFlopRaise,
        oopTurnBet,
        oopTurnRaise,
        oopTurnDonk,
        oopRiverBet,
        oopRiverRaise,
        oopRiverDonk,
        ipFlopBet,
        ipFlopRaise,
        ipTurnBet,
        ipTurnRaise,
        ipRiverBet,
        ipRiverRaise,
        addAllInThreshold,
        forceAllInThreshold,
        mergingThreshold,
        addedLines,
        removedLines
      );
    },

    privateCards(player: number) {
      const ret = this.game.private_cards(player);
      return {
        ptr: ret.pointer,
        byteLength: ret.byte_length,
      } as ReadonlyBuffer;
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

    applyHistory(history: Uint32Array) {
      this.game.apply_history(history);
    },

    actions() {
      return this.game.actions();
    },

    actionsAfterChance() {
      return this.game.actions_after_chance();
    },

    possibleCards() {
      return this.game.possible_cards();
    },

    equity(player: number): number {
      return this.game.equity(player);
    },
  };
}

type InitOutput = InitOutputST | InitOutputMT;
type Handler = ReturnType<typeof createHandler>;

let wasm: InitOutput;
let handler: Handler;

async function init(num_threads: number) {
  let mod: Mod;

  const browser = detect();
  if (browser && (browser.name === "safari" || browser.os === "iOS")) {
    mod = await import("../pkg/solver-st/solver.js");
    wasm = await mod.default();
  } else {
    mod = await import("../pkg/solver-mt/solver.js");
    wasm = await mod.default();
    await (mod as ModMT).initThreadPool(num_threads);
  }

  handler = createHandler(mod);
}

function getMemory() {
  return Comlink.proxy(wasm.memory);
}

function getHandler() {
  return Comlink.proxy(handler);
}

export interface WorkerApi {
  init: typeof init;
  getMemory: typeof getMemory;
  getHandler: typeof getHandler;
}

Comlink.expose({ init, getMemory, getHandler });
