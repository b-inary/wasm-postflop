<template>
  <aside
    class="flex flex-col w-48 my-4 overflow-y-auto border-r"
    style="height: calc(100vh - 88px)"
  >
    <div :class="itemStyle('OOPRange')" @click="store.mainView = 'OOPRange'">
      Range 1 (OOP)
    </div>

    <div :class="itemStyle('IPRange')" @click="store.mainView = 'IPRange'">
      Range 2 (IP)
    </div>

    <div :class="itemStyle('Board')" @click="store.mainView = 'Board'">
      <p>Board</p>
      <p class="text-center">
        <span class="text-blue-600">10♦ </span>
        <span class="text-blue-600">9♦ </span>
        <span class="text-pink-600">6♥</span>
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
import { defineComponent } from "vue";
import { useStore, MainView } from "../store";

export default defineComponent({
  setup() {
    const store = useStore();

    return {
      store,
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
  @apply mx-2;
  @apply my-1;
  @apply px-2;
  @apply py-1;
  @apply border-2;
  @apply rounded-md;
  @apply cursor-pointer;
}
</style>
