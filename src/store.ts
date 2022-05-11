import { defineStore } from "pinia";

export type MainView =
  | "OOPRange"
  | "IPRange"
  | "Board"
  | "TreeConfig"
  | "RunSolver"
  | "Result";

export const useStore = defineStore("app", {
  state: () => {
    return {
      mainView: "RunSolver" as MainView,
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
    };
  },

  getters: {
    hasSolverRun: (state) => {
      return (
        state.isSolverRunning ||
        state.isSolverPaused ||
        state.isSolverTerminated
      );
    },
  },
});
