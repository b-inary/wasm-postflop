<template>
  <div class="flex">
    <board-selector-card
      v-for="rank in 13"
      :key="4 * rank - 4"
      class="m-1"
      :card-id="55 - 4 * rank"
    />
  </div>

  <div class="flex">
    <board-selector-card
      v-for="rank in 13"
      :key="4 * rank - 3"
      class="m-1"
      :card-id="54 - 4 * rank"
    />
  </div>

  <div class="flex">
    <board-selector-card
      v-for="rank in 13"
      :key="4 * rank - 2"
      class="m-1"
      :card-id="53 - 4 * rank"
    />
  </div>

  <div class="flex">
    <board-selector-card
      v-for="rank in 13"
      :key="4 * rank - 1"
      class="m-1"
      :card-id="52 - 4 * rank"
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
      generateRandomBoard,
    };
  },
});
</script>
