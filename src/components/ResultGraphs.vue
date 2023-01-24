<template>
  <div class="flex w-full h-full">
    <div v-if="!results.isEmpty" class="flex flex-col h-full" style="flex: 4">
      <div class="w-full px-4 pt-4 pb-2" style="flex: 7">
        <div class="relative h-full">
          <LineChart
            class="absolute left-0 top-0 w-full h-full"
            :data="chartData"
            :options="chartOptions"
            :plugins="[verticalLinePlugin]"
            @mouseleave="tableScrollTarget = null"
          />
        </div>
      </div>

      <div class="relative mx-4 mt-6 mb-8" style="flex: 2">
        <div
          v-if="displayPlayer === ['', 'oop', 'ip'][playerIndex + 1]"
          class="absolute flex left-[3.25rem] top-0 h-full"
          :style="{ width: `${chartWidth}px` }"
          @mouseleave="tableScrollTarget = null"
        >
          <div
            v-for="(item, i) in strategyItems"
            :key="i"
            class="hover:brightness-75"
            :style="{ flex: item.weight, 'background-image': item.bgImage }"
            @mouseover="tableScrollTarget = item.cards"
          ></div>
        </div>
      </div>
    </div>

    <div v-else class="flex h-full items-center justify-center" style="flex: 4">
      Graphs not available
    </div>

    <ResultTable
      style="flex: 3"
      table-mode="graphs"
      :graph-type="displayOptions.contentGraphs"
      :cards="cards"
      :selected-spot="selectedSpot"
      :results="results"
      :display-player="displayPlayer"
      :scroll-target="tableScrollTarget"
    />
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, ref } from "vue";
import { cardText, cardPairOrder, toFixed1, toFixedAdaptive } from "../utils";
import {
  Results,
  Spot,
  SpotChance,
  SpotPlayer,
  DisplayOptions,
} from "../result-types";

import {
  Chart,
  ChartData,
  ChartOptions,
  LineController,
  LineElement,
  PointElement,
  LinearScale,
  Title,
  Legend,
  Tooltip,
  Point,
  Plugin,
} from "chart.js";

import { Line as LineChart } from "vue-chartjs";
import ResultTable from "./ResultTable.vue";

Chart.register(
  LineController,
  LineElement,
  PointElement,
  LinearScale,
  Title,
  Legend,
  Tooltip
);

Chart.defaults.font.size = 14;
Chart.defaults.font.family =
  "system-ui, 'Noto Sans', 'Open Sans', Roboto, sans-serif";

const sky500 = "#0ea5e9";
const lime500 = "#84cc16";
const xTicks = 10000;

