<template>
  <table class="shadow-md">
    <tr v-for="row in 13" :key="row" class="h-2.5">
      <td
        v-for="col in 13"
        :key="col"
        class="relative w-2.5 border-[0.5px] border-black"
      >
        <div
          :class="
            'absolute w-full h-full left-0 top-0 ' +
            (row === col ? 'bg-neutral-700' : 'bg-neutral-800')
          "
        >
          <div
            class="absolute w-full h-full left-0 top-0 bg-bottom bg-no-repeat"
            :style="{
              'background-image': `linear-gradient(${yellow500} 0% 100%)`,
              'background-size': `100% ${cellValue(row, col)}%`,
            }"
          ></div>
        </div>
      </td>
    </tr>
  </table>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { useConfigStore } from "../store";

const yellow500 = "#eab308";

export default defineComponent({
  props: {
    player: {
      type: Number,
      required: true,
    },
  },

  setup(props) {
    const config = useConfigStore();

    const cellValue = (row: number, col: number) => {
      const cellIndex = (row - 1) * 13 + (col - 1);
      return config.range[props.player][cellIndex];
    };

    return { yellow500, cellValue };
  },
});
</script>
