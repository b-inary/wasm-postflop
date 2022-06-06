<template>
  <aside
    class="flex flex-col shrink-0 w-52 my-4 overflow-y-auto border-r-2"
    style="height: calc(100vh - 5.5rem)"
  >
    <div :class="itemStyle('About')" @click="store.mainView = 'About'">
      About
    </div>

    <div :class="itemStyle('OOPRange')" @click="store.mainView = 'OOPRange'">
      Range 1 (OOP)
      <div class="flex my-2 justify-center">
        <range-mini-viewer :player="0" />
      </div>
    </div>

    <div :class="itemStyle('IPRange')" @click="store.mainView = 'IPRange'">
      Range 2 (IP)
      <div class="flex my-2 justify-center">
        <range-mini-viewer :player="1" />
      </div>
    </div>

    <div :class="itemStyle('Board')" @click="store.mainView = 'Board'">
      <p>Board</p>
      <p class="flex justify-center">
        <span
          v-for="item in boardTexts"
          :key="item.rank + item.suit"
          :class="'inline-block mx-px ' + item.colorClass"
        >
          {{ item.rank + item.suit }}
        </span>
      </p>
    </div>

    <div
      :class="itemStyle('TreeConfig')"
      @click="store.mainView = 'TreeConfig'"
    >
      Tree Configuration
    </div>

    <div :class="itemStyle('RunSolver')" @click="store.mainView = 'RunSolver'">
      Run Solver
    </div>

    <div :class="itemStyle('Result')" @click="store.mainView = 'Result'">
      Result
    </div>
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
  @apply mx-2 my-1 px-2.5 py-1.5 border-[3px] rounded-xl cursor-pointer select-none;
}
</style>
