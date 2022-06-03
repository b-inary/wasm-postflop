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
      :class="
        'rounded-lg shadow-sm ml-3 px-3.5 py-1.5 text-white text-sm font-medium ' +
        'bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-4 focus:ring-blue-300 ' +
        'disabled:opacity-40 disabled:bg-blue-600'
      "
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
        {{ (memoryUsage / (1024 * 1024 * 1024)).toFixed(2) }}
        GB of RAM
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
        {{ (memoryUsageCompressed / (1024 * 1024 * 1024)).toFixed(2) }}
        GB of RAM
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
          (targetExploitability < 0
            ? 'ring-1 ring-red-600 border-red-600 bg-red-50'
            : '')
        "
        :disabled="store.hasSolverRun"
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
          (maxIterations < 1 || maxIterations % 1 !== 0
            ? 'ring-1 ring-red-600 border-red-600 bg-red-50'
            : '')
        "
        :disabled="store.hasSolverRun"
        min="1"
        max="100000"
      />
    </p>

    <p class="mt-6">
      <button
        :class="
          'rounded-lg shadow-sm px-3.5 py-1.5 text-white text-sm font-medium ' +
          'bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-4 focus:ring-blue-300 ' +
          'disabled:opacity-40 disabled:bg-blue-600'
        "
        :disabled="
          store.hasSolverRun ||
          memoryUsageCompressed > maxMemoryUsage ||
          targetExploitability < 0 ||
          maxIterations < 1 ||
          maxIterations % 1 !== 0
        "
        @click="runSolver"
      >
        Run solver
      </button>
      <button
        :class="
          'rounded-lg shadow-sm ml-3 px-3.5 py-1.5 text-white text-sm font-medium ' +
          'bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-4 focus:ring-red-300 ' +
          'disabled:opacity-40 disabled:bg-red-600'
        "
        :disabled="!store.isSolverRunning"
        @click="() => (terminateFlag = true)"
      >
        Stop
      </button>
      <button
        v-if="!store.isSolverPaused"
        :class="
          'rounded-lg shadow-sm ml-3 px-3.5 py-1.5 text-white text-sm font-medium ' +
          'bg-green-600 hover:bg-green-700 focus:outline-none focus:ring-4 focus:ring-green-300 ' +
          'disabled:opacity-40 disabled:bg-green-600'
        "
        :disabled="!store.isSolverRunning"
        @click="() => (pauseFlag = true)"
      >
        Pause
      </button>
      <button
        v-else
        :class="
          'rounded-lg shadow-sm ml-3 px-3.5 py-1.5 text-white text-sm font-medium ' +
          'bg-green-600 hover:bg-green-700 focus:outline-none focus:ring-4 focus:ring-green-300 ' +
          'disabled:opacity-40 disabled:bg-green-600'
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
import { useStore } from "../store";
import { detect } from "detect-browser";

const maxMemoryUsage = 3.9 * 1024 * 1024 * 1024;
const browser = detect();
const isSafari = browser && (browser.name === "safari" || browser.os === "iOS");

function checkConfig(store: ReturnType<typeof useStore>): string | null {
  if (store.board.length !== 3) {
    return "Board must consist of exactly three cards";
  }

  if (store.startingPot <= 0) {
    return "Starting pot must be positive";
  }

  if (store.startingPot > 100000) {
    return "Starting pot is too large";
  }

  if (store.startingPot % 1 !== 0) {
    return "Starting pot must be an integer";
  }

  if (store.effectiveStack <= 0) {
    return "Effective stack must be positive";
  }

  if (store.effectiveStack > 100000) {
    return "Effective stack is too large";
  }

  if (store.effectiveStack % 1 !== 0) {
    return "Effective stack is be an integer";
  }

  if (store.oopFlopBetSizes === null) {
    return "Invalid tree configuration (OOP flop bet)";
  }

  if (store.oopFlopRaiseSizes === null) {
    return "Invalid tree configuration (OOP flop raise)";
  }

  if (store.oopTurnBetSizes === null) {
    return "Invalid tree configuration (OOP turn bet)";
  }

  if (store.oopTurnRaiseSizes === null) {
    return "Invalid tree configuration (OOP turn raise)";
  }

  if (store.oopRiverBetSizes === null) {
    return "Invalid tree configuration (OOP river bet)";
  }

  if (store.oopRiverRaiseSizes === null) {
    return "Invalid tree configuration (OOP river raise)";
  }

  if (store.ipFlopBetSizes === null) {
    return "Invalid tree configuration (IP flop bet)";
  }

  if (store.ipFlopRaiseSizes === null) {
    return "Invalid tree configuration (IP flop raise)";
  }

  if (store.ipTurnBetSizes === null) {
    return "Invalid tree configuration (IP turn bet)";
  }

  if (store.ipTurnRaiseSizes === null) {
    return "Invalid tree configuration (IP turn raise)";
  }

  if (store.ipRiverBetSizes === null) {
    return "Invalid tree configuration (IP river bet)";
  }

  if (store.ipRiverRaiseSizes === null) {
    return "Invalid tree configuration (IP river raise)";
  }

  if (store.addAllInThreshold < 0) {
    return "Invalid add all-in threshold";
  }

  if (store.addAllInThreshold > 1000) {
    return "Add all-in threshold is too large";
  }

  if (store.forceAllInThreshold < 0) {
    return "Invalid force all-in threshold";
  }

  if (store.forceAllInThreshold > 100) {
    return "Force all-in threshold is too large";
  }

  return null;
}

