<template>
  <div v-for="suit in 4" :key="suit" class="flex">
    <BoardSelectorCard
      v-for="rank in 13"
      :key="rank"
      class="m-1"
      :card-id="56 - 4 * rank - suit"
      :is-selected="config.board.includes(56 - 4 * rank - suit)"
      @click="toggleCard(56 - 4 * rank - suit)"
    />
  </div>

  <div class="flex mt-4 mx-1 gap-3">
    <button class="button-base button-blue" @click="config.board = []">
      Clear
    </button>
    <button class="button-base button-blue" @click="generateRandomBoard">
      Random Flop
    </button>
  </div>

  <div
    v-if="
      config.board.length >= 3 &&
      config.expectedBoardLength > 0 &&
      config.board.length !== config.expectedBoardLength
    "
    class="mt-5 font-bold text-orange-500"
  >
    <span class="underline">Warning:</span>
    The edited tree assumes a {{ config.expectedBoardLength }}-card board.
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { useConfigStore } from "../store";

import BoardSelectorCard from "./BoardSelectorCard.vue";

export default defineComponent({
  components: {
    BoardSelectorCard,
  },

  setup() {
    const config = useConfigStore();

    const toggleCard = (cardId: number) => {
      if (config.board.includes(cardId)) {
        config.board = config.board.filter((card) => card !== cardId);
      } else if (config.board.length < 5) {
        config.board.push(cardId);
        config.board.sort((a, b) => b - a);
      }
    };

    const generateRandomBoard = () => {
      config.board = [];

      while (config.board.length < 3) {
        const randomCard = Math.floor(Math.random() * 52);
        if (!config.board.includes(randomCard)) {
          config.board.push(randomCard);
        }
      }

      config.board.sort((a, b) => b - a);
    };

    return {
      config,
      toggleCard,
      generateRandomBoard,
    };
  },
});
</script>
