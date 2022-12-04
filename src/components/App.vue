<template>
  <nav-bar />

  <div class="w-full mx-auto max-w-screen-xl">
    <div v-show="store.navView === 'Solver'" class="flex">
      <side-bar />

      <div
        class="flex-grow my-4 px-6 pt-2 overflow-y-auto"
        style="height: calc(100vh - 4.5rem)"
      >
        <div class="flex">
          <div
            :class="
              'mb-5 pl-2 pr-3 pb-0.5 text-lg font-bold border-l-8 border-b-2 ' +
              'border-blue-600 rounded rounded-br-none'
            "
          >
            {{ header }}
          </div>
        </div>

        <div v-if="store.sideView === 'About'">
          <about-page />
        </div>
        <div v-show="store.sideView === 'OOPRange'">
          <range-editor :player="0" />
        </div>
        <div v-show="store.sideView === 'IPRange'">
          <range-editor :player="1" />
        </div>
        <div v-show="store.sideView === 'Board'">
          <board-selector />
        </div>
        <div v-show="store.sideView === 'TreeConfig'">
          <tree-config />
        </div>
        <div v-show="store.sideView === 'RunSolver'">
          <run-solver />
        </div>
      </div>
    </div>

    <div
      v-show="store.navView === 'Results'"
      class="my-4 px-6 pt-2 overflow-y-auto"
      style="height: calc(100vh - 4.5rem)"
    >
      <result-viewer />
    </div>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent } from "vue";
import { useStore } from "../store";

import AboutPage from "./AboutPage.vue";
import BoardSelector from "./BoardSelector.vue";
import NavBar from "./NavBar.vue";
import RangeEditor from "./RangeEditor.vue";
import ResultViewer from "./ResultViewer.vue";
import RunSolver from "./RunSolver.vue";
import SideBar from "./SideBar.vue";
import TreeConfig from "./TreeConfig.vue";

export default defineComponent({
  components: {
    AboutPage,
    BoardSelector,
    NavBar,
    RangeEditor,
    ResultViewer,
    RunSolver,
    SideBar,
    TreeConfig,
  },

  setup() {
    const store = useStore();
    const header = computed(
      () =>
        ({
          About: "Welcome to WASM Postflop!",
          OOPRange: "OOP Range",
          IPRange: "IP Range",
          Board: "Board",
          TreeConfig: "Tree Configuration",
          RunSolver: "Run Solver",
        }[store.sideView])
    );

    return {
      store,
      header,
    };
  },
});
</script>
