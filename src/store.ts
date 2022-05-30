import { defineStore } from "pinia";
import * as GlobalWorker from "./global-worker";

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
    mainView: "About" as MainView,
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
    isSolverRunning: false,
    isSolverPaused: false,
    isSolverFinished: false,
    isFinalizing: false,
    privateHandCards: [] as number[][][],
    actions: [] as {
      type: "Player" | "Turn" | "River";
      candidates: { str: string; isSelected: boolean; isTerminal: boolean }[];
    }[],
    result: [] as {
      card1: number;
      card2: number;
      weight: number;
      weightNormalized: number;
      expectedValue: number;
      strategy: number[];
    }[],
    resultSummary: [] as {
      enabled: boolean;
      weight: number;
      expectedValue: number;
      strategy: number[];
    }[],
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

    currentRound: (state): "Flop" | "Turn" | "River" => {
      if (state.actions.findIndex((a) => a.type === "River") >= 0) {
        return "River";
      } else if (state.actions.findIndex((a) => a.type === "Turn") >= 0) {
        return "Turn";
      } else {
        return "Flop";
      }
    },
  },

  actions: {
    async initResults() {
      const handler = await GlobalWorker.getHandler();

      for (let player = 0; player < 2; ++player) {
        const result = await handler.privateHandCards(player);
        this.privateHandCards[player] = [];
        for (let i = 0; i < result.length; i += 2) {
          this.privateHandCards[player].push([result[i + 1], result[i]]);
        }
      }

      this.actions = [];
      this.result = [];
    },

    async onSolverFinished() {
      const handler = await GlobalWorker.getHandler();

      const actions = (await handler.getActions()).split("/");
      const isTerminal = await handler.isTerminal();
      const isChance = "0" <= actions[0][0] && actions[0][0] <= "9";
      this.actions.push({
        type: !isChance
          ? "Player"
          : this.currentRound === "Flop"
          ? "Turn"
          : "River",
        candidates: [...Array(actions.length)]
          .map((_, i) => {
            return {
              str: actions[i],
              isSelected: false,
              isTerminal: Boolean(isTerminal[i]),
            };
          })
          .reverse(),
      });

      const player = await handler.currentPlayer();
      const cards = this.privateHandCards[player];

      const weights = await handler.getWeights();
      const weightsNormalized = await handler.getNormalizedWeights();
      const expectedValues = await handler.getExpectedValues();
      const strategy = await handler.getStrategy();
      const numActions = strategy.length / cards.length;

      this.result = [];
      for (let i = 0; i < cards.length; ++i) {
        this.result.push({
          card1: cards[i][0],
          card2: cards[i][1],
          weight: weights[i],
          weightNormalized: weightsNormalized[i],
          expectedValue: expectedValues[i],
          strategy: [...Array(numActions)]
            .map((_, j) => strategy[j * cards.length + i])
            .reverse(),
        });
      }

      const weightSum = Array.from({ length: 13 * 13 }, () => 0);
      this.resultSummary = Array.from({ length: 13 * 13 }, () => {
        return {
          enabled: false,
          weight: 0,
          expectedValue: 0,
          strategy: Array.from({ length: numActions }, () => 0),
        };
      });

      for (let i = 0; i < cards.length; ++i) {
        const rank1 = Math.floor(cards[i][0] / 4);
        const suit1 = cards[i][0] % 4;
        const rank2 = Math.floor(cards[i][1] / 4);
        const suit2 = cards[i][1] % 4;

        let row, col;
        if (rank1 === rank2) {
          row = 12 - rank1;
          col = 12 - rank1;
        } else if (suit1 === suit2) {
          row = 12 - rank1;
          col = 12 - rank2;
        } else {
          row = 12 - rank2;
          col = 12 - rank1;
        }

        const idx = row * 13 + col;
        const weight = weightsNormalized[i];
        weightSum[idx] += weight;
        this.resultSummary[idx].enabled ||= weights[i] >= 0.5 / 100;
        this.resultSummary[idx].weight += weight * weights[i];
        this.resultSummary[idx].expectedValue += expectedValues[i];
        for (let j = 0; j < numActions; ++j) {
          this.resultSummary[idx].strategy[j] +=
            weight * this.result[i].strategy[j];
        }
      }

      for (let i = 0; i < 13 * 13; ++i) {
        if (weightSum[i] > 0) {
          this.resultSummary[i].weight /= weightSum[i];
          this.resultSummary[i].expectedValue /= weightSum[i];
          for (let j = 0; j < numActions; ++j) {
            this.resultSummary[i].strategy[j] /= weightSum[i];
          }
        }
      }
    },
  },
});
