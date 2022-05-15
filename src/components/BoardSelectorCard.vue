<template>
  <div
    :class="
      'relative w-10 h-14 rounded-lg border shadow cursor-pointer ' +
      '' +
      (store.board.includes(cardId)
        ? 'bg-yellow-200 border-red-600'
        : 'border-black')
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
import { useStore } from "../store";

const ranks = [
  "2",
  "3",
  "4",
  "5",
  "6",
  "7",
  "8",
  "9",
  "10",
  "J",
  "Q",
  "K",
  "A",
];

const suits = ["♣", "♦", "♥", "♠"];

export default defineComponent({
  props: {
    cardId: {
      type: Number,
      required: true,
    },
  },

  setup(props) {
    const store = useStore();
    const rank = ranks[Math.floor(props.cardId / 4)];
    const suit = suits[props.cardId % 4];

    let colorClass;
    switch (props.cardId % 4) {
      case 0:
        colorClass = "text-green-600";
        break;
      case 1:
        colorClass = "text-blue-600";
        break;
      case 2:
        colorClass = "text-pink-600";
        break;
      case 3:
        colorClass = "text-black";
        break;
    }

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
