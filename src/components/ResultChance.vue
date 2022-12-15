<template>
  <div class="flex w-full h-full">
    <div class="flex flex-col h-full pt-2 gap-[1%]" style="flex: 5">
      <div
        v-for="suit in 4"
        :key="suit"
        class="flex shrink-0 w-full justify-center gap-[1%]"
      >
        <div class="w-[5.5%]"></div>
        <BoardSelectorCard
          v-for="rank in 13"
          :key="rank"
          class="disabled:opacity-75 disabled:brightness-75"
          font-size="max(1.2vw, 13px)"
          width="5.5%"
          :card-id="56 - 4 * rank - suit"
          :disabled="selectedChance.cards[56 - 4 * rank - suit].isDead"
          :is-selected="selectedChance.selectedIndex === 56 - 4 * rank - suit"
          @click="deal(56 - 4 * rank - suit)"
        />
      </div>

      <div
        ref="chartParentDiv"
        class="flex flex-grow w-full max-h-[50%] mt-3 justify-center"
      >
        <div
          v-if="chartParentDivHeight >= 180"
          class="flex w-[90%] h-full items-center justify-center"
        >
          Graph here
        </div>
      </div>
    </div>

    <div class="bg-gray-300" style="flex: 4"></div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, watch } from "vue";
import { ChanceReports, SpotChance } from "../result-types";

import BoardSelectorCard from "./BoardSelectorCard.vue";

export default defineComponent({
  components: {
    BoardSelectorCard,
  },

  props: {
    selectedChance: {
      type: Object as () => SpotChance,
      required: true,
    },
    chanceReports: {
      type: Object as () => ChanceReports | null,
      required: true,
    },
  },

  emits: {
    "deal-card": (_card: number) => true,
  },

  setup(_, context) {
    const chartParentDiv = ref<HTMLDivElement | null>(null);
    const chartParentDivHeight = ref(0);

    const assignChartParentDivHeight = () => {
      if (chartParentDiv.value) {
        chartParentDivHeight.value = chartParentDiv.value.clientHeight;
      }
    };

    watch(chartParentDiv, assignChartParentDivHeight);
    window.addEventListener("resize", assignChartParentDivHeight);

    const deal = (card: number) => {
      context.emit("deal-card", card);
    };

    return {
      chartParentDiv,
      chartParentDivHeight,
      deal,
    };
  },
});
</script>
