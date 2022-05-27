<template>
  <p class="flex my-1 items-center">
    Number of threads:
    <input
      v-model="store.numThreads"
      type="number"
      :class="
        'w-20 ml-2 px-2 py-1 rounded-lg text-sm text-center ' +
        (store.numThreads < 1 ||
        store.numThreads > 64 ||
        store.numThreads % 1 !== 0
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
      :disabled="isTreeBuilding || store.isSolverRunning"
      @click="buildTree"
    >
      Build tree
    </button>
  </p>

  <p class="my-1">Status: {{ treeStatus }}</p>

  <div v-if="store.isTreeBuilt" class="mt-4">
    <p>
      <label
        :class="
          store.memoryUsage > maxMemoryUsage
            ? 'text-gray-500'
            : !store.hasSolverRun
            ? 'cursor-pointer'
            : ''
        "
      >
        <input
          v-model="store.isCompressionEnabled"
          class="mr-2 cursor-pointer disabled:opacity-40 disabled:cursor-default"
          type="radio"
          name="compression"
          :value="false"
          :disabled="store.hasSolverRun || store.memoryUsage > maxMemoryUsage"
        />
        No compression: requires
        {{ (store.memoryUsage / (1024 * 1024 * 1024)).toFixed(2) }}
        GB of RAM
        {{
          store.memoryUsage <= maxMemoryUsage ? "(fast)" : "(limit exceeded)"
        }}
      </label>
    </p>
    <p>
      <label
        :class="
          store.memoryUsageCompressed > maxMemoryUsage
            ? 'text-gray-500'
            : !store.hasSolverRun
            ? 'cursor-pointer'
            : ''
        "
      >
        <input
          v-model="store.isCompressionEnabled"
          class="mr-2 cursor-pointer disabled:opacity-40 disabled:cursor-default"
          type="radio"
          name="compression"
          :value="true"
          :disabled="
            store.hasSolverRun || store.memoryUsageCompressed > maxMemoryUsage
          "
        />
        Use compression: requires
        {{ (store.memoryUsageCompressed / (1024 * 1024 * 1024)).toFixed(2) }}
        GB of RAM
        {{
          store.memoryUsageCompressed <= maxMemoryUsage
            ? ""
            : "(limit exceeded)"
        }}
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
          store.memoryUsageCompressed > maxMemoryUsage ||
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
        @click="() => (store.terminateFlag = true)"
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
        @click="() => (store.pauseFlag = true)"
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
      <div v-if="store.isSolverRunning" class="flex items-center">
        <span class="spinner inline-block mr-3"></span>
        Solver running...
      </div>
      <div v-else-if="store.isSolverPaused">Solver paused.</div>
      <div v-else>Solver finished.</div>
      {{ iterationText }}
      <br />
      {{ exploitabilityText }}
      <br />
      {{ expectedValueText }}
      <br />
      {{ timeText }}
    </div>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, ref } from "vue";
import * as GlobalWorker from "../global-worker";
import { useStore } from "../store";

const maxMemoryUsage = 3.9 * 1024 * 1024 * 1024;

