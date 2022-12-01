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

export const cardText = (card: number) => {
  return {
    rank: ranks[Math.floor(card / 4)],
    suit: suits[card % 4],
    colorClass: suitClasses[card % 4],
  };
};

const forbiddenChars = /[beox+-]/;

const parseFloat = (s: string): number => {
  if (forbiddenChars.test(s)) {
    return Number.NaN;
  } else {
    return Number(s);
  }
};

export const sanitizeBetString = (
  s: string,
  is_raise: boolean
): { s: string; valid: boolean } => {
  const trimmed = s.trim();
  if (trimmed === "") return { s: "", valid: true };

  const split = trimmed.split(",");
  const elements = split.map((s) => s.trim().toLowerCase());

  if (elements[elements.length - 1] === "") {
    elements.pop();
  }

  if (elements.length >= 32) {
    return { s: "Too many specifications", valid: false };
  }

  let sanitized = "";

  for (const e of elements) {
    if (sanitized !== "") {
      sanitized += ", ";
    }
    if (e === "") {
      return { s: "Found empty string", valid: false };
    } else if (e.endsWith("x")) {
      // Previous bet relative
      if (!is_raise) {
        return {
          s: `Multiplicative size is not allowed: ${e}`,
          valid: false,
        };
      } else {
        const float = parseFloat(e.slice(0, -1));
        if (Number.isNaN(float)) {
          return { s: `Invalid number: ${e}`, valid: false };
        } else if (float <= 1.0) {
          return { s: `Multiplier must be greater than 1: ${e}`, valid: false };
        } else {
          sanitized += `${float}x`;
        }
      }
    } else if (e.endsWith("c")) {
      // Additive
      const float = parseFloat(e.slice(0, -1));
      if (Number.isNaN(float)) {
        return { s: `Invalid number: ${e}`, valid: false };
      } else if (float % 1 !== 0) {
        return { s: `Addition size must be an integer: ${e}`, valid: false };
      } else if (float > 100000) {
        return { s: `Addition size too large: ${e}`, valid: false };
      } else {
        sanitized += `${float}c`;
      }
    } else if (e.includes("e")) {
      // Geometric
      const split = e.split("e");
      if (split.length !== 2) {
        return { s: `Invalid geometric specification: ${e}`, valid: false };
      }
      const float1 = split[0] === "" ? null : parseFloat(split[0]);
      const float2 = split[1] === "" ? null : parseFloat(split[1]);
      if (float1 !== null) {
        if (Number.isNaN(float1)) {
          return { s: `Invalid number: ${e}`, valid: false };
        } else if (float1 % 1 !== 0 || float1 === 0) {
          return {
            s: `Geometric size must be a positive integer: ${e}`,
            valid: false,
          };
        } else if (float1 > 100) {
          return { s: `Geometric size too large: ${e}`, valid: false };
        }
      }
      if (float2 !== null && Number.isNaN(float2)) {
        return { s: `Invalid number: ${e}`, valid: false };
      }
      sanitized += `${float1 ?? ""}e${float2 ?? ""}`;
    } else if (e === "a") {
      // All-in
      sanitized += "a";
    } else {
      // Pot relative
      const float = parseFloat(e);
      if (Number.isNaN(float)) {
        return { s: `Invalid number: ${e}`, valid: false };
      }
      sanitized += `${float}`;
    }
  }

  return { s: sanitized, valid: true };
};

export const saveConfigTmp = () => {
  const config = useConfigStore();
  const tmpConfig = useTmpConfigStore();

  tmpConfig.rangeRaw[0].set(config.rangeRaw[0]);
  tmpConfig.rangeRaw[1].set(config.rangeRaw[1]);

  tmpConfig.$patch({
    board: [...config.board],
    startingPot: config.startingPot,
    effectiveStack: config.effectiveStack,
    donkOption: config.donkOption,
    oopFlopBet: config.oopFlopBet,
    oopFlopRaise: config.oopFlopRaise,
    oopTurnBet: config.oopTurnBet,
    oopTurnRaise: config.oopTurnRaise,
    oopTurnDonk: config.oopTurnDonk,
    oopRiverBet: config.oopRiverBet,
    oopRiverRaise: config.oopRiverRaise,
    oopRiverDonk: config.oopRiverDonk,
    ipFlopBet: config.ipFlopBet,
    ipFlopRaise: config.ipFlopRaise,
    ipTurnBet: config.ipTurnBet,
    ipTurnRaise: config.ipTurnRaise,
    ipRiverBet: config.ipRiverBet,
    ipRiverRaise: config.ipRiverRaise,
    addAllInThreshold: config.addAllInThreshold,
    forceAllInThreshold: config.forceAllInThreshold,
    mergingThreshold: config.mergingThreshold,
  });
};

