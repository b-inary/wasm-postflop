<template>
  <p class="flex items-center">
    <label>
      Number of threads:
      <input
        v-model="store.numThreads"
        class="w-20 ml-2 px-2 py-1 rounded-lg text-sm"
        type="number"
        min="1"
        max="64"
      />
    </label>
    <button
      :class="
        'rounded-lg shadow-sm ml-3 px-3.5 py-1.5 text-white text-sm font-medium ' +
        'bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-4 focus:ring-blue-300 ' +
        'disabled:opacity-40 disabled:bg-blue-600 disabled:cursor-not-allowed'
      "
      :disabled="isTreeBuilding || store.isSolverRunning"
      @click="buildTree"
    >
      Build tree
    </button>
  </p>

  <p>Status: {{ treeStatus }}</p>

  <div v-if="store.isTreeBuilt" class="mt-4">
    <p>
      <label>
        <input
          v-model="store.isCompressionEnabled"
          class="mr-2 disabled:opacity-40"
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
      <br />
      <label>
        <input
          v-model="store.isCompressionEnabled"
          class="mr-2 disabled:opacity-40"
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
      <button
        :class="
          'rounded-lg shadow-sm px-3.5 py-1.5 text-white text-sm font-medium ' +
          'bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-4 focus:ring-blue-300 ' +
          'disabled:opacity-40 disabled:bg-blue-600 disabled:cursor-not-allowed'
        "
        :disabled="
          store.hasSolverRun || store.memoryUsageCompressed > maxMemoryUsage
        "
        @click="runSolver"
      >
        Run solver
      </button>
      <button
        :class="
          'rounded-lg shadow-sm ml-3 px-3.5 py-1.5 text-white text-sm font-medium ' +
          'bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-4 focus:ring-red-300 ' +
          'disabled:opacity-40 disabled:bg-red-600 disabled:cursor-not-allowed'
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
          'disabled:opacity-40 disabled:bg-green-600 disabled:cursor-not-allowed'
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
          'disabled:opacity-40 disabled:bg-green-600 disabled:cursor-not-allowed'
        "
        @click="resumeSolver"
      >
        Resume
      </button>
    </p>

    <p v-if="store.hasSolverRun" class="mt-4">
      {{ iterationText }}
      <br />
      {{ exploitabilityText }}
      <br />
      {{ expectedValueText }}
      <br />
      {{ timeText }}
    </p>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, ref } from "vue";
import * as GlobalWorker from "../global-worker";
import { useStore } from "../store";

const maxMemoryUsage = 3.9 * 1024 * 1024 * 1024;

export default defineComponent({
  setup() {
    const store = useStore();

    let startTime = 0;
    const isTreeBuilding = ref(false);
    const treeStatus = ref("Module not loaded");

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
        const percentText = `${((exploitability * 100) / 60).toFixed(2)}%`;
        return `Exploitability: ${exploitability.toFixed(3)} (${percentText})`;
      }
    });

    const expectedValueText = computed(() => {
      const ev = store.expectedValue;
      if (ev[0] === -1 && ev[1] === -1) {
        return "";
      } else {
        return `EV: ${ev[0].toFixed(3)} vs ${ev[1].toFixed(3)}`;
      }
    });

    const timeText = computed(() => {
      if (store.elapsedTimeMs === 0 || !store.isSolverTerminated) {
        return "";
      } else {
        return `Time: ${(store.elapsedTimeMs / 1000).toFixed(2)}s`;
      }
    });

    const buildTree = async () => {
      store.isTreeBuilt = false;
      const numThreadsInt = Math.floor(store.numThreads);

      if (store.numThreads !== numThreadsInt) {
        treeStatus.value = "Error: Invalid number of threads";
        return;
      }

      if (numThreadsInt > 64) {
        treeStatus.value = "Error: Too many threads";
        return;
      }

      if (store.board.length !== 3) {
        treeStatus.value = "Error: Board must consist of 3 cards";
        return;
      }

      isTreeBuilding.value = true;
      treeStatus.value = "Building tree...";

      await GlobalWorker.init(numThreadsInt);
      const handler = await GlobalWorker.getHandler();

      const errorString = await handler.init(new Uint8Array(store.board));

      if (errorString) {
        isTreeBuilding.value = false;
        treeStatus.value = "Build tree error: " + errorString;
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

      const threadText = `${numThreadsInt} thread${
        numThreadsInt === 1 ? "" : "s"
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

      const initialPot = 60;
      const maxIter = 1000;
      const target = initialPot * 0.005;

      while (store.currentIteration < maxIter) {
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

      await handler.normalize();
      store.expectedValue = [
        (await handler.ev(0)) + initialPot / 2,
        (await handler.ev(1)) + initialPot / 2,
      ];

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
