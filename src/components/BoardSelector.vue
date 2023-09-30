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
    <input
      v-model="boardText"
      type="text"
      class="w-40 px-2 py-1 rounded-lg text-sm"
      @focus="($event.target as HTMLInputElement).select()"
      @change="onBoardTextChange"
    />
    <button class="button-base button-blue" @click="clearBoard">Clear</button>
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
    class="mt-5 text-orange-500 font-semibold"
  >
    <span class="underline">Warning:</span>
    The edited tree assumes a {{ config.expectedBoardLength }}-card board.
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import { useConfigStore } from "../store";
import { cardText, parseCardString } from "../utils";

import BoardSelectorCard from "./BoardSelectorCard.vue";

export default defineComponent({
  components: {
    BoardSelectorCard,
  },

  setup() {
    const config = useConfigStore();
    const boardText = ref("");

    const toggleCard = (cardId: number, updateText = true) => {
      if (config.board.includes(cardId)) {
        config.board = config.board.filter((card) => card !== cardId);
      } else if (config.board.length < 5) {
        config.board.push(cardId);
        if (config.board.length <= 3) {
          config.board.sort((a, b) => b - a);
        }
      }

      if (updateText) {
        setBoardTextFromButtons();
      }
    };

    const setBoardTextFromButtons = () => {
      boardText.value = config.board
        .map(cardText)
        .map(({ rank, suitLetter }) => rank + suitLetter)
        .join(", ");
    };

    const onBoardTextChange = () => {
      config.board = [];

      const cardIds = boardText.value
        // Allow pasting in things like [Ah Kd Qc], by reformatting to Ah,Kd,Qc
        .trim()
        .replace(/[^A-Za-z0-9\s,]/g, "")
        .replace(/\s+/g, ",")
        .split(",")
        .map(parseCardString)
        .filter((cardId): cardId is number => cardId !== null);

      new Set(cardIds).forEach((cardId) => toggleCard(cardId, false));
      setBoardTextFromButtons();
    };

    const clearBoard = () => {
      config.board = [];
      setBoardTextFromButtons();
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
      setBoardTextFromButtons();
    };

    return {
      config,
      boardText,
      toggleCard,
      onBoardTextChange,
      clearBoard,
      generateRandomBoard,
    };
  },
});
</script>
