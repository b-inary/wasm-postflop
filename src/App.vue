<template>
  <p>
    <label>
      Number of threads:
      <input v-model="numThreads" type="number" min="1" max="64" />
    </label>
    {{ " " }}
    <button :disabled="isSolverRunning" @click="buildTree">Build tree</button>
    <br />
    Status: {{ treeStatus }}
  </p>

  <div v-if="isTreeBuilt">
    <p>
      <label>
        <input
          v-model="enableCompression"
          type="radio"
          name="compression"
          :value="false"
          :disabled="hasSolverRun || memUsage > maxMemUsage"
        />
        No compression: needs
        {{ (memUsage / (1024 * 1024 * 1024)).toFixed(2) }} GB of RAM
        {{ memUsage <= maxMemUsage ? "(fast)" : "(limit exceeded)" }}
      </label>
      <br />
      <label>
        <input
          v-model="enableCompression"
          type="radio"
          name="compression"
          :value="true"
          :disabled="hasSolverRun || memUsageCompressed > maxMemUsage"
        />
        Use compression: needs
        {{ (memUsageCompressed / (1024 * 1024 * 1024)).toFixed(2) }} GB of RAM
        {{ memUsageCompressed <= maxMemUsage ? "" : "(limit exceeded)" }}
      </label>
    </p>

    <p>
      <button
        :disabled="hasSolverRun || memUsageCompressed > maxMemUsage"
        @click="runSolver"
      >
        Run solver
      </button>
      {{ " " }}
      <button :disabled="!isSolverRunning" @click="() => (terminate = true)">
        Stop
      </button>
      {{ " " }}
      <button
        v-if="!(hasSolverRun && !hasSolverFinished && !isSolverRunning)"
        :disabled="!isSolverRunning"
        @click="() => (pause = true)"
      >
        Pause
      </button>
      <button v-else @click="resumeSolver">Resume</button>
    </p>

    <p v-if="hasSolverRun">
      {{ iterText }}
      <br />
      {{ exploitText }}
      <br />
      {{ evText }}
      <br />
      {{ timeText }}
    </p>
  </div>
</template>

<script lang="ts">
import * as Comlink from "comlink";
import { defineComponent, ref } from "vue";
import { WorkerApi } from "./wasm-worker";

export default defineComponent({
  setup() {
    let worker: Worker | null = null;
    let startTime = 0;
    let elapsedTime = 0;
    let currentIter = 0;

    const numThreads = ref(navigator.hardwareConcurrency || 1);
    const isTreeBuilt = ref(false);
    const treeStatus = ref("Module not loaded");
    const enableCompression = ref(false);
    const memUsage = ref(0);
    const memUsageCompressed = ref(0);
    const maxMemUsage = 3.9 * 1024 * 1024 * 1024;
    const terminate = ref(false);
    const pause = ref(false);
    const hasSolverRun = ref(false);
    const isSolverRunning = ref(false);
    const hasSolverFinished = ref(false);
    const iterText = ref("");
    const exploitText = ref("");
    const evText = ref("");
    const timeText = ref("");

    const buildTree = async () => {
      isTreeBuilt.value = false;

      if (worker) {
        worker.terminate();
        worker = null;
      }

      const numThreadsInt = Math.floor(Number(numThreads.value));

      if (numThreads.value !== numThreadsInt) {
        treeStatus.value = "Error: Invalid number of threads";
        return;
      }

      if (numThreads.value > 64) {
        treeStatus.value = "Error: Too many threads";
        return;
      }

      treeStatus.value = "Building tree...";

      worker = new Worker(new URL("./wasm-worker.ts", import.meta.url), {
        type: "module",
      });

      await Comlink.wrap<WorkerApi>(worker).init(numThreads.value);
      const handler = await Comlink.wrap<WorkerApi>(worker).getHandler();

      const errorString = await handler.init();

      if (errorString) {
        treeStatus.value = "Build tree error: " + errorString;
        return;
      }

      memUsage.value = await handler.memoryUsage(false);
      memUsageCompressed.value = await handler.memoryUsage(true);

      if (memUsage.value <= maxMemUsage) {
        enableCompression.value = false;
      } else if (memUsageCompressed.value <= maxMemUsage) {
        enableCompression.value = true;
      }

      const threadText = `${numThreads.value} thread${
        numThreads.value === 1 ? "" : "s"
      }`;

      isTreeBuilt.value = true;
      hasSolverRun.value = false;
      treeStatus.value = `Successfully built tree (${threadText})`;
    };

    const runSolver = async () => {
      if (worker === null) {
        return;
      }

      const handler = await Comlink.wrap<WorkerApi>(worker).getHandler();

      terminate.value = false;
      pause.value = false;

      hasSolverRun.value = true;
      isSolverRunning.value = true;
      hasSolverFinished.value = false;

      iterText.value = "Allocating memory...";
      exploitText.value = "";
      evText.value = "";
      timeText.value = "";

      startTime = performance.now();
      elapsedTime = 0;

      await handler.allocateMemory(enableCompression.value);

      currentIter = 0;
      iterText.value = "Iteration 0";
      await resumeSolver();
    };

    const resumeSolver = async () => {
      if (worker === null) {
        return;
      }

      const handler = await Comlink.wrap<WorkerApi>(worker).getHandler();

      isSolverRunning.value = true;

      if (startTime === 0) {
        startTime = performance.now();
      }

      const initialPot = 60;
      const maxIter = 1000;
      const target = initialPot * 0.005;

      for (; currentIter < maxIter; ++currentIter) {
        if (pause.value) {
          const end = performance.now();
          elapsedTime += end - startTime;
          startTime = 0;
          pause.value = false;
          isSolverRunning.value = false;
          return;
        }

        await handler.iterate(currentIter);
        iterText.value = `Iteration ${currentIter + 1}`;

        if ((currentIter + 1) % 10 === 0 || terminate.value) {
          const exp = await handler.exploitability();
          const perText = `${((exp * 100) / initialPot).toFixed(2)}%`;
          exploitText.value = `Exploitability: ${exp.toFixed(3)} (${perText})`;
          if (exp <= target) {
            terminate.value = true;
          }
          if (terminate.value) {
            break;
          }
        }
      }

      await handler.normalize();
      const ev0 = (await handler.ev(0)) + initialPot / 2;
      const ev1 = (await handler.ev(1)) + initialPot / 2;
      evText.value = `EV: ${ev0.toFixed(3)} vs ${ev1.toFixed(3)}`;

      const end = performance.now();
      elapsedTime += end - startTime;
      timeText.value = `Time: ${(elapsedTime / 1000).toFixed(2)}s`;

      isSolverRunning.value = false;
      hasSolverFinished.value = true;
    };

    return {
      numThreads,
      isTreeBuilt,
      treeStatus,
      enableCompression,
      memUsage,
      memUsageCompressed,
      maxMemUsage,
      terminate,
      pause,
      hasSolverRun,
      isSolverRunning,
      hasSolverFinished,
      iterText,
      exploitText,
      evText,
      timeText,
      buildTree,
      runSolver,
      resumeSolver,
    };
  },
});
</script>
