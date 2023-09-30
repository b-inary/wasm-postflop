<template>
  <div class="flex w-full h-full">
    <div
      class="flex flex-col h-full items-center pt-[1%] gap-[1%]"
      style="flex: 4"
    >
      <div
        v-for="suit in 4"
        :key="suit"
        class="flex shrink-0 w-full justify-center gap-[1%]"
      >
        <div class="w-[3.25rem]"></div>
        <BoardSelectorCard
          v-for="rank in 13"
          :key="rank"
          class="disabled:opacity-75 disabled:brightness-75"
          font-size="max(1.2vw, 13px)"
          width="5.5%"
          :card-id="56 - 4 * rank - suit"
          :disabled="selectedChance.cards[56 - 4 * rank - suit].isDead"
          :is-selected="selectedChance.selectedIndex === 56 - 4 * rank - suit"
          @click="deal(56 - 4 * rank - suit)"
        />
        <div></div>
      </div>

      <div
        ref="chartParentDiv"
        class="relative flex-grow max-h-[50%] my-1.5"
        style="width: calc(84.5% + 3.25rem)"
      >
        <div
          v-if="chanceReports && chartParentDivHeight >= 180"
          class="absolute left-0 top-0 w-full h-full"
        >
          <Bar :data="chartData!" :options="chartOptions" />
        </div>
      </div>
    </div>

    <ResultTable
      style="flex: 3"
      table-mode="chance"
      :chance-type="selectedChance.player"
      :selected-spot="selectedSpot"
      :chance-reports="chanceReports"
      :display-player="displayPlayer"
    />
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, ref, watch } from "vue";
import { ranks, suits, cardId, toFixed1, toFixedAdaptive } from "../utils";
import {
  ChanceReports,
  Spot,
  SpotChance,
  SpotPlayer,
  DisplayOptions,
} from "../result-types";

import {
  Chart,
  ChartData,
  ChartOptions,
  BarController,
  BarElement,
  CategoryScale,
  LinearScale,
  Title,
  Tooltip,
} from "chart.js";

import BoardSelectorCard from "./BoardSelectorCard.vue";
import ResultTable from "./ResultTable.vue";
import { Bar } from "vue-chartjs";

Chart.register(
  BarController,
  BarElement,
  CategoryScale,
  LinearScale,
  Title,
  Tooltip
);

Chart.defaults.font.size = 14;
Chart.defaults.font.family =
  "system-ui, 'Noto Sans', 'Open Sans', Roboto, sans-serif";

const labels = [...ranks].reverse();

const green600 = "#16a34a";
const blue600 = "#2563eb";
const pink600 = "#db2777";
const black = "#000000";
const suitColor = [green600, blue600, pink600, black];

