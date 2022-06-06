import { defineStore } from "pinia";

export type MainView =
  | "About"
  | "OOPRange"
  | "IPRange"
  | "Board"
  | "TreeConfig"
  | "RunSolver"
  | "Result";

const ranks = [
  "2",
  "3",
  "4",
  "5",
  "6",
  "7",
  "8",
  "9",
  "10",
  "J",
  "Q",
  "K",
  "A",
];

const suits = ["♣", "♦", "♥", "♠"];

const suitClasses = [
  "text-green-600",
  "text-blue-600",
  "text-pink-600",
  "text-black",
];

export function cardText(card: number) {
  return {
    rank: ranks[Math.floor(card / 4)],
    suit: suits[card % 4],
    colorClass: suitClasses[card % 4],
  };
}

function parseNumber(s: string): number {
  if (/^(\d+\.?\d*|\.\d+)$/.test(s)) {
    return Number(s);
  } else {
    return Number.NaN;
  }
}

function parseBetSizes(sizesStr: string): number[] | null {
  const trimmed = sizesStr.trim();
  if (trimmed === "") return [];

  const split = trimmed.split(sizesStr.includes(",") ? "," : /\s+/);
  const sizes = split.map((s) => parseNumber(s.trim()) / 100);

  if (sizes.some((s) => Number.isNaN(s) || s <= 0)) {
    return null;
  } else {
    return sizes;
  }
}

export function saveConfig() {
  const config = useConfigStore();
  const savedConfig = useSavedConfigStore();

  savedConfig.rangeRaw[0].set(config.rangeRaw[0]);
  savedConfig.rangeRaw[1].set(config.rangeRaw[1]);

  savedConfig.$patch({
    board: [...config.board],
    startingPot: config.startingPot,
    effectiveStack: config.effectiveStack,
    oopFlopBetSizes: config.oopFlopBetSizes ?? [],
    oopFlopRaiseSizes: config.oopFlopRaiseSizes ?? [],
    oopTurnBetSizes: config.oopTurnBetSizes ?? [],
    oopTurnRaiseSizes: config.oopTurnRaiseSizes ?? [],
    oopRiverBetSizes: config.oopRiverBetSizes ?? [],
    oopRiverRaiseSizes: config.oopRiverRaiseSizes ?? [],
    ipFlopBetSizes: config.ipFlopBetSizes ?? [],
    ipFlopRaiseSizes: config.ipFlopRaiseSizes ?? [],
    ipTurnBetSizes: config.ipTurnBetSizes ?? [],
    ipTurnRaiseSizes: config.ipTurnRaiseSizes ?? [],
    ipRiverBetSizes: config.ipRiverBetSizes ?? [],
    ipRiverRaiseSizes: config.ipRiverRaiseSizes ?? [],
    addAllInThreshold: config.addAllInThreshold,
    forceAllInThreshold: config.forceAllInThreshold,
    adjustLastTwoBetSizes: config.adjustLastTwoBetSizes,
  });
}

export const useStore = defineStore("app", {
  state: () => ({
    mainView: "About" as MainView,
    isSolverRunning: false,
    isSolverPaused: false,
    isSolverFinished: false,
    isFinalizing: false,
    normalizer: 0,
  }),

  getters: {
    hasSolverRun: (state) => {
      return (
        state.isSolverRunning ||
        state.isSolverPaused ||
        state.isSolverFinished ||
        state.isFinalizing
      );
    },
  },
});

export const useConfigStore = defineStore("config", {
  state: () => ({
    range: [
      Array.from({ length: 13 * 13 }, () => 0),
      Array.from({ length: 13 * 13 }, () => 0),
    ],
    rangeRaw: [
      Float32Array.from({ length: (52 * 51) / 2 }, () => 0),
      Float32Array.from({ length: (52 * 51) / 2 }, () => 0),
    ],
    board: [] as number[],
    startingPot: 180,
    effectiveStack: 910,
    oopFlopBetSizesStr: "50",
    oopFlopRaiseSizesStr: "60",
    oopTurnBetSizesStr: "60",
    oopTurnRaiseSizesStr: "60",
    oopRiverBetSizesStr: "70",
    oopRiverRaiseSizesStr: "60",
    ipFlopBetSizesStr: "50",
    ipFlopRaiseSizesStr: "60",
    ipTurnBetSizesStr: "60",
    ipTurnRaiseSizesStr: "60",
    ipRiverBetSizesStr: "70",
    ipRiverRaiseSizesStr: "60",
    addAllInThreshold: 120,
    forceAllInThreshold: 10,
    adjustLastTwoBetSizes: true,
  }),

  getters: {
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

export const useSavedConfigStore = defineStore("savedConfig", {
  state: () => ({
    rangeRaw: [
      Float32Array.from({ length: (52 * 51) / 2 }, () => 0),
      Float32Array.from({ length: (52 * 51) / 2 }, () => 0),
    ],
    board: [] as number[],
    startingPot: 0,
    effectiveStack: 0,
    oopFlopBetSizes: [] as number[],
    oopFlopRaiseSizes: [] as number[],
    oopTurnBetSizes: [] as number[],
    oopTurnRaiseSizes: [] as number[],
    oopRiverBetSizes: [] as number[],
    oopRiverRaiseSizes: [] as number[],
    ipFlopBetSizes: [] as number[],
    ipFlopRaiseSizes: [] as number[],
    ipTurnBetSizes: [] as number[],
    ipTurnRaiseSizes: [] as number[],
    ipRiverBetSizes: [] as number[],
    ipRiverRaiseSizes: [] as number[],
    addAllInThreshold: 0,
    forceAllInThreshold: 0,
    adjustLastTwoBetSizes: true,
  }),
});
