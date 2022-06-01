<template>
  <div v-for="suit in 4" :key="suit" class="flex">
    <board-selector-card
      v-for="rank in 13"
      :key="rank"
      class="m-1"
      :card-id="56 - 4 * rank - suit"
      :is-selected="store.board.includes(56 - 4 * rank - suit)"
      @click="toggleCard(56 - 4 * rank - suit)"
    />
  </div>

  <div class="mt-4 mx-1">
    <button
      :class="
        'rounded-lg shadow-sm px-3.5 py-1.5 text-white text-sm font-medium ' +
        'bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-4 focus:ring-blue-300'
      "
      @click="store.board = []"
    >
      Clear
    </button>

    <button
      :class="
        'rounded-lg shadow-sm ml-3 px-3.5 py-1.5 text-white text-sm font-medium ' +
        'bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-4 focus:ring-blue-300'
      "
      @click="generateRandomBoard"
    >
      Random
    </button>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { useStore } from "../store";

import BoardSelectorCard from "./BoardSelectorCard.vue";

export default defineComponent({
  components: {
    BoardSelectorCard,
  },

  setup() {
    const store = useStore();

    const toggleCard = (cardId: number) => {
      if (store.board.includes(cardId)) {
        store.board = store.board.filter((card) => card !== cardId);
      } else if (store.board.length < 5) {
        store.board.push(cardId);
        store.board.sort((a, b) => b - a);
      }
    };

    const generateRandomBoard = () => {
      store.board = [];

      while (store.board.length < 3) {
        const randomCard = Math.floor(Math.random() * 52);
        if (!store.board.includes(randomCard)) {
          store.board.push(randomCard);
        }
      }

      store.board.sort((a, b) => b - a);
    };

    return {
      store,
      toggleCard,
      generateRandomBoard,
    };
  },
});
</script>
