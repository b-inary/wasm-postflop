<template>
  <div
    v-if="!store.isSolverFinished"
    class="flex w-full max-w-screen-xl mx-auto px-4 py-6 items-center"
  >
    <span
      v-if="store.isSolverRunning || store.isFinalizing"
      class="spinner inline-block mr-3"
    ></span>
    {{
      !store.hasSolverRun
        ? "Solver has not run."
        : store.isSolverRunning
        ? "Solver running..."
        : store.isFinalizing
        ? "Finalizing..."
        : "Solver paused."
    }}
  </div>

  <div v-else class="flex flex-col h-full">
    <ResultNav
      :is-handler-updated="isHandlerUpdated"
      :is-locked="isLocked"
      :cards="cards"
      :dealt-card="dealtCard"
      @update:is-handler-updated="(value) => (isHandlerUpdated = value)"
      @update:is-locked="(value) => (isLocked = value)"
      @trigger-update="onUpdateSpot"
    />

    <ResultMiddle
      :display-mode="displayMode"
      :chance-mode="chanceMode"
      :auto-player-basics="autoPlayerBasics"
      :auto-player-chance="autoPlayerChance"
      :copy-success="copySuccess"
      @update:display-mode="updateDisplayMode"
      @update:display-options="updateDisplayOptions"
      @copy-to-clipboard="copyRangeTextToClipboard"
      @reset-copy-success="resetCopySuccess"
    />

    <div class="flex flex-grow">
      <template v-if="displayMode === 'basics'">
        <ResultBasics
          style="flex: 5"
          :cards="cards"
          :selected-spot="selectedSpot"
          :selected-chance="selectedChance"
          :current-board="currentBoard"
          :total-bet-amount="totalBetAmount"
          :results="results"
          :display-options="displayOptions"
          :display-player="displayPlayerBasics"
          :is-compare-mode="false"
        />
        <div class="bg-gray-300" style="flex: 4"></div>
      </template>

      <template v-else-if="displayMode === 'compare'">
        <ResultBasics
          style="flex: 5"
          :cards="cards"
          :selected-spot="selectedSpot"
          :selected-chance="selectedChance"
          :current-board="currentBoard"
          :total-bet-amount="totalBetAmount"
          :results="results"
          :display-options="displayOptions"
          display-player="oop"
          :is-compare-mode="true"
        />
        <div class="bg-gray-300" style="flex: 2"></div>
        <ResultBasics
          style="flex: 5"
          :cards="cards"
          :selected-spot="selectedSpot"
          :selected-chance="selectedChance"
          :current-board="currentBoard"
          :total-bet-amount="totalBetAmount"
          :results="results"
          :display-options="displayOptions"
          display-player="ip"
          :is-compare-mode="true"
        />
      </template>

      <template v-else-if="displayMode === 'chance' && selectedChance">
        <ResultChance
          :selected-chance="selectedChance"
          :chance-reports="chanceReports"
          @deal-card="onDealCard"
        />
      </template>
    </div>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, ref } from "vue";
import { useStore, useSavedConfigStore } from "../store";
import { handler, memory } from "../global-worker";

import {
  Results,
  ChanceReports,
  Spot,
  SpotChance,
  SpotPlayer,
  DisplayMode,
  DisplayOptions,
} from "../result-types";

import ResultNav from "./ResultNav.vue";
import ResultMiddle from "./ResultMiddle.vue";
import ResultBasics from "./ResultBasics.vue";
import ResultChance from "./ResultChance.vue";

