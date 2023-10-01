<template>
  <div class="flex my-1 items-center">
    Number of threads:
    <input
      v-model="numThreads"
      type="number"
      :class="
        'w-20 ml-2 px-2 py-1 rounded-lg text-sm text-center ' +
        (numThreads < 1 ||
        numThreads > (isSafari ? 1 : 64) ||
        numThreads % 1 !== 0
          ? 'input-error'
          : '')
      "
      min="1"
      max="64"
    />
    <button
      class="ml-3 button-base button-blue"
      :disabled="
        isTreeBuilding ||
        store.isSolverRunning ||
        store.isFinalizing ||
        numThreads < 1 ||
        numThreads > (isSafari ? 1 : 64) ||
        numThreads % 1 !== 0
      "
      @click="buildTree"
    >
      Build New Tree
    </button>
  </div>

  <div class="my-1">Status: {{ treeStatus }}</div>

  <div v-if="isTreeBuilt" class="mt-3">
    <div>
      Precision mode:
      <Tippy
        class="inline-block cursor-help"
        max-width="500px"
        placement="bottom"
        trigger="mouseenter click"
        :delay="[200, 0]"
        :interactive="true"
      >
        <QuestionMarkCircleIcon class="w-5 h-5 text-gray-600" />
        <template #content>
          <div class="px-1 py-0.5 text-justify">
            Precision mode mainly affects memory usage, but it also has several
            other effects.
            <ul class="pl-6 list-disc">
              <li class="mt-1">
                32-bit FP (floating-point): This is recommended if the memory
                usage is below the limit (= 3.9GB). It has about 7 significant
                digits and better performance.
              </li>
              <li class="mt-1">
                16-bit integer: This setting can help to avoid the memory limit
                if the 32-bit FP mode is not usable. Since the significant
                digits are about 4 digits, it is not suitable for satisfying an
                exploitability target below 0.1%. Performance is also worse than
                in 32-bit FP mode.
              </li>
            </ul>
          </div>
        </template>
      </Tippy>
    </div>
    <div class="mt-1 ml-2">
      <label :class="{ 'cursor-pointer': !store.hasSolverRun }">
        <input
          v-model="isCompressionEnabled"
          class="mr-2 cursor-pointer disabled:cursor-default"
          type="radio"
          name="compression"
          :value="false"
          :disabled="store.hasSolverRun"
        />
        <span class="inline-block w-[6.75rem] ml-1">32-bit FP:</span>
        needs
        {{
          memoryUsage >= 1023.5 * 1024 * 1024
            ? (memoryUsage / (1024 * 1024 * 1024)).toFixed(2) + "GB"
            : (memoryUsage / (1024 * 1024)).toFixed(0) + "MB"
        }}
        RAM
        {{ memoryUsage > maxMemoryUsage ? "(limit exceeded)" : "" }}
      </label>
    </div>
    <div class="ml-2">
      <label :class="{ 'cursor-pointer': !store.hasSolverRun }">
        <input
          v-model="isCompressionEnabled"
          class="mr-2 cursor-pointer disabled:cursor-default"
          type="radio"
          name="compression"
          :value="true"
          :disabled="store.hasSolverRun"
        />
        <span class="inline-block w-[6.75rem] ml-1">16-bit integer:</span>
        needs
        {{
          memoryUsageCompressed >= 1023.5 * 1024 * 1024
            ? (memoryUsageCompressed / (1024 * 1024 * 1024)).toFixed(2) + "GB"
            : (memoryUsageCompressed / (1024 * 1024)).toFixed(0) + "MB"
        }}
        RAM
        {{ memoryUsageCompressed > maxMemoryUsage ? "(limit exceeded)" : "" }}
      </label>
    </div>
    <div v-if="memoryUsage > maxMemoryUsage" class="mt-1.5">
      RAM limit: 3.9GB (= 4GB Wasm limit - 0.1GB margin)
    </div>

    <div class="mt-4">
      Target exploitability:
      <Tippy
        class="inline-block cursor-help"
        max-width="500px"
        placement="bottom"
        trigger="mouseenter click"
        :delay="[200, 0]"
        :interactive="true"
      >
        <QuestionMarkCircleIcon class="w-5 h-5 text-gray-600" />
        <template #content>
          <div class="px-1 py-0.5 text-justify">
            <div>
              Specifies the acceptable distance to the Nash equilibrium. A lower
              value gives more accurate results, but also requires more
              computation time.
            </div>
            <div class="mt-3">
              <span class="underline">A more detailed description:</span>
              When a Nash equilibrium solution is obtained, the strategies of
              both players become MESs (Maximally Exploitative Strategies) to
              each other. Using this property, we define the distance to the
              Nash equilibrium of the obtained strategy as follows:
            </div>
            <div class="my-1 text-center">
              Distance = (Opponent's MES EV) - (Opponent's obtained EV).
            </div>
            <div>
              This distance is always non-negative and is zero if and only if
              the obtained strategy is a part of a particular Nash equilibrium.
              Exploitability is defined as the average distance of both players.
            </div>
          </div>
        </template>
      </Tippy>
      <input
        v-model="targetExploitability"
        type="number"
        :class="
          'w-20 ml-3 px-2 py-1 rounded-lg text-sm text-center ' +
          (targetExploitability <= 0 ? 'input-error' : '')
        "
        :disabled="store.hasSolverRun && !store.isSolverPaused"
        min="0"
        step="0.05"
      />
      %
    </div>

    <div class="mt-1">
      Maximum number of iterations:
      <input
        v-model="maxIterations"
        type="number"
        :class="
          'w-[5.5rem] ml-2 px-2 py-1 rounded-lg text-sm text-center ' +
          (maxIterations < 0 ||
          maxIterations % 1 !== 0 ||
          maxIterations > 100000
            ? 'input-error'
            : '')
        "
        :disabled="store.hasSolverRun && !store.isSolverPaused"
        min="0"
        max="100000"
      />
    </div>

    <div class="flex mt-6 gap-3">
      <button
        class="button-base button-blue"
        :disabled="
          store.hasSolverRun ||
          memoryUsageSelected > maxMemoryUsage ||
          targetExploitability <= 0 ||
          maxIterations < 0 ||
          maxIterations % 1 !== 0 ||
          maxIterations > 100000
        "
        @click="runSolver"
      >
        Run Solver
      </button>
      <button
        class="button-base button-red"
        :disabled="!store.isSolverRunning"
        @click="() => (terminateFlag = true)"
      >
        Stop
      </button>
      <button
        v-if="!store.isSolverPaused"
        class="button-base button-green"
        :disabled="!store.isSolverRunning"
        @click="() => (pauseFlag = true)"
      >
        Pause
      </button>
      <button
        v-else
        class="button-base button-green"
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
    </div>

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
import { init, handler } from "../global-worker";
import {
  useStore,
  useConfigStore,
  useTmpConfigStore,
  saveConfig,
  saveConfigTmp,
} from "../store";
import {
  MAX_AMOUNT,
  convertBetString,
  ROOT_LINE_STRING,
  INVALID_LINE_STRING,
  readableLineString,
} from "../utils";
import { detect } from "detect-browser";