function checkConfig(store: ReturnType<typeof useStore>): string | null {
  if (store.numThreads < 1 || store.numThreads % 1 !== 0) {
    return "Invalid number of threads";
  }

  if (store.numThreads > 64) {
    return "Too many threads";
  }

  if (store.board.length !== 3) {
    return "Board must consist of 3 cards";
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

    let startTime = 0;
    const isTreeBuilding = ref(false);
    const treeStatus = ref("Module not loaded");
    const targetExploitability = ref(0.5);
    const maxIterations = ref(1000);

    const iterationText = computed(() => {
      if (store.currentIteration === -1) {
        return "Allocating memory...";
      } else {
        return `Iteration: ${store.currentIteration}`;
      }
    });

    const exploitabilityText = computed(() => {
      const exploitability = store.exploitability;
      if (exploitability === -1) {
        return "";
      } else {
        const percent = (exploitability * 100) / store.startingPot;
        const percentText = `${percent.toFixed(2)}%`;
        return `Exploitability: ${exploitability.toFixed(2)} (${percentText})`;
      }
    });

    const expectedValueText = computed(() => {
      const ev = store.expectedValue;
      if (ev[0] === -1 && ev[1] === -1) {
        return "";
      } else {
        return `EV: ${ev[0].toFixed(2)} vs ${ev[1].toFixed(2)}`;
      }
    });

    const timeText = computed(() => {
      if (store.elapsedTimeMs === -1) {
        return "Finalizing...";
      } else if (store.elapsedTimeMs === 0 || !store.isSolverTerminated) {
        return "";
      } else {
        return `Time: ${(store.elapsedTimeMs / 1000).toFixed(2)}s`;
      }
    });

    const buildTree = async () => {
      store.isTreeBuilt = false;

      const configError = checkConfig(store);
      if (configError !== null) {
        treeStatus.value = `Error: ${configError}`;
        return;
      }

      // needed for type inference
      if (
        store.oopFlopBetSizes === null ||
        store.oopFlopRaiseSizes == null ||
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

      await GlobalWorker.init(store.numThreads);
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

      store.memoryUsage = await handler.memoryUsage(false);
      store.memoryUsageCompressed = await handler.memoryUsage(true);

      if (store.memoryUsage <= maxMemoryUsage) {
        store.isCompressionEnabled = false;
      } else if (store.memoryUsageCompressed <= maxMemoryUsage) {
        store.isCompressionEnabled = true;
      }

      store.isTreeBuilt = true;
      store.isSolverRunning = false;
      store.isSolverPaused = false;
      store.isSolverTerminated = false;

      const threadText = `${store.numThreads} thread${
        store.numThreads === 1 ? "" : "s"
      }`;

      isTreeBuilding.value = false;
      treeStatus.value = `Successfully built tree (${threadText})`;
    };

    const runSolver = async () => {
      const handler = await GlobalWorker.getHandler();

      store.terminateFlag = false;
      store.pauseFlag = false;

      store.isSolverRunning = true;

      store.currentIteration = -1;
      store.exploitability = -1;
      store.expectedValue = [-1, -1];
      store.elapsedTimeMs = 0;

      startTime = performance.now();

      await handler.allocateMemory(store.isCompressionEnabled);

      store.currentIteration = 0;
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

      while (store.currentIteration < maxIterations.value) {
        if (store.pauseFlag) {
          const end = performance.now();
          store.elapsedTimeMs += end - startTime;
          startTime = 0;
          store.pauseFlag = false;
          store.isSolverRunning = false;
          store.isSolverPaused = true;
          return;
        }

        await handler.iterate(store.currentIteration);
        ++store.currentIteration;

        if (store.currentIteration % 10 === 0 || store.terminateFlag) {
          store.exploitability = await handler.exploitability();
          if (store.exploitability <= target || store.terminateFlag) {
            break;
          }
        }
      }

      store.elapsedTimeMs = -1;
      await handler.finalize();

      const oopEv = (await handler.ev()) + store.startingPot / 2;
      store.expectedValue = [oopEv, store.startingPot - oopEv];

      const end = performance.now();
      store.elapsedTimeMs += end - startTime;

      store.isSolverRunning = false;
      store.isSolverTerminated = true;
    };

    return {
      store,
      isTreeBuilding,
      treeStatus,
      maxMemoryUsage,
      targetExploitability,
      maxIterations,
      iterationText,
      exploitabilityText,
      expectedValueText,
      timeText,
      buildTree,
      runSolver,
      resumeSolver,
    };
  },
});
</script>

<style scoped>
.spinner {
  text-indent: -9999em;
  width: 1em;
  height: 1em;
  border-radius: 50%;
  background: #404040;
  background: -moz-linear-gradient(left, #404040 10%, rgba(64, 64, 64, 0) 42%);
  background: -webkit-linear-gradient(
    left,
    #404040 10%,
    rgba(64, 64, 64, 0) 42%
  );
  background: -o-linear-gradient(left, #404040 10%, rgba(64, 64, 64, 0) 42%);
  background: -ms-linear-gradient(left, #404040 10%, rgba(64, 64, 64, 0) 42%);
  background: linear-gradient(to right, #404040 10%, rgba(64, 64, 64, 0) 42%);
  position: relative;
  -webkit-animation: spinner-animation 0.8s infinite linear;
  animation: spinner-animation 0.8s infinite linear;
  -webkit-transform: translateZ(0);
  -ms-transform: translateZ(0);
  transform: translateZ(0);
}

.spinner:before {
  width: 50%;
  height: 50%;
  background: #404040;
  border-radius: 100% 0 0 0;
  position: absolute;
  top: 0;
  left: 0;
  content: "";
}

.spinner:after {
  @apply bg-gray-100;
  width: 75%;
  height: 75%;
  border-radius: 50%;
  content: "";
  margin: auto;
  position: absolute;
  top: 0;
  left: 0;
  bottom: 0;
  right: 0;
}

@-webkit-keyframes spinner-animation {
  0% {
    -webkit-transform: rotate(0deg);
    transform: rotate(0deg);
  }
  100% {
    -webkit-transform: rotate(360deg);
    transform: rotate(360deg);
  }
}

@keyframes spinner-animation {
  0% {
    -webkit-transform: rotate(0deg);
    transform: rotate(0deg);
  }
  100% {
    -webkit-transform: rotate(360deg);
    transform: rotate(360deg);
  }
}
</style>
