<template>
  <div class="min-w-[1080px]" :style="{ height: clientHeight + 'px' }">
    <NavBar />

    <div
      v-show="store.navView === 'Solver'"
      class="flex w-full mx-auto max-w-screen-xl"
      style="height: calc(100% - 2.5rem)"
    >
      <SideBar style="height: calc(100% - 2rem)" />

      <div
        class="flex-grow my-4 px-6 pt-2 overflow-y-auto"
        style="height: calc(100% - 2rem)"
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
          <AboutPage />
        </div>
        <div v-show="store.sideView === 'OOPRange'">
          <RangeEditor :player="0" />
        </div>
        <div v-show="store.sideView === 'IPRange'">
          <RangeEditor :player="1" />
        </div>
        <div v-show="store.sideView === 'Board'">
          <BoardSelector />
        </div>
        <div v-show="store.sideView === 'TreeConfig'">
          <TreeConfig />
        </div>
        <div v-show="store.sideView === 'RunSolver'">
          <RunSolver />
        </div>
      </div>
    </div>

    <div
      v-show="store.navView === 'Results'"
      style="height: calc(max(100%, 720px) - 2.5rem)"
    >
      <ResultViewer />
    </div>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, ref } from "vue";
import { useStore } from "../store";

import NavBar from "./NavBar.vue";
import SideBar from "./SideBar.vue";
import AboutPage from "./AboutPage.vue";
import RangeEditor from "./RangeEditor.vue";
import BoardSelector from "./BoardSelector.vue";
import TreeConfig from "./TreeConfig.vue";
import RunSolver from "./RunSolver.vue";
import ResultViewer from "./ResultViewer.vue";

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

    const clientHeight = ref(document.documentElement.clientHeight - 0.01);
    window.addEventListener("resize", () => {
      clientHeight.value = document.documentElement.clientHeight - 0.01;
    });

    return {
      store,
      header,
      clientHeight,
    };
  },
});
</script>