export default defineComponent({
  setup() {
    const store = useStore();

    const numThreads = ref((!isSafari && navigator.hardwareConcurrency) || 1);
    const targetExploitability = ref(0.5);
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
    const exploitability = ref(-1);
    const elapsedTimeMs = ref(-1);

    let startTime = 0;

    const iterationText = computed(() => {
      if (currentIteration.value === -1) {
        return "Allocating memory...";
      } else {
        return `Iteration: ${currentIteration.value}`;
      }
    });

    const exploitabilityText = computed(() => {
      if (exploitability.value === -1) {
        return "";
      } else {
        const valueText = exploitability.value.toFixed(2);
        const percent = (exploitability.value * 100) / store.startingPot;
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

      const configError = checkConfig(store);
      if (configError !== null) {
        treeStatus.value = `Error: ${configError}`;
        return;
      }

      // needed for type inference
      if (
        store.oopFlopBetSizes === null ||
        store.oopFlopRaiseSizes === null ||
        store.oopTurnBetSizes === null ||
        store.oopTurnRaiseSizes === null ||
        store.oopRiverBetSizes === null ||
        store.oopRiverRaiseSizes === null ||
        store.ipFlopBetSizes === null ||
        store.ipFlopRaiseSizes === null ||
        store.ipTurnBetSizes === null ||
        store.ipTurnRaiseSizes === null ||
        store.ipRiverBetSizes === null ||
        store.ipRiverRaiseSizes === null
      ) {
        return;
      }

      isTreeBuilding.value = true;
      treeStatus.value = "Building tree...";

      await GlobalWorker.init(numThreads.value);
      const handler = await GlobalWorker.getHandler();

      const errorString = await handler.init(
        store.rangeRaw[0],
        store.rangeRaw[1],
        new Uint8Array(store.board),
        store.startingPot,
        store.effectiveStack,
        new Float32Array(store.oopFlopBetSizes),
        new Float32Array(store.oopFlopRaiseSizes),
        new Float32Array(store.oopTurnBetSizes),
        new Float32Array(store.oopTurnRaiseSizes),
        new Float32Array(store.oopRiverBetSizes),
        new Float32Array(store.oopRiverRaiseSizes),
        new Float32Array(store.ipFlopBetSizes),
        new Float32Array(store.ipFlopRaiseSizes),
        new Float32Array(store.ipTurnBetSizes),
        new Float32Array(store.ipTurnRaiseSizes),
        new Float32Array(store.ipRiverBetSizes),
        new Float32Array(store.ipRiverRaiseSizes),
        store.addAllInThreshold / 100,
        store.forceAllInThreshold / 100,
        store.adjustLastTwoBetSizes
      );

      if (errorString) {
        isTreeBuilding.value = false;
        treeStatus.value = "Error: " + errorString;
        return;
      }

      memoryUsage.value = await handler.memoryUsage(false);
      memoryUsageCompressed.value = await handler.memoryUsage(true);

      if (memoryUsage.value <= maxMemoryUsage) {
        isCompressionEnabled.value = false;
      } else if (memoryUsageCompressed.value <= maxMemoryUsage) {
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
      exploitability.value = -1;
      elapsedTimeMs.value = -1;

      store.isSolverRunning = true;

      startTime = performance.now();

      await handler.allocateMemory(isCompressionEnabled.value);

      currentIteration.value = 0;
      await resumeSolver();
    };

    const resumeSolver = async () => {
      const handler = await GlobalWorker.getHandler();

      store.isSolverRunning = true;
      store.isSolverPaused = false;

      if (startTime === 0) {
        startTime = performance.now();
      }

      const target = (store.startingPot * targetExploitability.value) / 100;

      while (currentIteration.value < maxIterations.value) {
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

        if (currentIteration.value % 10 === 0 || terminateFlag.value) {
          exploitability.value = await handler.exploitability();
          if (exploitability.value <= target || terminateFlag.value) {
            break;
          }
        }
      }

      store.isSolverRunning = false;
      store.isFinalizing = true;

      store.normalizer = await handler.finalize();

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
