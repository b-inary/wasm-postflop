<template>
  <table class="bg-gray-200 shadow">
    <tr v-for="row in 13" :key="row" class="h-2.5">
      <td
        v-for="col in 13"
        :key="col"
        class="relative w-2.5 border border-gray-500"
      >
        <div
          class="absolute bottom-0 left-0 w-full bg-yellow-300"
          :style="{ height: weightWidth(row, col) }"
        ></div>
      </td>
    </tr>
  </table>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { useConfigStore } from "../store";

export default defineComponent({
  props: {
    player: {
      type: Number,
      required: true,
    },
  },

  setup(props) {
    const config = useConfigStore();

    const range = config.range[props.player];

    const weightWidth = (row: number, col: number) => {
      return range[13 * (row - 1) + col - 1] + "%";
    };

    return {
      weightWidth,
    };
  },
});
</script>
