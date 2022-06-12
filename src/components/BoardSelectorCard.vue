<template>
  <button
    :class="
      'relative w-10 h-14 rounded-lg border shadow select-none ' +
      (isSelected
        ? 'bg-yellow-300 ring-1 ring-red-600 border-red-600'
        : 'bg-white border-black')
    "
  >
    <span :class="'absolute top-0.5 left-1.5 text-xl font-bold ' + colorClass">
      {{ rank }}
    </span>
    <span :class="'absolute bottom-0.5 right-1 ' + colorClass">
      {{ suit }}
    </span>
  </button>
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
    isSelected: {
      type: Boolean,
      default: false,
    },
  },

  setup(props) {
    const store = useStore();

    const { rank, suit, colorClass } = cardText(props.cardId);

    return {
      store,
      rank,
      suit,
      colorClass,
    };
  },
});
</script>
