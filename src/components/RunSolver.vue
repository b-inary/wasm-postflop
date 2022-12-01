<template>
  <p class="flex my-1 items-center">
    Number of threads:
    <input
      v-model="numThreads"
      type="number"
      :class="
        'w-20 ml-2 px-2 py-1 rounded-lg text-sm text-center ' +
        (numThreads < 1 ||
        numThreads > (isSafari ? 1 : 64) ||
        numThreads % 1 !== 0
          ? 'ring-1 ring-red-600 border-red-600 bg-red-50'
          : '')
      "
      min="1"
      max="64"
    />
    <button
      class="ml-3 button button-blue"
      :disabled="isTreeBuilding || store.isSolverRunning || store.isFinalizing"
      @click="buildTree"
    >
      Build tree
    </button>
  </p>

  <p class="my-1">Status: {{ treeStatus }}</p>

  <div v-if="isTreeBuilt" class="mt-4">
    <p>
      <label
        :class="
          memoryUsage > maxMemoryUsage
            ? 'text-gray-400'
            : !store.hasSolverRun
            ? 'cursor-pointer'
            : ''
        "
      >
        <input
          v-model="isCompressionEnabled"
          class="mr-2 cursor-pointer disabled:opacity-40 disabled:cursor-default"
          type="radio"
          name="compression"
          :value="false"
          :disabled="store.hasSolverRun || memoryUsage > maxMemoryUsage"
        />
        No compression: requires
        {{
          memoryUsage >= 1023.5 * 1024 * 1024
            ? (memoryUsage / (1024 * 1024 * 1024)).toFixed(2) + " GB"
            : (memoryUsage / (1024 * 1024)).toFixed(0) + " MB"
        }}
        of RAM
        {{ memoryUsage <= maxMemoryUsage ? "(fast)" : "(limit exceeded)" }}
      </label>
    </p>
    <p>
      <label
        :class="
          memoryUsageCompressed > maxMemoryUsage
            ? 'text-gray-400'
            : !store.hasSolverRun
            ? 'cursor-pointer'
            : ''
        "
      >
        <input
          v-model="isCompressionEnabled"
          class="mr-2 cursor-pointer disabled:opacity-40 disabled:cursor-default"
          type="radio"
          name="compression"
          :value="true"
          :disabled="
            store.hasSolverRun || memoryUsageCompressed > maxMemoryUsage
          "
        />
        Use compression: requires
        {{
          memoryUsageCompressed >= 1023.5 * 1024 * 1024
            ? (memoryUsageCompressed / (1024 * 1024 * 1024)).toFixed(2) + " GB"
            : (memoryUsageCompressed / (1024 * 1024)).toFixed(0) + " MB"
        }}
        of RAM
        {{ memoryUsageCompressed <= maxMemoryUsage ? "" : "(limit exceeded)" }}
      </label>
    </p>

    <p class="mt-4">
      Target exploitability:
      <input
        v-model="targetExploitability"
        type="number"
        :class="
          'w-20 ml-2 px-2 py-1 rounded-lg text-sm text-center ' +
          (targetExploitability <= 0
            ? 'ring-1 ring-red-600 border-red-600 bg-red-50'
            : '')
        "
        :disabled="store.hasSolverRun && !store.isSolverPaused"
        min="0"
        step="0.05"
      />
      %
    </p>

    <p class="mt-1">
      Maximum number of iterations:
      <input
        v-model="maxIterations"
        type="number"
        :class="
          'w-[5.5rem] ml-2 px-2 py-1 rounded-lg text-sm text-center ' +
          (maxIterations < 0 ||
          maxIterations % 1 !== 0 ||
          maxIterations > 100000
            ? 'ring-1 ring-red-600 border-red-600 bg-red-50'
            : '')
        "
        :disabled="store.hasSolverRun && !store.isSolverPaused"
        min="0"
        max="100000"
      />
    </p>

    <p class="flex mt-6 gap-3">
      <button
        class="button button-blue"
        :disabled="
          store.hasSolverRun ||
          memoryUsageCompressed > maxMemoryUsage ||
          targetExploitability <= 0 ||
          maxIterations < 0 ||
          maxIterations % 1 !== 0 ||
          maxIterations > 100000
        "
        @click="runSolver"
      >
        Run solver
      </button>
      <button
        class="button button-red"
        :disabled="!store.isSolverRunning"
        @click="() => (terminateFlag = true)"
      >
        Stop
      </button>
      <button
        v-if="!store.isSolverPaused"
        class="button button-green"
        :disabled="!store.isSolverRunning"
        @click="() => (pauseFlag = true)"
      >
        Pause
      </button>
      <button
        v-else
        class="button button-green"
        :disabled="
          targetExploitability <= 0 ||
          maxIterations < 0 ||
          maxIterations % 1 !== 0 ||
          maxIterations > 100000
        "
        @click="resumeSolver"
      >
        Resume
      </button>
    </p>

    <div v-if="store.hasSolverRun" class="mt-6">
      <div class="flex items-center">
        <span
          v-if="store.isSolverRunning || store.isFinalizing"
          class="spinner inline-block mr-3"
        ></span>
        {{
          store.isSolverRunning
            ? "Solver running..."
            : store.isFinalizing
            ? "Finalizing..."
            : store.isSolverPaused
            ? "Solver paused."
            : "Solver finished."
        }}
      </div>
      {{ iterationText }}
      <br />
      {{ exploitabilityText }}
      <br />
      {{ timeText }}
    </div>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, ref } from "vue";
