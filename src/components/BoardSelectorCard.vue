<template>
  <button
    :class="
      'relative rounded-lg border select-none enabled:shadow ' +
      (isSelected
        ? 'bg-yellow-300 ring-1 ring-red-600 border-red-600'
        : 'bg-white border-black')
    "
    :style="{
      '--width': width,
      '--font-size': fontSize,
      width: 'var(--width)',
      'padding-top': 'calc(var(--width) * 1.4 - 2px)',
    }"
  >
    <span
      :class="'absolute top-0 font-semibold ' + colorClass"
      :style="{
        left: '15%',
        'font-size': 'calc(var(--font-size) * 1.25)',
      }"
    >
      {{ rank }}
    </span>
    <span
      :class="'absolute ' + colorClass"
      :style="{
        bottom: '5%',
        right: '10%',
        'font-size': 'var(--font-size)',
      }"
    >
      {{ suit }}
    </span>
  </button>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { cardText } from "../utils";

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
    width: {
      type: String,
      default: "40px",
    },
    fontSize: {
      type: String,
      default: "1rem",
    },
  },

  setup(props) {
    const { rank, suit, colorClass } = cardText(props.cardId);
    return { rank, suit, colorClass };
  },
});
</script>
