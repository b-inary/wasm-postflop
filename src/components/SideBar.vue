<template>
  <aside
    class="flex flex-col shrink-0 w-52 my-4 overflow-y-auto border-r-2"
    style="height: calc(100vh - 5.5rem)"
  >
    <button :class="itemStyle('About')" @click="store.mainView = 'About'">
      About
    </button>

    <button :class="itemStyle('OOPRange')" @click="store.mainView = 'OOPRange'">
      Range 1 (OOP)
      <span class="flex my-2 justify-center">
        <range-mini-viewer :player="0" />
      </span>
    </button>

    <button :class="itemStyle('IPRange')" @click="store.mainView = 'IPRange'">
      Range 2 (IP)
      <span class="flex my-2 justify-center">
        <range-mini-viewer :player="1" />
      </span>
    </button>

    <button :class="itemStyle('Board')" @click="store.mainView = 'Board'">
      Board
      <span class="flex justify-center">
        <span
          v-for="item in boardTexts"
          :key="item.rank + item.suit"
          :class="'inline-block mx-px ' + item.colorClass"
        >
          {{ item.rank + item.suit }}
        </span>
      </span>
    </button>

    <button
      :class="itemStyle('TreeConfig')"
      @click="store.mainView = 'TreeConfig'"
    >
      Tree Configuration
    </button>

    <button
      :class="itemStyle('RunSolver')"
      @click="store.mainView = 'RunSolver'"
    >
      Run Solver
    </button>

    <button :class="itemStyle('Result')" @click="store.mainView = 'Result'">
      Result
    </button>
  </aside>
</template>

<script lang="ts">
import { computed, defineComponent } from "vue";
import { MainView, cardText, useStore, useConfigStore } from "../store";

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
      itemStyle: (view: MainView) => {
        return (
          "side-bar-item " +
          (view === store.mainView ? "border-blue-600" : "border-transparent")
        );
      },
    };
  },
});
</script>

<style scoped>
.side-bar-item {
  @apply block mx-2 my-1 px-2.5 py-1.5 border-[3px] rounded-xl text-left cursor-pointer select-none;
}
</style>