import * as GlobalWorker from "../global-worker";
import {
  useStore,
  useConfigStore,
  useTmpConfigStore,
  saveConfig,
  saveConfigTmp,
} from "../store";
import { detect } from "detect-browser";

const maxMemoryUsage = 3.9 * 1024 * 1024 * 1024;
const browser = detect();
const isSafari = browser && (browser.name === "safari" || browser.os === "iOS");

const checkConfig = (
  config: ReturnType<typeof useConfigStore>
): string | null => {
  if (config.board.length < 3) {
    return "Board must consist of at least three cards";
  }

  if (config.startingPot <= 0) {
    return "Starting pot must be positive";
  }

  if (config.startingPot > 100000) {
    return "Starting pot is too large";
  }

  if (config.startingPot % 1 !== 0) {
    return "Starting pot must be an integer";
  }

  if (config.effectiveStack <= 0) {
    return "Effective stack must be positive";
  }

  if (config.effectiveStack > 100000) {
    return "Effective stack is too large";
  }

  if (config.effectiveStack % 1 !== 0) {
    return "Effective stack is be an integer";
  }

  const betConfig = [
    { s: config.oopFlopBetSanitized, kind: "OOP flop bet" },
    { s: config.oopFlopRaiseSanitized, kind: "OOP flop raise" },
    { s: config.oopTurnBetSanitized, kind: "OOP turn bet" },
    { s: config.oopTurnRaiseSanitized, kind: "OOP turn raise" },
    { s: config.oopRiverBetSanitized, kind: "OOP river bet" },
    { s: config.oopRiverRaiseSanitized, kind: "OOP river raise" },
    { s: config.ipFlopBetSanitized, kind: "IP flop bet" },
    { s: config.ipFlopRaiseSanitized, kind: "IP flop raise" },
    { s: config.ipTurnBetSanitized, kind: "IP turn bet" },
    { s: config.ipTurnRaiseSanitized, kind: "IP turn raise" },
    { s: config.ipRiverBetSanitized, kind: "IP river bet" },
    { s: config.ipRiverRaiseSanitized, kind: "IP river raise" },
  ];

  for (const { s, kind } of betConfig) {
    if (!s.valid) {
      return `${kind}: ${s.s}`;
    }
  }

  if (config.donkOption) {
    if (!config.oopTurnDonkSanitized.valid) {
      return `OOP turn donk: ${config.oopTurnDonkSanitized.s}`;
    }
    if (!config.oopRiverDonkSanitized.valid) {
      return `OOP river donk: ${config.oopRiverDonkSanitized.s}`;
    }
  }

  if (config.addAllInThreshold < 0) {
    return "Invalid add all-in threshold";
  }

  if (config.forceAllInThreshold < 0) {
    return "Invalid force all-in threshold";
  }

  if (config.mergingThreshold < 0) {
    return "Invalid merging threshold";
  }

  return null;
};

