<template>
  <div
    :class="
      'relative w-10 h-14 rounded-lg border shadow cursor-pointer select-none ' +
      (store.board.includes(cardId)
        ? 'bg-yellow-300 ring-1 ring-red-600 border-red-600'
        : 'bg-white border-black')
    "
    @click="toggleCard"
  >
    <div :class="'absolute top-0.5 left-1.5 text-xl font-bold ' + colorClass">
      {{ rank }}
    </div>
    <div :class="'absolute bottom-0.5 right-1 ' + colorClass">
      {{ suit }}
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { cardText, useStore } from "../store";

export default defineComponent({
  props: {
    cardId: {
      type: Number,
      required: true,
    },
  },

  setup(props) {
    const store = useStore();

    const { rank, suit, colorClass } = cardText(props.cardId);

    const toggleCard = () => {
      if (store.board.includes(props.cardId)) {
        store.board = store.board.filter((card) => card !== props.cardId);
      } else if (store.board.length < 5) {
        store.board.push(props.cardId);
        store.board.sort((a, b) => b - a);
      }
    };

    return {
      store,
      rank,
      suit,
      colorClass,
      toggleCard,
    };
  },
});
</script>
