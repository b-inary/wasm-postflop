<template>
  <table class="bg-white">
    <tr v-for="row in 13" :key="row" class="h-2.5">
      <td
        v-for="col in 13"
        :key="col"
        class="relative w-2.5 border border-black"
      >
        <div
          class="absolute top-0 left-0 h-full bg-yellow-300"
          :style="'width: ' + weightWidth(row, col)"
        ></div>
      </td>
    </tr>
  </table>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { useStore } from "../store";

export default defineComponent({
  props: {
    player: {
      type: Number,
      required: true,
    },
  },

  setup(props) {
    const store = useStore();

    const range = store.range[props.player];

    const weightWidth = (row: number, col: number) => {
      return range[13 * (row - 1) + col - 1] + "%";
    };

    return {
      weightWidth,
    };
  },
});
</script>
