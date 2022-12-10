<template>
  <div
    v-if="!store.isSolverFinished"
    class="w-full max-w-screen-xl mx-auto px-4 py-6"
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
      :cards-length="cardsLength"
      @update:is-handler-updated="(value) => (isHandlerUpdated = value)"
      @update:is-locked="(value) => (isLocked = value)"
      @trigger-update="onUpdateSpot"
    />

    <ResultMiddle
      :display-mode="displayMode"
      :stats-mode="statsMode"
      :copy-success="copySuccess"
      @update:display-mode="updateDisplayMode"
      @update:display-options="updateDisplayOptions"
      @copy-to-clipboard="copyRangeTextToClipboard"
      @reset-copy-success="resetCopySuccess"
    />
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, ref } from "vue";
import { useStore, useSavedConfigStore } from "../store";
import { handler, memory } from "../global-worker";

import ResultNav from "./ResultNav.vue";
import ResultMiddle from "./ResultMiddle.vue";

import { DisplayMode, DisplayOptions } from "../result-types";

export default defineComponent({
  components: {
    ResultNav,
    ResultMiddle,
  },

  setup() {
    const store = useStore();
    const config = useSavedConfigStore();

    /* Navigation */

    const isHandlerUpdated = ref(false);
    const isLocked = ref(false);

    const cards = ref([new Uint16Array(), new Uint16Array()]);
    const cardsLength = computed(() =>
      cards.value.map((cards) => cards.length)
    );

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

      cards.value = [];
      for (let player = 0; player < 2; ++player) {
        const cardsBuffer = await handler.privateCards(player);
        cards.value.push(
          new Uint16Array(
            memoryBuffer,
            cardsBuffer.ptr,
            cardsBuffer.byteLength / 2
          )
        );
      }

      isHandlerUpdated.value = true;
    };

    const clear = () => {
      cards.value = [new Uint16Array(), new Uint16Array()];
    };

    const onUpdateSpot = () => {
      isLocked.value = false;
    };

    /* Middle Bar */

    const displayMode = ref("basics");
    const statsMode = ref("");

    const updateDisplayMode = (mode: DisplayMode) => {
      displayMode.value = mode;
    };

    const displayOptions = ref<DisplayOptions>({
      player: "auto",
      barHeight: "normalized",
      suit: "grouped",
      strategy: "show",
      content: "default",
    });

    const copySuccess = ref(0);

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

    return {
      store,
      config,
      isHandlerUpdated,
      isLocked,
      cardsLength,
      onUpdateSpot,
      displayMode,
      statsMode,
      updateDisplayMode,
      updateDisplayOptions,
      copySuccess,
      copyRangeTextToClipboard,
      resetCopySuccess,
    };
  },
});
</script>
