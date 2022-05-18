import { defineStore } from "pinia";

export type MainView =
  | "OOPRange"
  | "IPRange"
  | "Board"
  | "TreeConfig"
  | "RunSolver"
  | "Result";

function parseNumber(s: string): number {
  if (/^(\d+\.?\d*|\.\d+)$/.test(s)) {
    return Number(s);
  } else {
    return Number.NaN;
  }
}

function parseBetSizes(sizesStr: string): number[] | null {
  if (sizesStr === "") {
    return [];
  }

  const sizes = sizesStr.split(",").map((s) => parseNumber(s.trim()) / 100);

  if (sizes.some((s) => Number.isNaN(s) || s <= 0)) {
    return null;
  } else {
    return sizes;
  }
}

export const useStore = defineStore("app", {
  state: () => ({
    mainView: "Board" as MainView,
    board: [] as number[],
    startingPot: 180,
    effectiveStack: 710,
    oopFlopBetSizesStr: "33, 75",
    oopFlopRaiseSizesStr: "60",
    oopTurnBetSizesStr: "33, 75",
    oopTurnRaiseSizesStr: "60",
    oopRiverBetSizesStr: "33, 75",
    oopRiverRaiseSizesStr: "60",
    ipFlopBetSizesStr: "33, 75",
    ipFlopRaiseSizesStr: "60",
    ipTurnBetSizesStr: "33, 75",
    ipTurnRaiseSizesStr: "60",
    ipRiverBetSizesStr: "33, 75",
    ipRiverRaiseSizesStr: "60",
    addAllInThreshold: 120,
    forceAllInThreshold: 10,
    adjustLastTwoBetSizes: true,
    numThreads: navigator.hardwareConcurrency || 1,
    isTreeBuilt: false,
    isCompressionEnabled: false,
    memoryUsage: 0,
    memoryUsageCompressed: 0,
    terminateFlag: false,
    pauseFlag: false,
    isSolverRunning: false,
    isSolverPaused: false,
    isSolverTerminated: false,
    currentIteration: 0,
    exploitability: 0,
    expectedValue: [0, 0],
    elapsedTimeMs: 0,
  }),

  getters: {
    hasSolverRun: (state) => {
      return (
        state.isSolverRunning ||
        state.isSolverPaused ||
        state.isSolverTerminated
      );
    },

    oopFlopBetSizes: (state) => parseBetSizes(state.oopFlopBetSizesStr),
    oopFlopRaiseSizes: (state) => parseBetSizes(state.oopFlopRaiseSizesStr),
    oopTurnBetSizes: (state) => parseBetSizes(state.oopTurnBetSizesStr),
    oopTurnRaiseSizes: (state) => parseBetSizes(state.oopTurnRaiseSizesStr),
    oopRiverBetSizes: (state) => parseBetSizes(state.oopRiverBetSizesStr),
    oopRiverRaiseSizes: (state) => parseBetSizes(state.oopRiverRaiseSizesStr),
    ipFlopBetSizes: (state) => parseBetSizes(state.ipFlopBetSizesStr),
    ipFlopRaiseSizes: (state) => parseBetSizes(state.ipFlopRaiseSizesStr),
    ipTurnBetSizes: (state) => parseBetSizes(state.ipTurnBetSizesStr),
    ipTurnRaiseSizes: (state) => parseBetSizes(state.ipTurnRaiseSizesStr),
    ipRiverBetSizes: (state) => parseBetSizes(state.ipRiverBetSizesStr),
    ipRiverRaiseSizes: (state) => parseBetSizes(state.ipRiverRaiseSizesStr),
  },
});
