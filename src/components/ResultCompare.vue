<template>
  <div class="flex flex-col w-full h-full px-2 py-1 gap-2.5">
    <div class="flex text-lg font-semibold">
      <div class="flex items-center">
        <span>OOP</span>
        <StarIcon
          v-if="player === 'oop'"
          class="inline-block ml-0.5 w-5 h-5 text-yellow-500"
        />
      </div>
      <div class="flex-grow"></div>
      <div class="flex items-center">
        <StarIcon
          v-if="player === 'ip'"
          class="inline-block mr-0.5 w-5 h-5 text-yellow-500"
        />
        <span>IP</span>
      </div>
    </div>

    <div class="flex flex-col gap-0.5">
      <div class="flex">
        <Adaptive
          :value="combos[0]"
          :class="{
            'w-14': true,
            'font-semibold': combos[0] && combos[0] >= (1 - EPS) * combos[1],
          }"
        />
        <div class="flex-grow text-center underline">Combos</div>
        <Adaptive
          :value="combos[1]"
          :class="{
            'w-14': true,
            'text-right': true,
            'font-semibold': combos[1] && (1 - EPS) * combos[0] <= combos[1],
          }"
        />
      </div>
      <BarChart :values="combos" />
    </div>

    <div class="flex flex-col gap-0.5">
      <div class="flex">
        <Percentage
          :value="equity[0]"
          :class="{
            'w-14': true,
            'font-semibold':
              !isNaN(equity[0]) && equity[0] >= (1 - EPS) * equity[1],
          }"
        />
        <div class="flex-grow text-center underline">Equity</div>
        <Percentage
          :value="equity[1]"
          :class="{
            'w-14': true,
            'text-right': true,
            'font-semibold':
              !isNaN(equity[1]) && (1 - EPS) * equity[0] <= equity[1],
          }"
        />
      </div>
      <BarChart :values="equity" />
    </div>

    <div class="flex flex-col gap-0.5">
      <div class="flex">
        <Ev
          :value="ev[0]"
          :digits="evDigits"
          :class="{
            'w-14': true,
            'font-semibold': !isNaN(ev[0]) && ev[0] >= (1 - EPS) * ev[1],
          }"
        />
        <div class="flex-grow text-center underline">EV</div>
        <Ev
          :value="ev[1]"
          :digits="evDigits"
          :class="{
            'w-14': true,
            'text-right': true,
            'font-semibold': !isNaN(ev[1]) && (1 - EPS) * ev[0] <= ev[1],
          }"
        />
      </div>
      <BarChart :values="ev" />
    </div>

    <div class="flex flex-col gap-0.5">
      <div class="flex">
        <Percentage
          :value="eqr[0]"
          :class="{
            'w-14': true,
            'font-semibold': !isNaN(eqr[0]) && eqr[0] >= (1 - EPS) * eqr[1],
          }"
        />
        <div class="flex-grow text-center underline">EQR</div>
        <Percentage
          :value="eqr[1]"
          :class="{
            'w-14': true,
            'text-right': true,
            'font-semibold': !isNaN(eqr[1]) && (1 - EPS) * eqr[0] <= eqr[1],
          }"
        />
      </div>
      <BarChart :values="eqr" />
    </div>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, h } from "vue";
import { average, toFixed1, toFixed, toFixedAdaptive } from "../utils";
import { Results, Spot, SpotChance } from "../result-types";

import { StarIcon } from "@heroicons/vue/24/solid";

const EPS = 2e-6;
const sky500 = "#0ea5e9";
const lime500 = "#84cc16";

const Adaptive = (props: { value: number; class: object }) => {
  const split = computed(() => toFixedAdaptive(props.value).split("."));
  if (isNaN(props.value)) return h("div", { class: "px-1" }, "-");
  return h("div", { class: props.class }, [
    h("span", {}, split.value[0] + "."),
    h("span", { class: "text-sm" }, split.value[1]),
  ]);
};

const Percentage = (props: { value: number; class: object }) => {
  const str = computed(() => toFixed1(props.value * 100));
  if (isNaN(props.value)) return h("div", { class: "px-1" }, "-");
  return h("div", { class: props.class }, [
    h("span", {}, str.value.slice(0, -1)),
    h("span", { class: "text-sm" }, str.value.slice(-1) + "%"),
  ]);
};

const Ev = (props: { value: number; digits: number; class: object }) => {
  const str = computed(() => toFixed[props.digits - 1](props.value));
  if (isNaN(props.value)) return h("div", { class: "px-1" }, "-");
  return h("span", { class: props.class }, [
    h("span", {}, str.value.slice(0, -props.digits)),
    h("span", { class: "text-xs" }, str.value.slice(-props.digits)),
  ]);
};

const BarChart = (props: { values: number[] }) => {
  if (
    (props.values[0] === 0 && props.values[1] === 0) ||
    isNaN(props.values[0]) ||
    isNaN(props.values[1])
  ) {
    return h("div", { class: "h-4 rounded-lg bg-neutral-400" });
  }
  const total = props.values[0] + props.values[1];
  const sep = `${(props.values[0] * 100) / total}%`;
  const bg = `linear-gradient(to right, ${sky500} ${sep}, ${lime500} ${sep})`;
  return h("div", { class: "h-4 rounded-lg", style: `background: ${bg}` });
};

export default defineComponent({
  components: {
    Adaptive,
    Percentage,
    Ev,
    BarChart,
    StarIcon,
  },

  props: {
    selectedSpot: { type: Object as () => Spot, required: true },
    selectedChance: { type: Object as () => SpotChance | null, required: true },
    results: { type: Object as () => Results, required: true },
  },

  setup(props) {
    const player = computed(() => {
      if (props.selectedChance) return "chance";
      return props.selectedSpot.player;
    });

    const combos = computed(() => {
      const results = props.results;

      const ret: number[] = [];
      for (let i = 0; i < 2; ++i) {
        let sum = 0;
        const n = results.weights[i].length;
        for (let j = 0; j < n; ++j) {
          const weight = results.weights[i][j];
          const normalizer = results.normalizer[i][j];
          sum += normalizer && weight;
        }
        ret.push(sum);
      }

      return ret;
    });

    const equity = computed(() => {
      const results = props.results;
      if (results.isEmpty) return [Number.NaN, Number.NaN];
      return [
        average(results.equity[0], results.normalizer[0]),
        average(results.equity[1], results.normalizer[1]),
      ];
    });

    const ev = computed(() => {
      const results = props.results;
      if (results.isEmpty) return [Number.NaN, Number.NaN];
      return [
        average(results.ev[0], results.normalizer[0]),
        average(results.ev[1], results.normalizer[1]),
      ];
    });

    const eqr = computed(() => {
      const results = props.results;
      if (results.isEmpty) return [Number.NaN, Number.NaN];
      const ret = [
        ev.value[0] / (results.eqrBase[0] * equity.value[0]),
        ev.value[1] / (results.eqrBase[1] * equity.value[1]),
      ];
      if (!isFinite(ret[0])) ret[0] = Number.NaN;
      if (!isFinite(ret[1])) ret[1] = Number.NaN;
      return ret;
    });

    const evDigits = computed(() => {
      const results = props.results;
      if (results.isEmpty) return 3;
      const maxEv = Math.max(...ev.value.map((x) => Math.abs(x)));
      return maxEv < 9.9995 ? 3 : maxEv < 99.995 ? 2 : 1;
    });

    return {
      EPS,
      player,
      combos,
      equity,
      ev,
      eqr,
      evDigits,
    };
  },
});
</script>
