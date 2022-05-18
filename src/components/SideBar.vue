<template>
  <aside
    class="flex flex-col w-52 my-4 overflow-y-auto border-r-2"
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
      <p class="flex justify-center">
        <span
          v-for="item in boardTexts"
          :key="item.text"
          :class="'inline-block mx-px ' + item.class"
        >
          {{ item.text }}
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
import { useStore, MainView } from "../store";

const ranks = [
  "2",
  "3",
  "4",
  "5",
  "6",
  "7",
  "8",
  "9",
  "10",
  "J",
  "Q",
  "K",
  "A",
];

const suits = ["♣", "♦", "♥", "♠"];

export default defineComponent({
  setup() {
    const store = useStore();

    const boardTexts = computed(() => {
      const texts = [];

      for (let i = 0; i < store.board.length; i++) {
        const rank = ranks[Math.floor(store.board[i] / 4)];
        const suit = suits[store.board[i] % 4];

        let colorClass;
        switch (store.board[i] % 4) {
          case 0:
            colorClass = "text-green-600";
            break;
          case 1:
            colorClass = "text-blue-600";
            break;
          case 2:
            colorClass = "text-pink-600";
            break;
          case 3:
            colorClass = "text-black";
            break;
        }

        texts.push({ text: `${rank}${suit}`, class: colorClass });
      }

      if (texts.length === 0) {
        texts.push({ text: "-", class: "text-black" });
      }

      return texts;
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
  @apply mx-2 my-1 px-2 py-1 border-2 rounded-md cursor-pointer;
}
</style>
