<template>
  <table class="shadow-md">
    <tr v-for="row in 13" :key="row" class="h-2.5">
      <td
        v-for="col in 13"
        :key="col"
        class="relative w-2.5 border border-black"
      >
        <div
          class="absolute left-0 top-0 w-full h-full"
          :style="{ background: bgImage(row, col) }"
        ></div>
      </td>
    </tr>
  </table>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { useConfigStore } from "../store";

const amber500 = "#f59e0b";
const neutral700 = "#404040";
const neutral800 = "#262626";

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
    const index = (row: number, col: number) => (row - 1) * 13 + (col - 1);

    const bgImage = (row: number, col: number) => {
      const weight = range[index(row, col)];
      const neutral = row === col ? neutral700 : neutral800;
      return `linear-gradient(to top, ${amber500} ${weight}%, ${neutral} ${weight}%)`;
    };

    return { bgImage };
  },
});
</script>
