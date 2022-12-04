<template>
  <nav
    class="sticky flex top-0 z-30 w-full h-10 shadow-lg px-4 justify-center bg-slate-800 text-gray-50"
  >
    <div class="flex relative w-full">
      <div class="flex items-center z-10">
        <a href="#" class="px-4 text-lg font-semibold">WASM Postflop</a>
      </div>

      <div
        class="flex absolute w-full h-full left-0 top-0 gap-3 justify-center"
      >
        <button
          :class="
            'relative px-5 font-semibold transition-colors ' +
            'hover:bg-slate-700 hover:text-blue-200 ' +
            (store.navView === 'Solver' ? 'bg-slate-700 text-blue-200' : '')
          "
          @click="store.navView = 'Solver'"
        >
          <div class="solver-icon"></div>
          <span class="pl-8">Solver</span>
        </button>
        <button
          :class="
            'relative px-5 font-semibold transition-colors ' +
            'hover:bg-slate-700 hover:text-blue-200 ' +
            (store.navView === 'Results' ? 'bg-slate-700 text-blue-200' : '')
          "
          @click="store.navView = 'Results'"
        >
          <div class="result-icon"></div>
          <span class="pl-8">Results</span>
        </button>
      </div>

      <div class="flex absolute right-0 h-full items-center">
        <a
          href="https://github.com/b-inary/wasm-postflop"
          class="flex px-4 h-full items-center font-semibold hover:bg-slate-700"
        >
          <img
            src="../assets/GitHub-Mark-Light-32px.png"
            class="inline-block w-5 h-5 mr-2.5"
          />
          GitHub
        </a>
      </div>
    </div>
  </nav>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { useStore } from "../store";

export default defineComponent({
  setup() {
    return {
      store: useStore(),
    };
  },
});
</script>

<style scoped>
.solver-icon {
  position: absolute;
  margin-top: 4px;
  width: 20px;
  height: 14px;
  border-radius: 3px;
  border: solid 2px currentColor;
}

.solver-icon:before {
  content: "";
  position: absolute;
  left: 7px;
  bottom: -6px;
  width: 2px;
  height: 5px;
  background-color: currentColor;
}

.solver-icon:after {
  content: "";
  position: absolute;
  left: 2px;
  bottom: -7px;
  width: 12px;
  height: 2px;
  background-color: currentColor;
}

.result-icon {
  position: absolute;
  margin-left: 7px;
  margin-top: 4px;
  width: 5px;
  height: 19px;
  background-color: currentColor;
}

.result-icon:before {
  content: "";
  position: absolute;
  right: 7px;
  bottom: 0;
  width: 5px;
  height: 13px;
  background-color: currentColor;
}

.result-icon:after {
  content: "";
  position: absolute;
  left: 7px;
  bottom: 0;
  width: 5px;
  height: 10px;
  background-color: currentColor;
}
</style>
