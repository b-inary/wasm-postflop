<template>
  <div class="ml-2">
    <div class="flex">
      <table class="bg-white" @mouseleave="mouseLeave">
        <tr v-for="row in 13" :key="row" class="h-9">
          <td
            v-for="col in 13"
            :key="col"
            :class="
              'relative w-10 text-center align-middle border-black cursor-default select-none ' +
              (row === col ? 'border-2' : 'border')
            "
            @mousedown="dragStart(row, col)"
            @mouseup="dragEnd"
            @mouseover="mouseOver(row, col)"
          >
            <div
              class="absolute top-0 left-0 h-full bg-yellow-300"
              :style="'width: ' + weightWidth(row, col)"
            ></div>
            <div class="relative z-10">
              {{ cellText(row, col) }}
            </div>
          </td>
        </tr>
      </table>

      <div class="ml-6 pt-12">
        <span class="inline-block w-9">{{ hoveringCellText }}</span>
        {{ hoveringCellWeight }}
      </div>
    </div>

    <div class="mt-5">
      <input
        v-model="rangeText"
        type="text"
        :class="
          'w-[30rem] px-2 py-1 rounded-lg text-sm ' +
          (rangeTextError ? 'ring-1 ring-red-600 border-red-600 bg-red-50' : '')
        "
        @focus="($event.target as HTMLInputElement).select()"
        @change="onRangeTextChange"
      />
      <div v-if="rangeTextError" class="mt-1 text-red-600">
        Error: {{ rangeTextError }}
      </div>
    </div>

    <div class="mt-5">
      Weight:
      <input
        v-model="weight"
        type="range"
        class="ml-4 w-40 align-middle"
        min="0"
        max="100"
        step="5"
      />
      <input
        v-model="weight"
        type="number"
        :class="
          'w-20 ml-4 px-2 py-1 rounded-lg text-sm text-center ' +
          (weight < 0 || weight > 100
            ? 'ring-1 ring-red-600 border-red-600 bg-red-50'
            : '')
        "
        min="0"
        max="100"
        step="5"
        @change="onWeightChange"
      />
      %

      <button
        :class="
          'rounded-lg shadow-sm ml-16 px-3.5 py-1.5 text-white text-sm font-medium ' +
          'bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-4 focus:ring-blue-300 ' +
          'disabled:opacity-40 disabled:bg-blue-600 disabled:cursor-not-allowed'
        "
        @click="clearRange"
      >
        Clear
      </button>
    </div>

    <div class="mt-3">
      {{ numCombos.toFixed(1) }} combos ({{
        ((numCombos * 100) / ((52 * 51) / 2)).toFixed(1)
      }}%)
    </div>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, ref } from "vue";
import { useStore } from "../store";
import { RangeManager } from "../../pkg/range/range";

const ranks = ["2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"];

const comboPat = "(?:(?:[AKQJT2-9]{2}[os]?)|(?:(?:[AKQJT2-9][cdhs]){2}))";
const weightPat = "(?:(?:[01](\\.\\d*)?)|(?:\\.\\d+))";
const trimRegex = /\s*([-:,])\s*/g;
const rangeRegex = new RegExp(
  `^(?<range>${comboPat}(?:\\+|(?:-${comboPat}))?)(?::(?<weight>${weightPat}))?$`
);

type DraggingMode = "none" | "enabling" | "disabling";

export default defineComponent({
  props: {
    player: {
      type: Number,
      required: true,
    },
  },

  setup(props) {
    const store = useStore();

    const range = RangeManager.new();
    const rangeStore = store.range[props.player];
    const rangeStoreRaw = store.rangeRaw[props.player];
    const rangeText = ref("");
    const rangeTextError = ref("");
    const weight = ref(100);
    const hoveringCellText = ref("");
    const hoveringCellWeight = ref("");

    let draggingMode = "none" as DraggingMode;

    const update = (row: number, col: number, weight: number) => {
      const idx = 13 * (row - 1) + col - 1;
      range.update(row, col, weight / 100);
      rangeStore[idx] = weight;
      rangeStoreRaw.set(range.raw_data());
      rangeText.value = range.to_string();
      rangeTextError.value = "";
    };

    const numCombos = computed(() => {
      let combos = 0;
      for (let row = 0; row < 13; row++) {
        for (let col = 0; col < 13; col++) {
          const dup = [4, 6, 12][Math.sign(row - col) + 1];
          combos += (dup * rangeStore[13 * row + col]) / 100;
        }
      }
      return combos;
    });

    const cellText = (row: number, col: number) => {
      const r1 = 13 - Math.min(row, col);
      const r2 = 13 - Math.max(row, col);
      return ranks[r1] + ranks[r2] + ["s", "", "o"][Math.sign(row - col) + 1];
    };

    const onRangeTextChange = () => {
      const trimmed = rangeText.value.replace(trimRegex, "$1").trim();
      const ranges = trimmed.split(",");

      if (ranges.slice(-1)[0] === "") {
        ranges.pop();
      }

      for (const range of ranges) {
        if (!rangeRegex.test(range)) {
          rangeTextError.value = `Failed to parse range: ${
            range || "(empty string)"
          }`;
          return;
        }
      }

      const errorString = range.from_string(rangeText.value);

      if (errorString) {
        rangeTextError.value = errorString;
      } else {
        const weights = range.get_weights();
        for (let i = 0; i < 13 * 13; ++i) {
          rangeStore[i] = weights[i] * 100;
        }
        rangeStoreRaw.set(range.raw_data());
        rangeText.value = range.to_string();
        rangeTextError.value = "";
      }
    };

    const dragStart = (row: number, col: number) => {
      const idx = 13 * (row - 1) + col - 1;

      if (rangeStore[idx] !== weight.value) {
        draggingMode = "enabling";
        update(row, col, weight.value);
      } else {
        draggingMode = "disabling";
        update(row, col, 0);
      }

      hoveringCellText.value = cellText(row, col) + ":";
      hoveringCellWeight.value = rangeStore[idx].toFixed(1) + "%";
    };

    const dragEnd = () => {
      draggingMode = "none";
    };

    const mouseOver = (row: number, col: number) => {
      const idx = 13 * (row - 1) + col - 1;

      if (draggingMode !== "none") {
        if (draggingMode === "enabling") {
          update(row, col, weight.value);
        } else {
          update(row, col, 0);
        }
      }

      hoveringCellText.value = cellText(row, col) + ":";
      hoveringCellWeight.value = rangeStore[idx].toFixed(1) + "%";
    };

    const mouseLeave = () => {
      dragEnd();
      hoveringCellText.value = "";
      hoveringCellWeight.value = "";
    };

    const weightWidth = (row: number, col: number) => {
      return rangeStore[13 * (row - 1) + col - 1] + "%";
    };

    const onWeightChange = () => {
      weight.value = Math.max(0, Math.min(100, weight.value));
    };

    const clearRange = () => {
      range.clear();
      rangeStore.fill(0);
      rangeStoreRaw.fill(0);
      rangeText.value = "";
      rangeTextError.value = "";
      weight.value = 100;
    };

    return {
      rangeText,
      rangeTextError,
      weight,
      numCombos,
      hoveringCellText,
      hoveringCellWeight,
      cellText,
      onRangeTextChange,
      dragStart,
      dragEnd,
      mouseOver,
      mouseLeave,
      weightWidth,
      onWeightChange,
      clearRange,
    };
  },
});
</script>