const convertBetString = (s: string): string => {
  if (s === "") return s;
  return s
    .split(", ")
    .map((e) => ("acex".includes(e[e.length - 1]) ? e : e + "%"))
    .join(",");
};

export default defineComponent({
  setup() {
    const store = useStore();
    const config = useConfigStore();
    const tmpConfig = useTmpConfigStore();

    const numThreads = ref((!isSafari && navigator.hardwareConcurrency) || 1);
    const targetExploitability = ref(0.3);
    const maxIterations = ref(1000);

    const isTreeBuilding = ref(false);
    const isTreeBuilt = ref(false);
    const treeStatus = ref("Module not loaded");
    const memoryUsage = ref(0);
    const memoryUsageCompressed = ref(0);
    const isCompressionEnabled = ref(false);
    const terminateFlag = ref(false);
    const pauseFlag = ref(false);
    const currentIteration = ref(-1);
    const exploitability = ref(Number.POSITIVE_INFINITY);
    const elapsedTimeMs = ref(-1);

    let startTime = 0;
    let exploitabilityUpdated = false;

    const iterationText = computed(() => {
      if (currentIteration.value === -1) {
        return "Allocating memory...";
      } else {
        return `Iteration: ${currentIteration.value}`;
      }
    });

    const exploitabilityText = computed(() => {
      if (!Number.isFinite(exploitability.value)) {
        return "";
      } else {
        const valueText = exploitability.value.toFixed(2);
        const percent = (exploitability.value * 100) / config.startingPot;
        const percentText = `${percent.toFixed(2)}%`;
        return `Exploitability: ${valueText} (${percentText})`;
      }
    });

    const timeText = computed(() => {
      if (elapsedTimeMs.value === -1 || !store.isSolverFinished) {
        return "";
      } else {
        return `Time: ${(elapsedTimeMs.value / 1000).toFixed(2)}s`;
      }
    });

    const buildTree = async () => {
      isTreeBuilt.value = false;

      if (numThreads.value < 1 || numThreads.value % 1 !== 0) {
        treeStatus.value = "Error: Invalid number of threads";
        return;
      }

      if (numThreads.value > 64) {
        treeStatus.value = "Error: Too many threads";
        return;
      }

      if (isSafari && numThreads.value > 1) {
        treeStatus.value =
          "Error: Multithreading is not supported on iOS and Safari";
        return;
      }

      const configError = checkConfig(config);
      if (configError !== null) {
        treeStatus.value = `Error: ${configError}`;
        return;
      }

      saveConfigTmp();
      isTreeBuilding.value = true;
      treeStatus.value = "Building tree...";

      await GlobalWorker.init(numThreads.value);
      const handler = await GlobalWorker.getHandler();

      const errorString = await handler.init(
        tmpConfig.rangeRaw[0],
        tmpConfig.rangeRaw[1],
        new Uint8Array(tmpConfig.board),
        tmpConfig.startingPot,
        tmpConfig.effectiveStack,
        tmpConfig.donkOption,
        convertBetString(tmpConfig.oopFlopBet),
        convertBetString(tmpConfig.oopFlopRaise),
        convertBetString(tmpConfig.oopTurnBet),
        convertBetString(tmpConfig.oopTurnRaise),
        convertBetString(tmpConfig.oopTurnDonk),
        convertBetString(tmpConfig.oopRiverBet),
        convertBetString(tmpConfig.oopRiverRaise),
        convertBetString(tmpConfig.oopRiverDonk),
        convertBetString(tmpConfig.ipFlopBet),
        convertBetString(tmpConfig.ipFlopRaise),
        convertBetString(tmpConfig.ipTurnBet),
        convertBetString(tmpConfig.ipTurnRaise),
        convertBetString(tmpConfig.ipRiverBet),
        convertBetString(tmpConfig.ipRiverRaise),
        tmpConfig.addAllInThreshold / 100,
        tmpConfig.forceAllInThreshold / 100,
        tmpConfig.mergingThreshold / 100
      );

      if (errorString) {
        isTreeBuilding.value = false;
        treeStatus.value = "Error: " + errorString;
        return;
      }

      saveConfig();

      memoryUsage.value = await handler.memoryUsage(false);
      memoryUsageCompressed.value = await handler.memoryUsage(true);

      if (
        memoryUsage.value > maxMemoryUsage &&
        memoryUsageCompressed.value <= maxMemoryUsage
      ) {
        isCompressionEnabled.value = true;
      }

      const threadText = `${numThreads.value} thread${
        numThreads.value === 1 ? "" : "s"
      }`;

      isTreeBuilding.value = false;
      isTreeBuilt.value = true;
      treeStatus.value = `Successfully built tree (${threadText})`;

      store.isSolverRunning = false;
      store.isSolverPaused = false;
      store.isSolverFinished = false;
    };

    const runSolver = async () => {
      const handler = await GlobalWorker.getHandler();

      terminateFlag.value = false;
      pauseFlag.value = false;
      currentIteration.value = -1;
      exploitability.value = Number.POSITIVE_INFINITY;
      elapsedTimeMs.value = -1;

      store.isSolverRunning = true;

      startTime = performance.now();

      await handler.allocateMemory(isCompressionEnabled.value);

      currentIteration.value = 0;
      exploitability.value = Math.max(await handler.exploitability(), 0);
      exploitabilityUpdated = true;

      await resumeSolver();
    };

    const resumeSolver = async () => {
      const handler = await GlobalWorker.getHandler();

      store.isSolverRunning = true;
      store.isSolverPaused = false;

      if (startTime === 0) {
        startTime = performance.now();
      }

      const target = (config.startingPot * targetExploitability.value) / 100;

      while (
        !terminateFlag.value &&
        currentIteration.value < maxIterations.value &&
        exploitability.value > target
      ) {
        if (pauseFlag.value) {
          const end = performance.now();
          elapsedTimeMs.value += end - startTime;
          startTime = 0;
          pauseFlag.value = false;
          store.isSolverRunning = false;
          store.isSolverPaused = true;
          return;
        }

        await handler.iterate(currentIteration.value);
        ++currentIteration.value;
        exploitabilityUpdated = false;

        if (currentIteration.value % 10 === 0) {
          exploitability.value = Math.max(await handler.exploitability(), 0);
          exploitabilityUpdated = true;
        }
      }

      if (!exploitabilityUpdated) {
        exploitability.value = Math.max(await handler.exploitability(), 0);
      }

      store.isSolverRunning = false;
      store.isFinalizing = true;

      await handler.finalize();

      store.isFinalizing = false;
      store.isSolverFinished = true;

      const end = performance.now();
      elapsedTimeMs.value += end - startTime;
    };

    return {
      store,
      numThreads,
      isSafari,
      targetExploitability,
      maxIterations,
      isTreeBuilding,
      isTreeBuilt,
      treeStatus,
      maxMemoryUsage,
      memoryUsage,
      memoryUsageCompressed,
      isCompressionEnabled,
      terminateFlag,
      pauseFlag,
      iterationText,
      exploitabilityText,
      timeText,
      buildTree,
      runSolver,
      resumeSolver,
    };
  },
});
</script>

<style scoped>
.button {
  @apply rounded-lg shadow-sm px-3.5 py-1.5 text-white text-sm font-medium;
  @apply focus:outline-none focus:ring-4 disabled:opacity-40;
}

.button-blue {
  @apply bg-blue-600 hover:bg-blue-700 focus:ring-blue-300 disabled:bg-blue-600;
}

.button-red {
  @apply bg-red-600 hover:bg-red-700 focus:ring-red-300 disabled:bg-red-600;
}

.button-green {
  @apply bg-green-600 hover:bg-green-700 focus:ring-green-300 disabled:bg-green-600;
}
</style>
