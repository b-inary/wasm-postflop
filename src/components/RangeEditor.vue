<template>
  <div class="flex mt-2">
    <div class="shrink-0 ml-1">
      <table class="bg-gray-200 shadow" @mouseleave="dragEnd">
        <tr v-for="row in 13" :key="row" class="h-9">
          <td
            v-for="col in 13"
            :key="col"
            :class="
              'relative w-10 border-black select-none ' +
              (row === col ? 'border-2' : 'border')
            "
            @mousedown="dragStart(row, col)"
            @mouseup="dragEnd"
            @mouseenter="mouseEnter(row, col)"
          >
            <div
              class="absolute bottom-0 left-0 w-full bg-yellow-300"
              :style="{ height: weightPercent(row, col) }"
            ></div>
            <div
              :class="
                'absolute -top-px left-px z-10 text-sm ' +
                (!hasWeight(row, col) ? 'text-gray-500' : '')
              "
            >
              {{ cellText(row, col) }}
            </div>
            <div class="absolute -bottom-px right-px z-10 text-sm">
              {{
                !hasWeight(row, col) || isWeightFull(row, col)
                  ? ""
                  : weightPercent(row, col)
              }}
            </div>
          </td>
        </tr>
      </table>

      <div class="mt-5">
        <div class="flex items-center">
          <input
            v-model="rangeText"
            type="text"
            :class="
              'w-[27rem] px-2 py-1 rounded-lg text-sm ' +
              (rangeTextError
                ? 'ring-1 ring-red-600 border-red-600 bg-red-50'
                : '')
            "
            @focus="($event.target as HTMLInputElement).select()"
            @change="onRangeTextChange"
          />

          <button
            :class="
              'rounded-lg shadow-sm ml-6 px-3.5 py-1.5 text-white text-sm font-medium ' +
              'bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-4 focus:ring-blue-300'
            "
            @click="clearRange"
          >
            Clear
          </button>
        </div>

        <div v-if="rangeTextError" class="mt-1 text-red-600">
          Error: {{ rangeTextError }}
        </div>
      </div>
    </div>

    <db-item-picker
      class="shrink ml-6"
      store-name="ranges"
      :index="player"
      :value="rangeText"
      :allow-save="rangeText !== '' && rangeTextError === ''"
      @load-item="loadRange"
    />
  </div>

  <div class="mt-3">
    Weight:
    <input
      v-model="weight"
      type="range"
      class="ml-3 w-40 align-middle"
      min="0"
      max="100"
      step="5"
      @change="onWeightChange"
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

    <span class="inline-block ml-8">
      {{ numCombos.toFixed(1) }} combos ({{
        ((numCombos * 100) / ((52 * 51) / 2)).toFixed(1)
      }}%)
    </span>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import { useConfigStore } from "../store";
import { RangeManager } from "../../pkg/range/range";

import DbItemPicker from "./DbItemPicker.vue";

const ranks = ["2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"];

const rankPat = "[AaKkQqJjTt2-9]";
const comboPat = `(?:(?:${rankPat}{2}[os]?)|(?:(?:${rankPat}[cdhs]){2}))`;
const weightPat = "(?:(?:[01](\\.\\d*)?)|(?:\\.\\d+))";
const trimRegex = /\s*([-:,])\s*/g;
const rangeRegex = new RegExp(
  `^(?<range>${comboPat}(?:\\+|(?:-${comboPat}))?)(?::(?<weight>${weightPat}))?$`
);

type DraggingMode = "none" | "enabling" | "disabling";

export default defineComponent({
  components: {
    DbItemPicker,
  },

  props: {
    player: {
      type: Number,
      required: true,
    },
  },

  setup(props) {
    const config = useConfigStore();

    const range = RangeManager.new();
    const rangeStore = config.range[props.player];
    const rangeStoreRaw = config.rangeRaw[props.player];
    const rangeText = ref("");
    const rangeTextError = ref("");
    const weight = ref(100);
    const numCombos = ref(0);

    let draggingMode = "none" as DraggingMode;

    const onUpdate = () => {
      rangeStoreRaw.set(range.raw_data());
      rangeText.value = range.to_string();
      rangeTextError.value = "";
      numCombos.value = rangeStoreRaw.reduce((acc, cur) => acc + cur, 0);
    };

    const update = (row: number, col: number, weight: number) => {
      const idx = 13 * (row - 1) + col - 1;
      range.update(row, col, weight / 100);
      rangeStore[idx] = weight;
      onUpdate();
    };

    const cellText = (row: number, col: number) => {
      const r1 = 13 - Math.min(row, col);
      const r2 = 13 - Math.max(row, col);
      return ranks[r1] + ranks[r2] + ["s", "", "o"][Math.sign(row - col) + 1];
    };

    const onRangeTextChange = () => {
      const trimmed = rangeText.value.replace(trimRegex, "$1").trim();
      const ranges = trimmed.split(",");

      if (ranges[ranges.length - 1] === "") {
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

      const errorString = range.from_string(trimmed);

      if (errorString) {
        rangeTextError.value = errorString;
      } else {
        const weights = range.get_weights();
        for (let i = 0; i < 13 * 13; ++i) {
          rangeStore[i] = weights[i] * 100;
        }
        onUpdate();
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
    };

    const dragEnd = () => {
      draggingMode = "none";
    };

    const mouseEnter = (row: number, col: number) => {
      if (draggingMode === "enabling") {
        update(row, col, weight.value);
      } else if (draggingMode === "disabling") {
        update(row, col, 0);
      }
    };

    const hasWeight = (row: number, col: number) => {
      return rangeStore[13 * (row - 1) + col - 1] > 0;
    };

    const isWeightFull = (row: number, col: number) => {
      return rangeStore[13 * (row - 1) + col - 1] === 100;
    };

    const weightPercent = (row: number, col: number) => {
      return rangeStore[13 * (row - 1) + col - 1].toFixed(0) + "%";
    };

    const onWeightChange = () => {
      weight.value = Math.round(Math.max(0, Math.min(100, weight.value)));
    };

    const clearRange = () => {
      range.clear();
      rangeStore.fill(0);
      rangeStoreRaw.fill(0);
      rangeText.value = "";
      rangeTextError.value = "";
      weight.value = 100;
      numCombos.value = 0;
    };

    const loadRange = (rangeStr: string) => {
      rangeText.value = rangeStr;
      onRangeTextChange();
    };

    return {
      rangeText,
      rangeTextError,
      weight,
      numCombos,
      cellText,
      onRangeTextChange,
      dragStart,
      dragEnd,
      mouseEnter,
      hasWeight,
      isWeightFull,
      weightPercent,
      onWeightChange,
      clearRange,
      loadRange,
    };
  },
});
</script>