import { Tippy } from "vue-tippy";
import { QuestionMarkCircleIcon } from "@heroicons/vue/20/solid";

const maxMemoryUsage = 3.9 * 1024 * 1024 * 1024; // 3.9 GB
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

  if (config.startingPot > MAX_AMOUNT) {
    return "Starting pot is too large";
  }

  if (config.startingPot % 1 !== 0) {
    return "Starting pot must be an integer";
  }

  if (config.effectiveStack <= 0) {
    return "Effective stack must be positive";
  }

  if (config.effectiveStack > MAX_AMOUNT) {
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

  if (
    config.expectedBoardLength > 0 &&
    config.board.length !== config.expectedBoardLength
  ) {
    return `Invalid board (expected ${config.expectedBoardLength} cards)`;
  }

  const addedLinesArray =
    config.addedLines === ""
      ? []
      : config.addedLines.split(",").map(readableLineString);

  const removedLinesArray =
    config.removedLines === ""
      ? []
      : config.removedLines.split(",").map(readableLineString);

  if (
    addedLinesArray.includes(ROOT_LINE_STRING) ||
    addedLinesArray.includes(INVALID_LINE_STRING) ||
    removedLinesArray.includes(ROOT_LINE_STRING) ||
    removedLinesArray.includes(INVALID_LINE_STRING)
  ) {
    return "Invalid line found (loaded broken configurations?)";
  }

  if (
    ![0, 3, 4, 5].includes(config.expectedBoardLength) ||
    (config.expectedBoardLength === 0 &&
      (addedLinesArray.length > 0 || removedLinesArray.length > 0)) ||
    (config.expectedBoardLength > 0 &&
      addedLinesArray.length === 0 &&
      removedLinesArray.length === 0)
  ) {
    return "Invalid configurations (loaded broken configurations?)";
  }

  return null;
};

export default defineComponent({
  components: {
    Tippy,
    QuestionMarkCircleIcon,
  },

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

    const memoryUsageSelected = computed(() => {
      if (isCompressionEnabled.value) {
        return memoryUsageCompressed.value;
      } else {
        return memoryUsage.value;
      }
    });

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

      const configError = checkConfig(config);
      if (configError !== null) {
        treeStatus.value = `Error: ${configError}`;
        return;
      }

      saveConfigTmp();
      isTreeBuilding.value = true;
      store.isSolverPaused = false;
      store.isSolverFinished = false;
      treeStatus.value = "Building tree...";

      await init(numThreads.value);
      if (!handler) return;

      const errorString = await handler.init(
        tmpConfig.rangeRaw[0],
        tmpConfig.rangeRaw[1],
        new Uint8Array(tmpConfig.board),
        tmpConfig.startingPot,
        tmpConfig.effectiveStack,
        tmpConfig.rakePercent / 100,
        tmpConfig.rakeCap,
        tmpConfig.donkOption,
        convertBetString(tmpConfig.oopFlopBet),
        convertBetString(tmpConfig.oopFlopRaise),
        convertBetString(tmpConfig.oopTurnBet),
        convertBetString(tmpConfig.oopTurnRaise),
        tmpConfig.donkOption ? convertBetString(tmpConfig.oopTurnDonk) : "",
        convertBetString(tmpConfig.oopRiverBet),
        convertBetString(tmpConfig.oopRiverRaise),
        tmpConfig.donkOption ? convertBetString(tmpConfig.oopRiverDonk) : "",
        convertBetString(tmpConfig.ipFlopBet),
        convertBetString(tmpConfig.ipFlopRaise),
        convertBetString(tmpConfig.ipTurnBet),
        convertBetString(tmpConfig.ipTurnRaise),
        convertBetString(tmpConfig.ipRiverBet),
        convertBetString(tmpConfig.ipRiverRaise),
        tmpConfig.addAllInThreshold / 100,
        tmpConfig.forceAllInThreshold / 100,
        tmpConfig.mergingThreshold / 100,
        tmpConfig.addedLines,
        tmpConfig.removedLines
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
    };

    const runSolver = async () => {
      if (!handler) return;

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
      if (!handler) return;

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
      memoryUsageSelected,
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