export default defineComponent({
  components: {
    ResultNav,
    ResultMiddle,
    ResultBasics,
    ResultChance,
  },

  setup() {
    const store = useStore();
    const config = useSavedConfigStore();

    /* Navigation */

    const isHandlerUpdated = ref(false);
    const isLocked = ref(false);

    const cards = ref([new Uint16Array(), new Uint16Array()]);
    const dealtCard = ref(-1);

    const selectedSpot = ref<Spot | null>(null);
    const selectedChance = ref<SpotChance | null>(null);
    const currentBoard = ref([...config.board]);
    const results = ref<Results | null>(null);
    const chanceReports = ref<ChanceReports | null>(null);
    const totalBetAmount = ref([0, 0]);

    const isSolverFinished = ref(false);
    store.$subscribe(async (_, store) => {
      if (isSolverFinished.value !== store.isSolverFinished) {
        if ((isSolverFinished.value = store.isSolverFinished)) {
          await init();
        } else {
          clear();
        }
      }
    });

    const init = async () => {
      if (!handler || !memory) return;
      const memoryBuffer = await memory.buffer;
      const cardsBuffer = [
        await handler.privateCards(0),
        await handler.privateCards(1),
      ];

      cards.value = Array.from({ length: 2 }, (_, player) => {
        return new Uint16Array(
          memoryBuffer,
          cardsBuffer[player].ptr >>> 0,
          cardsBuffer[player].byteLength / 2
        );
      });

      isHandlerUpdated.value = true;
    };

    const clear = () => {
      cards.value = [new Uint16Array(), new Uint16Array()];
    };

    const onUpdateSpot = (
      newSelectedSpot: Spot | null,
      newSelectedChance: SpotChance | null,
      newCurrentBoard: number[],
      newResults: Results,
      newChanceReports: ChanceReports | null,
      newTotalBetAmount: number[]
    ) => {
      dealtCard.value = -1;
      selectedSpot.value = newSelectedSpot;
      selectedChance.value = newSelectedChance;
      currentBoard.value = newCurrentBoard;
      results.value = newResults;
      chanceReports.value = newChanceReports;
      totalBetAmount.value = newTotalBetAmount;
      isLocked.value = false;

      chanceMode.value = newSelectedChance?.player ?? "";
    };

    /* Middle Bar */

    const displayMode = ref<DisplayMode>("basics");
    const chanceMode = ref("");

    const displayOptions = ref<DisplayOptions>({
      playerBasics: "auto",
      playerChance: "auto",
      barHeight: "normalized",
      suit: "grouped",
      strategy: "show",
      contentBasics: "default",
      contentChance: "strategy",
    });

    const copySuccess = ref(0);

    const updateDisplayMode = (mode: DisplayMode) => {
      displayMode.value = mode;
    };

    const updateDisplayOptions = (options: DisplayOptions) => {
      displayOptions.value = options;
    };

    const copyRangeTextToClipboard = async () => {
      const text = "Hello World";
      navigator.clipboard
        .writeText(text)
        .then(() => (copySuccess.value = 1))
        .catch(() => (copySuccess.value = -1));
    };

    const resetCopySuccess = () => {
      copySuccess.value = 0;
    };

    /* Computed */

    const autoPlayerBasics = computed(() => {
      const spot = selectedSpot.value;
      const chance = selectedChance.value;
      if (!spot) return "oop";

      if (chance) {
        return chance.prevPlayer;
      } else if (spot.type === "terminal") {
        return spot.prevPlayer;
      } else {
        return (spot as SpotPlayer).player;
      }
    });

    const autoPlayerChance = computed(() => {
      const spot = selectedSpot.value;
      if (!spot) return "oop";
      if (spot.type === "terminal") {
        return spot.prevPlayer;
      } else {
        return (spot as SpotPlayer).player;
      }
    });

    const displayPlayerBasics = computed(() => {
      const optionPlayer = displayOptions.value.playerBasics;
      if (optionPlayer === "auto") {
        return autoPlayerBasics.value;
      } else {
        return optionPlayer;
      }
    });

    const displayPlayerChance = computed(() => {
      const optionPlayer = displayOptions.value.playerChance;
      if (optionPlayer === "auto") {
        return autoPlayerChance.value;
      } else {
        return optionPlayer;
      }
    });

    /* Results */

    const onDealCard = (card: number) => {
      dealtCard.value = card;
    };

    return {
      store,
      config,
      isHandlerUpdated,
      isLocked,
      cards,
      dealtCard,
      selectedSpot,
      selectedChance,
      currentBoard,
      results,
      chanceReports,
      totalBetAmount,
      onUpdateSpot,
      displayMode,
      chanceMode,
      displayOptions,
      updateDisplayMode,
      updateDisplayOptions,
      copySuccess,
      copyRangeTextToClipboard,
      resetCopySuccess,
      autoPlayerBasics,
      autoPlayerChance,
      displayPlayerBasics,
      displayPlayerChance,
      onDealCard,
    };
  },
});
</script>
