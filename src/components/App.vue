<template>
  <nav-bar />

  <div class="flex flex-row w-full mx-auto max-w-screen-xl">
    <side-bar />

    <main
      class="flex-grow my-4 px-6 py-2 overflow-y-auto"
      style="height: calc(100vh - 88px)"
    >
      <div v-show="store.mainView === 'Board'">
        <board-selector />
      </div>
      <div v-show="store.mainView === 'TreeConfig'">
        <tree-config />
      </div>
      <div v-show="store.mainView === 'RunSolver'">
        <run-solver />
      </div>
    </main>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { useStore } from "../store";

import BoardSelector from "./BoardSelector.vue";
import NavBar from "./NavBar.vue";
import RunSolver from "./RunSolver.vue";
import SideBar from "./SideBar.vue";
import TreeConfig from "./TreeConfig.vue";

export default defineComponent({
  components: {
    BoardSelector,
    NavBar,
    RunSolver,
    SideBar,
    TreeConfig,
  },

  setup() {
    return {
      store: useStore(),
    };
  },
});
</script>