export default defineComponent({
  components: {
    LineChart,
    ResultTable,
  },

  props: {
    cards: {
      type: Array as () => number[][],
      required: true,
    },
    selectedSpot: {
      type: Object as () => Spot,
      required: true,
    },
    selectedChance: {
      type: Object as () => SpotChance | null,
      required: true,
    },
    results: {
      type: Object as () => Results,
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

  setup(props) {
    const chartWidth = ref(0);
    const tableScrollTarget = ref<number | null>(null);

    const playerIndex = computed(() => {
      if (props.selectedSpot.type === "player" && !props.selectedChance) {
        return props.selectedSpot.player === "oop" ? 0 : 1;
      } else {
        return -1;
      }
    });

    const tupleData = computed(() => {
      if (props.results.isEmpty) return null;

      const results = props.results;
      const content = props.displayOptions.contentGraphs;

      const data: number[][][] = [];

      for (let player = 0; player < 2; ++player) {
        const cards = props.cards[player];
        const cardsLength = cards.length;

        const weights = results.weights[player];
        const normalizer = results.normalizer[player];

        let values: number[];
        if (content === "eq") {
          values = results.equity[player];
        } else if (content === "ev") {
          values = results.ev[player];
        } else {
          values = results.eqr[player]
            .map((x) => x ?? Number.NaN)
            .map((x) => (isFinite(x) ? x : Number.NaN));
        }

        let allTuples: number[][];
        if (player === playerIndex.value) {
          allTuples = Array.from({ length: cardsLength }, (_, i) => {
            const strategy = Array.from(
              { length: results.numActions },
              (_, j) => results.strategy[i + j * cardsLength]
            );
            return [cards[i], normalizer[i], values[i], ...strategy];
          });
        } else {
          allTuples = Array.from({ length: cardsLength }, (_, i) => {
            return [cards[i], normalizer[i], values[i]];
          });
        }

        const tuples = allTuples.filter(
          (_, i) => weights[i] > 0 && normalizer[i] > 0
        );

        tuples.sort((a, b) => {
          if (!isNaN(a[2]) && !isNaN(b[2])) {
            return a[2] - b[2] || cardPairOrder(a[0]) - cardPairOrder(b[0]);
          } else if (!isNaN(a[2])) {
            return -1;
          } else if (!isNaN(b[2])) {
            return 1;
          } else {
            return 0;
          }
        });

        data.push(tuples);
      }

      return data;
    });

    const chartData = computed((): ChartData<"line", Point[]> => {
      if (!tupleData.value) return { datasets: [] };

      const content = props.displayOptions.contentGraphs;
      const data: Point[][] = [];

      for (let player = 0; player < 2; ++player) {
        const tuples = tupleData.value[player];

        let currX = 0;
        const weightSum = tuples.reduce((acc, pair) => acc + pair[1], 0);
        const dataPlayer: (Point & { label: number })[] = [];

        for (const tuple of tuples) {
          const nextX = currX + xTicks * (tuple[1] / weightSum);
          for (let i = Math.ceil(currX); i < nextX; ++i) {
            dataPlayer.push({ x: i / xTicks, y: tuple[2], label: tuple[0] });
          }
          currX = nextX;
        }

        if (currX <= xTicks) {
          const lastTuple = tuples[tuples.length - 1];
          dataPlayer.push({ x: 1, y: lastTuple[2], label: lastTuple[0] });
        }

        data.push(dataPlayer);
      }

      let oopLabel = "";
      let ipLabel = "";
      if (playerIndex.value === 0) oopLabel = "★ ";
      if (playerIndex.value === 1) ipLabel = "★ ";

      const contentText = content === "eq" ? "Equity" : content.toUpperCase();
      oopLabel += `OOP ${contentText}`;
      ipLabel += `IP ${contentText}`;

      return {
        datasets: [
          {
            label: oopLabel,
            data: data[0],
            borderWidth: props.displayPlayer === "oop" ? 4 : 2,
            borderColor: sky500,
            backgroundColor: sky500,
            pointStyle: false,
            stepped: "middle",
          },
          {
            label: ipLabel,
            data: data[1],
            borderWidth: props.displayPlayer === "ip" ? 4 : 2,
            borderColor: lime500,
            backgroundColor: lime500,
            pointStyle: false,
            stepped: "middle",
          },
        ],
      };
    });

    const chartOptions = computed((): ChartOptions<"line"> => {
      const content = props.displayOptions.contentGraphs;
      const styleY = content === "ev" ? "decimal" : "percent";
      const formatX = { style: "percent", minimumFractionDigits: 0 };
      const formatY = {
        style: styleY,
        useGrouping: false,
        minimumFractionDigits: 0,
      };

      return {
        responsive: true,
        maintainAspectRatio: false,
        animation: false,
        parsing: false,
        normalized: true,
        scales: {
          x: {
            type: "linear",
            ticks: { format: formatX },
            afterFit(axis) {
              chartWidth.value = axis.width;
            },
          },
          y: {
            min: content === "eq" ? 0 : undefined,
            max: content === "eq" ? 1 : undefined,
            suggestedMin: content === "ev" ? 0 : undefined,
            ticks: {
              format: formatY,
            },
            afterFit(axis) {
              axis.width = 52;
            },
          },
        },
        interaction: {
          intersect: false,
          mode: "index",
        },
        plugins: {
          legend: {
            labels: {
              font: { size: 16 },
              color: "rgba(0, 0, 0, 0.9)",
              boxHeight: 12,
            },
          },
          tooltip: {
            padding: 8,
            boxWidth: 10,
            boxHeight: 10,
            callbacks: {
              title() {
                return "";
              },
              label(context) {
                const value = context.parsed.y;
                const cardPair = (context.raw as { label: number }).label;
                if (
                  props.displayPlayer === ["oop", "ip"][context.datasetIndex]
                ) {
                  tableScrollTarget.value = cardPair;
                }
                const c1 = cardText(cardPair & 0xff);
                const c2 = cardText(cardPair >>> 8);
                const label = `${c2.rank}${c2.suit}${c1.rank}${c1.suit}`;
                if (content === "ev") {
                  return ` ${label}: ${toFixedAdaptive(value)}`;
                } else {
                  return ` ${label}: ${toFixed1(value * 100)}%`;
                }
              },
            },
          },
        },
      };
    });

    let verticalLinePosition: number | null = null;

    const verticalLinePlugin: Plugin<"line"> = {
      id: "verticalLine",

      afterEvent(chart, args) {
        if (args.event.type === "mousemove") {
          if (args.inChartArea) {
            verticalLinePosition = args.event.x;
          } else {
            verticalLinePosition = null;
            chart.draw();
          }
        } else if (args.event.type === "mouseout") {
          verticalLinePosition = null;
          chart.draw();
        }
      },

      afterDraw(chart) {
        const x = verticalLinePosition;
        if (x === null) return;
        const ctx = chart.ctx;
        ctx.save();
        ctx.beginPath();
        ctx.setLineDash([4, 2]);
        ctx.moveTo(x, chart.scales.y.top);
        ctx.lineTo(x, chart.scales.y.bottom);
        ctx.lineWidth = 1;
        ctx.strokeStyle = "rgba(0, 0, 0, 0.9)";
        ctx.stroke();
        ctx.restore();
      },
    };

    const strategyItems = computed(() => {
      if (!tupleData.value || playerIndex.value === -1) return [];

      const numActions = props.results.numActions;
      const actionColors = (props.selectedSpot as SpotPlayer).actions.map(
        (a) => a.color
      );

      return tupleData.value[playerIndex.value].map((tuple) => {
        let bgImage = "linear-gradient(to top";
        let pos = 0;
        for (let i = numActions - 1; i >= 0; --i) {
          const width = tuple[i + 3] * 100;
          const color = actionColors[i];
          bgImage += `, ${color} ${pos}% ${pos + width}%`;
          pos += width;
        }
        bgImage += ")";
        return { cards: tuple[0], weight: tuple[1], bgImage };
      });
    });

    return {
      chartWidth,
      tableScrollTarget,
      playerIndex,
      chartData,
      chartOptions,
      verticalLinePlugin,
      strategyItems,
    };
  },
});
</script>