export const saveConfig = () => {
  const tmpConfig = useTmpConfigStore();
  const savedConfig = useSavedConfigStore();

  savedConfig.rangeRaw[0].set(tmpConfig.rangeRaw[0]);
  savedConfig.rangeRaw[1].set(tmpConfig.rangeRaw[1]);

  savedConfig.$patch({
    board: tmpConfig.board,
    startingPot: tmpConfig.startingPot,
    effectiveStack: tmpConfig.effectiveStack,
    donkOption: tmpConfig.donkOption,
    oopFlopBet: tmpConfig.oopFlopBet,
    oopFlopRaise: tmpConfig.oopFlopRaise,
    oopTurnBet: tmpConfig.oopTurnBet,
    oopTurnRaise: tmpConfig.oopTurnRaise,
    oopTurnDonk: tmpConfig.oopTurnDonk,
    oopRiverBet: tmpConfig.oopRiverBet,
    oopRiverRaise: tmpConfig.oopRiverRaise,
    oopRiverDonk: tmpConfig.oopRiverDonk,
    ipFlopBet: tmpConfig.ipFlopBet,
    ipFlopRaise: tmpConfig.ipFlopRaise,
    ipTurnBet: tmpConfig.ipTurnBet,
    ipTurnRaise: tmpConfig.ipTurnRaise,
    ipRiverBet: tmpConfig.ipRiverBet,
    ipRiverRaise: tmpConfig.ipRiverRaise,
    addAllInThreshold: tmpConfig.addAllInThreshold,
    forceAllInThreshold: tmpConfig.forceAllInThreshold,
    mergingThreshold: tmpConfig.mergingThreshold,
  });
};

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
    donkOption: false,
    oopFlopBet: "",
    oopFlopRaise: "",
    oopTurnBet: "",
    oopTurnRaise: "",
    oopTurnDonk: "",
    oopRiverBet: "",
    oopRiverRaise: "",
    oopRiverDonk: "",
    ipFlopBet: "",
    ipFlopRaise: "",
    ipTurnBet: "",
    ipTurnRaise: "",
    ipRiverBet: "",
    ipRiverRaise: "",
    addAllInThreshold: 0,
    forceAllInThreshold: 0,
    mergingThreshold: 0,
  }),

  getters: {
    oopFlopBetSanitized: (s) => sanitizeBetString(s.oopFlopBet, false),
    oopFlopRaiseSanitized: (s) => sanitizeBetString(s.oopFlopRaise, true),
    oopTurnBetSanitized: (s) => sanitizeBetString(s.oopTurnBet, false),
    oopTurnRaiseSanitized: (s) => sanitizeBetString(s.oopTurnRaise, true),
    oopTurnDonkSanitized: (s) => sanitizeBetString(s.oopTurnDonk, false),
    oopRiverBetSanitized: (s) => sanitizeBetString(s.oopRiverBet, false),
    oopRiverRaiseSanitized: (s) => sanitizeBetString(s.oopRiverRaise, true),
    oopRiverDonkSanitized: (s) => sanitizeBetString(s.oopRiverDonk, false),
    ipFlopBetSanitized: (s) => sanitizeBetString(s.ipFlopBet, false),
    ipFlopRaiseSanitized: (s) => sanitizeBetString(s.ipFlopRaise, true),
    ipTurnBetSanitized: (s) => sanitizeBetString(s.ipTurnBet, false),
    ipTurnRaiseSanitized: (s) => sanitizeBetString(s.ipTurnRaise, true),
    ipRiverBetSanitized: (s) => sanitizeBetString(s.ipRiverBet, false),
    ipRiverRaiseSanitized: (s) => sanitizeBetString(s.ipRiverRaise, true),
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
    donkOption: false,
    oopFlopBet: "",
    oopFlopRaise: "",
    oopTurnBet: "",
    oopTurnRaise: "",
    oopTurnDonk: "",
    oopRiverBet: "",
    oopRiverRaise: "",
    oopRiverDonk: "",
    ipFlopBet: "",
    ipFlopRaise: "",
    ipTurnBet: "",
    ipTurnRaise: "",
    ipRiverBet: "",
    ipRiverRaise: "",
    addAllInThreshold: 0,
    forceAllInThreshold: 0,
    mergingThreshold: 0,
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
    donkOption: false,
    oopFlopBet: "",
    oopFlopRaise: "",
    oopTurnBet: "",
    oopTurnRaise: "",
    oopTurnDonk: "",
    oopRiverBet: "",
    oopRiverRaise: "",
    oopRiverDonk: "",
    ipFlopBet: "",
    ipFlopRaise: "",
    ipTurnBet: "",
    ipTurnRaise: "",
    ipRiverBet: "",
    ipRiverRaise: "",
    addAllInThreshold: 0,
    forceAllInThreshold: 0,
    mergingThreshold: 0,
  }),
});
