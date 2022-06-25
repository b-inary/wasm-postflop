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
  const numberRegex = /^(\d+\.?\d*|\.\d+)$/;
  if (s.endsWith("x")) {
    if (numberRegex.test(s.slice(0, -1))) {
      return -Number(s.slice(0, -1));
    } else {
      return Number.NaN;
    }
  } else {
    if (numberRegex.test(s)) {
      return Number(s) / 100;
    } else {
      return Number.NaN;
    }
  }
}

function parseBetSizes(
  sizesStr: string,
  allowRelative: boolean
): number[] | null {
  const trimmed = sizesStr.trim();
  if (trimmed === "") return [];

  const split = trimmed.split(sizesStr.includes(",") ? "," : /\s+/);
  const sizes = split.map((s) => parseNumber(s.trim()));

  if (
    sizes.some(
      (s) =>
        Number.isNaN(s) ||
        s > 100 ||
        s < -100 ||
        (s < 0 && (!allowRelative || -1 <= s))
    )
  ) {
    return null;
  } else {
    return sizes;
  }
}

export function saveConfigTmp() {
  const config = useConfigStore();
  const tmpConfig = useTmpConfigStore();

  tmpConfig.rangeRaw[0].set(config.rangeRaw[0]);
  tmpConfig.rangeRaw[1].set(config.rangeRaw[1]);

  tmpConfig.$patch({
    board: [...config.board],
    startingPot: config.startingPot,
    effectiveStack: config.effectiveStack,
    oopFlopBet: config.oopFlopBet ?? [],
    oopFlopRaise: config.oopFlopRaise ?? [],
    oopTurnBet: config.oopTurnBet ?? [],
    oopTurnRaise: config.oopTurnRaise ?? [],
    oopRiverBet: config.oopRiverBet ?? [],
    oopRiverRaise: config.oopRiverRaise ?? [],
    ipFlopBet: config.ipFlopBet ?? [],
    ipFlopRaise: config.ipFlopRaise ?? [],
    ipTurnBet: config.ipTurnBet ?? [],
    ipTurnRaise: config.ipTurnRaise ?? [],
    ipRiverBet: config.ipRiverBet ?? [],
    ipRiverRaise: config.ipRiverRaise ?? [],
    addAllInThreshold: config.addAllInThreshold,
    forceAllInThreshold: config.forceAllInThreshold,
    adjustLastTwoBetSizes: config.adjustLastTwoBetSizes,
  });
}

export function saveConfig() {
  const tmpConfig = useTmpConfigStore();
  const savedConfig = useSavedConfigStore();

  savedConfig.rangeRaw[0].set(tmpConfig.rangeRaw[0]);
  savedConfig.rangeRaw[1].set(tmpConfig.rangeRaw[1]);

  savedConfig.$patch({
    board: tmpConfig.board,
    startingPot: tmpConfig.startingPot,
    effectiveStack: tmpConfig.effectiveStack,
    oopFlopBet: tmpConfig.oopFlopBet,
    oopFlopRaise: tmpConfig.oopFlopRaise,
    oopTurnBet: tmpConfig.oopTurnBet,
    oopTurnRaise: tmpConfig.oopTurnRaise,
    oopRiverBet: tmpConfig.oopRiverBet,
    oopRiverRaise: tmpConfig.oopRiverRaise,
    ipFlopBet: tmpConfig.ipFlopBet,
    ipFlopRaise: tmpConfig.ipFlopRaise,
    ipTurnBet: tmpConfig.ipTurnBet,
    ipTurnRaise: tmpConfig.ipTurnRaise,
    ipRiverBet: tmpConfig.ipRiverBet,
    ipRiverRaise: tmpConfig.ipRiverRaise,
    addAllInThreshold: tmpConfig.addAllInThreshold,
    forceAllInThreshold: tmpConfig.forceAllInThreshold,
    adjustLastTwoBetSizes: tmpConfig.adjustLastTwoBetSizes,
  });
}

export const useStore = defineStore("app", {
  state: () => ({
    mainView: "About" as MainView,
    isSolverRunning: false,
    isSolverPaused: false,
    isSolverFinished: false,
    isFinalizing: false,
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
    startingPot: 0,
    effectiveStack: 0,
    oopFlopBetStr: "",
    oopFlopRaiseStr: "",
    oopTurnBetStr: "",
    oopTurnRaiseStr: "",
    oopRiverBetStr: "",
    oopRiverRaiseStr: "",
    ipFlopBetStr: "",
    ipFlopRaiseStr: "",
    ipTurnBetStr: "",
    ipTurnRaiseStr: "",
    ipRiverBetStr: "",
    ipRiverRaiseStr: "",
    addAllInThreshold: 0,
    forceAllInThreshold: 0,
    adjustLastTwoBetSizes: true,
  }),

  getters: {
    oopFlopBet: (state) => parseBetSizes(state.oopFlopBetStr, false),
    oopFlopRaise: (state) => parseBetSizes(state.oopFlopRaiseStr, true),
    oopTurnBet: (state) => parseBetSizes(state.oopTurnBetStr, false),
    oopTurnRaise: (state) => parseBetSizes(state.oopTurnRaiseStr, true),
    oopRiverBet: (state) => parseBetSizes(state.oopRiverBetStr, false),
    oopRiverRaise: (state) => parseBetSizes(state.oopRiverRaiseStr, true),
    ipFlopBet: (state) => parseBetSizes(state.ipFlopBetStr, false),
    ipFlopRaise: (state) => parseBetSizes(state.ipFlopRaiseStr, true),
    ipTurnBet: (state) => parseBetSizes(state.ipTurnBetStr, false),
    ipTurnRaise: (state) => parseBetSizes(state.ipTurnRaiseStr, true),
    ipRiverBet: (state) => parseBetSizes(state.ipRiverBetStr, false),
    ipRiverRaise: (state) => parseBetSizes(state.ipRiverRaiseStr, true),
  },
});

export const useTmpConfigStore = defineStore("tmpConfig", {
  state: () => ({
    rangeRaw: [
      Float32Array.from({ length: (52 * 51) / 2 }, () => 0),
      Float32Array.from({ length: (52 * 51) / 2 }, () => 0),
    ],
    board: [] as number[],
    startingPot: 0,
    effectiveStack: 0,
    oopFlopBet: [] as number[],
    oopFlopRaise: [] as number[],
    oopTurnBet: [] as number[],
    oopTurnRaise: [] as number[],
    oopRiverBet: [] as number[],
    oopRiverRaise: [] as number[],
    ipFlopBet: [] as number[],
    ipFlopRaise: [] as number[],
    ipTurnBet: [] as number[],
    ipTurnRaise: [] as number[],
    ipRiverBet: [] as number[],
    ipRiverRaise: [] as number[],
    addAllInThreshold: 0,
    forceAllInThreshold: 0,
    adjustLastTwoBetSizes: true,
  }),
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
    oopFlopBet: [] as number[],
    oopFlopRaise: [] as number[],
    oopTurnBet: [] as number[],
    oopTurnRaise: [] as number[],
    oopRiverBet: [] as number[],
    oopRiverRaise: [] as number[],
    ipFlopBet: [] as number[],
    ipFlopRaise: [] as number[],
    ipTurnBet: [] as number[],
    ipTurnRaise: [] as number[],
    ipRiverBet: [] as number[],
    ipRiverRaise: [] as number[],
    addAllInThreshold: 0,
    forceAllInThreshold: 0,
    adjustLastTwoBetSizes: true,
  }),
});
