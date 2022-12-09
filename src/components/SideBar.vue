<template>
  <aside
    class="flex flex-col shrink-0 w-56 my-4 overflow-y-auto border-r-2"
    style="height: calc(100vh - 5.5rem)"
  >
    <button :class="itemStyle('About')" @click="store.sideView = 'About'">
      About
    </button>

    <button :class="itemStyle('OOPRange')" @click="store.sideView = 'OOPRange'">
      OOP Range
      <span class="flex my-2 justify-center">
        <RangeMiniViewer :player="0" />
      </span>
    </button>

    <button :class="itemStyle('IPRange')" @click="store.sideView = 'IPRange'">
      IP Range
      <span class="flex my-2 justify-center">
        <RangeMiniViewer :player="1" />
      </span>
    </button>

    <button :class="itemStyle('Board')" @click="store.sideView = 'Board'">
      Board
      <span class="flex mt-1 justify-center font-bold">
        <span
          v-for="item in boardTexts"
          :key="item.rank + item.suit"
          :class="'inline-block mx-0.5 ' + item.colorClass"
        >
          {{ item.rank + item.suit }}
        </span>
      </span>
    </button>

    <button
      :class="itemStyle('TreeConfig')"
      @click="store.sideView = 'TreeConfig'"
    >
      Tree Configuration
    </button>

    <button
      :class="itemStyle('RunSolver')"
      @click="store.sideView = 'RunSolver'"
    >
      Run Solver
    </button>
  </aside>
</template>

<script lang="ts">
import { computed, defineComponent } from "vue";
import { SideView, useStore, useConfigStore } from "../store";
import { cardText } from "../utils";

import RangeMiniViewer from "./RangeMiniViewer.vue";

export default defineComponent({
  components: {
    RangeMiniViewer,
  },

  setup() {
    const store = useStore();
    const config = useConfigStore();

    const boardTexts = computed(() => {
      if (config.board.length === 0) {
        return [{ rank: "-", suit: "", colorClass: "text-black" }];
      } else {
        return config.board.map(cardText);
      }
    });

    return {
      store,
      boardTexts,
      itemStyle: (view: SideView) => {
        return (
          "side-bar-item " +
          (view === store.sideView ? "font-bold bg-blue-100" : "")
        );
      },
    };
  },
});
</script>

<style scoped>
.side-bar-item {
  @apply block mx-2 my-1 px-4 py-3 rounded-3xl;
  @apply text-left text-[1.0625rem] select-none;
  @apply transition-colors hover:bg-blue-100;
}
</style>