export default defineComponent({
  components: {
    BoardSelectorCard,
    ResultTable,
    Bar,
  },

  props: {
    selectedSpot: {
      type: Object as () => Spot,
      required: true,
    },
    selectedChance: {
      type: Object as () => SpotChance,
      required: true,
    },
    chanceReports: {
      type: Object as () => ChanceReports | null,
      required: true,
    },
    displayOptions: {
      type: Object as () => DisplayOptions,
      required: true,
    },
    displayPlayer: {
      type: String as () => "oop" | "ip",
      required: true,
    },
  },

  emits: {
    "deal-card": (_card: number) => true,
  },

  setup(props, context) {
    const chartParentDiv = ref<HTMLDivElement | null>(null);
    const chartParentDivHeight = ref(0);

    const assignChartParentDivHeight = () => {
      if (chartParentDiv.value) {
        chartParentDivHeight.value = chartParentDiv.value.clientHeight;
      }
    };

    watch(chartParentDiv, assignChartParentDivHeight);
    window.addEventListener("resize", assignChartParentDivHeight);

    const chartData = computed((): ChartData<"bar", number[]> | null => {
      const reports = props.chanceReports;
      if (!reports) return null;

      const options = props.displayOptions;
      const playerIndex = props.displayPlayer === "oop" ? 0 : 1;

      let datasets: ChartData<"bar", number[]>["datasets"];
      const stacks = ["clubs", "diamonds", "hearts", "spades"];
      const defaultData = { barPercentage: 0.85, categoryPercentage: 0.75 };

      if (
        options.chartChance === "strategy-combos" ||
        options.chartChance === "strategy"
      ) {
        const isCombos = options.chartChance === "strategy-combos";
        if (reports.currentPlayer === props.displayPlayer) {
          const spot = props.selectedSpot as SpotPlayer;
          datasets = Array.from({ length: reports.numActions * 4 }, (_, i) => {
            const actionIndex = i >> 2;
            const suit = i & 3;
            const action = spot.actions[actionIndex];
            let label = action.name;
            if (action.amount !== "0") label += ` ${action.amount}`;
            return {
              data: Array.from({ length: 13 }, (_, rank) => {
                const card = cardId(rank, suit);
                if (reports.status[card] === 0) return 0;
                const coef = isCombos ? reports.combos[playerIndex][card] : 1;
                return coef * reports.strategy[actionIndex * 52 + card];
              }).reverse(),
              label,
              backgroundColor: action.color,
              stack: stacks[suit],
              ...defaultData,
            };
          }).reverse();
        } else {
          datasets = Array.from({ length: 4 }, (_, suit) => ({
            data: Array.from({ length: 13 }, (_, rank) => {
              const card = cardId(rank, suit);
              if (reports.status[card] === 0) return 0;
              return isCombos ? reports.combos[playerIndex][card] : 1;
            }).reverse(),
            backgroundColor: suitColor[suit],
            stack: stacks[suit],
            ...defaultData,
          })).reverse();
        }
      } else {
        const data =
          options.chartChance === "eq"
            ? reports.equity[playerIndex]
            : options.chartChance === "ev"
            ? reports.ev[playerIndex]
            : reports.eqr[playerIndex];
        datasets = Array.from({ length: 4 }, (_, suit) => ({
          data: Array.from(
            { length: 13 },
            (_, rank) => data[4 * rank + suit]
          ).reverse(),
          backgroundColor: suitColor[suit],
          stack: stacks[suit],
          ...defaultData,
        })).reverse();
      }

      return { labels, datasets };
    });

    const chartOptions = computed((): ChartOptions<"bar"> => {
      const option = props.displayOptions.chartChance;
      const style = ["strategy", "eq", "eqr"].includes(option)
        ? "percent"
        : "decimal";
      const format = { style, useGrouping: false, minimumFractionDigits: 0 };

      const titleText =
        props.displayPlayer.toUpperCase() +
        " " +
        {
          "strategy-combos": "Strategy (Combos)",
          strategy: "Strategy",
          eq: "Equity",
          ev: "EV",
          eqr: "EQR",
        }[option];

      return {
        responsive: true,
        maintainAspectRatio: false,
        animation: false,
        normalized: true,
        scales: {
          y: {
            stacked: true,
            min: ["ev", "eqr"].includes(option) ? undefined : 0,
            max: option === "strategy" ? 1 : undefined,
            ticks: { format },
            afterFit(axis) {
              axis.width = 52;
            },
          },
        },
        plugins: {
          title: {
            display: true,
            text: titleText,
            font: { size: 16, weight: "normal" },
            color: "rgba(0, 0, 0, 0.9)",
          },
          legend: {
            display: false,
          },
          tooltip: {
            titleMarginBottom: 4,
            callbacks: {
              title(context) {
                const rank = 12 - context[0].dataIndex;
                const suit = 3 - (context[0].datasetIndex & 3);
                return ranks[rank] + suits[suit];
              },
              label(context) {
                const value = context.parsed.y;
                let label = context.dataset.label ?? "";
                if (label) label += ": ";
                if (["strategy-combos", "ev"].includes(option)) {
                  return ` ${label}${toFixedAdaptive(value)}`;
                } else {
                  return ` ${label}${toFixed1(value * 100)}%`;
                }
              },
            },
          },
        },
      };
    });

    const deal = (card: number) => {
      context.emit("deal-card", card);
    };

    return {
      chartParentDiv,
      chartParentDivHeight,
      chartData,
      chartOptions,
      deal,
    };
  },
});
</script>
