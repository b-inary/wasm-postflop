<template>
  <div class="w-full h-full">
    <table
      class="w-full h-full table-fixed select-none snug"
      :style="{
        '--font-scale': isCompareMode ? 0.875 : 1,
        '--pair-font-size':
          'calc(max(1rem, min(1.35vw, 2.25vh)) * var(--font-scale))',
        '--value-font-size':
          'calc(max(0.889rem, min(1.2vw, 2vh)) * var(--font-scale))',
      }"
      @mouseleave="onMouseLeaveTable"
    >
      <tr v-for="row in 13" :key="row">
        <td
          v-for="col in 13"
          :key="col"
          :class="
            'relative border border-black ' +
            (clickedCellIndex === cellIndex(row, col) ? 'clicked-cell' : '')
          "
          @click="onClickCell(row, col)"
          @mouseenter="onMouseEnterCell(row, col)"
        >
          <div
            :class="
              'flex absolute w-full h-full left-0 top-0 ' +
              (row === col ? 'bg-neutral-700' : 'bg-neutral-800')
            "
          >
            <div
              v-for="(column, k) in columns(row, col)"
              :key="k"
              class="flex-grow h-full bg-left-bottom bg-no-repeat"
              :style="{
                'background-image': column?.bgImage ?? 'none',
                'background-size': column?.bgSize ?? 'auto',
              }"
            ></div>
          </div>
          <div
            :class="
              'absolute -top-px left-[0.1875rem] z-10 text-shadow ' +
              (hasWeight(row, col) ? 'text-white' : 'text-neutral-500')
            "
            style="font-size: var(--pair-font-size)"
          >
            {{ cellText(row, col) }}
          </div>
          <div
            class="absolute bottom-px right-1 z-10 text-shadow text-white overflow-hidden"
            style="
              max-width: calc(100% - 0.25rem);
              font-size: var(--value-font-size);
            "
            :data-set="(strTmp = cellValueText[cellIndex(row, col)])"
          >
            {{ strTmp.split(".")[0]
            }}<span v-if="strTmp.includes('.')"
              >.<span style="font-size: 85%">{{
                strTmp.split(".")[1]
              }}</span></span
            >
          </div>
        </td>
      </tr>
    </table>
  </div>
</template>

<script lang="ts">
import {
  computed,
  defineComponent,
  ref,
  toRefs,
  onUnmounted,
  watch,
} from "vue";
import { useSavedConfigStore } from "../store";
import {
  ranks,
  cardPairCellIndex,
  toFixed1,
  toFixed,
  colorString,
} from "../utils";

import {
  Results,
  Spot,
  SpotChance,
  SpotPlayer,
  DisplayOptions,
  HoverContent,
} from "../result-types";

const yellow500 = "#eab308";
const colorGradient = [
  { red: 0xef, green: 0x44, blue: 0x44 }, // red-500
  { red: 0xf9, green: 0x73, blue: 0x16 }, // orange-500
  { red: 0xf5, green: 0x9e, blue: 0x0b }, // amber-500
  { red: 0xea, green: 0xb3, blue: 0x08 }, // yellow-500
  { red: 0x84, green: 0xcc, blue: 0x16 }, // lime-500
  { red: 0x22, green: 0xc5, blue: 0x5e }, // green-500
  { red: 0x10, green: 0xb9, blue: 0x81 }, // emerald-500
];

const getColor = (
  value: number,
  lowest: number,
  middle: number,
  highest: number
) => {
  if (value <= lowest) return colorString(colorGradient[0]);
  if (value > middle && value >= highest) return colorString(colorGradient[6]);

  let colorRate;
  let gradientRate;

  if (value <= middle) {
    colorRate = (value - lowest) / (middle - lowest);
    gradientRate = colorRate * 3;
  } else {
    colorRate = (value - middle) / (highest - middle);
    gradientRate = colorRate * 3 + 3;
  }

  const gradientIndex = Math.floor(gradientRate);
  const r = gradientRate - gradientIndex;

  const color1 = colorGradient[gradientIndex];
  const color2 = colorGradient[gradientIndex + 1];

  const retColor = { red: 0, green: 0, blue: 0 };
  for (const primary of ["red", "green", "blue"] as const) {
    const primary1 = color1[primary];
    const primary2 = color2[primary];
    retColor[primary] = Math.floor(primary1 * (1 - r) + primary2 * r);
  }

  return colorString(retColor);
};

export default defineComponent({
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
    currentBoard: {
      type: Array as () => number[],
      required: true,
    },
    results: {
      type: Object as () => Results,
      required: true,
    },
    totalBetAmount: {
      type: Array as () => number[],
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
    isCompareMode: {
      type: Boolean,
      required: true,
    },
  },

  emits: {
    "update-hover-content": (_content: HoverContent | null) => true,
  },

  setup(props, context) {
    const config = useSavedConfigStore();

    const clickedCellIndex = ref(-1);

    const evDigits = computed(() => {
      const results = props.results;
      if (results.isEmpty) return 3;
      const playerIndex = props.displayPlayer === "oop" ? 0 : 1;
      const maxEv = Math.max(...results.ev[playerIndex]);
      return maxEv < 9.9995 ? 3 : maxEv < 99.995 ? 2 : 1;
    });

    const toFixedEv = computed(() => {
      return toFixed[evDigits.value - 1];
    });

    const numActions = computed(() => {
      return (
        (props.displayOptions.strategy === "show" &&
          props.displayPlayer === props.selectedSpot.player &&
          !props.selectedChance &&
          props.results.numActions) ||
        0
      );
    });

    const cellData = computed(() => {
      const results = props.results;
      const options = props.displayOptions;
      const player = props.displayPlayer;

      const isSuitIndividual = options.suit === "individual";

      const data = Array.from({ length: 13 * 13 }, (_, i) => {
        const row = Math.floor(i / 13);
        const col = i % 13;
        let len = 1;
        if (isSuitIndividual) {
          len = row === col ? 6 : row < col ? 4 : 12;
        }
        return Array.from({ length: len }, () => ({
          indices: [] as number[],
          weight: 0,
          normalizer: 0,
          equity: 0,
          ev: 0,
          strategy: Array.from({ length: numActions.value }, () => 0),
        }));
      });

      const playerIndex = player === "oop" ? 0 : 1;
      const cardsLength = props.cards[playerIndex].length;
      const isEmpty = props.results.isEmpty;

      for (let i = 0; i < cardsLength; ++i) {
        const weight = results.weights[playerIndex][i];
        const normalizer = results.normalizer[playerIndex][i];
        if (weight === 0 || normalizer === 0) continue;

        const pair = props.cards[playerIndex][i];
        const card1 = pair & 0xff;
        const card2 = pair >>> 8;
        const { row, col, index } = cardPairCellIndex(card1, card2);
        const cellIndex = row * 13 + col;
        const suitIndex = isSuitIndividual ? index : 0;
        const target = data[cellIndex][suitIndex];

        target.indices.push(i);
        target.weight += weight;
        target.normalizer += normalizer;

        if (!isEmpty) {
          target.equity += results.equity[playerIndex][i] * normalizer;
          target.ev += results.ev[playerIndex][i] * normalizer;
        }

        if (numActions.value > 0) {
          for (let j = 0; j < numActions.value; ++j) {
            const k = j * cardsLength + i;
            target.strategy[j] += results.strategy[k] * normalizer;
          }
        }
      }

      for (const cell of data) {
        const hasWeight = cell.some((suit) => suit.weight > 0);
        if (!hasWeight) cell.length = 0;
      }

      return data;
    });

    const { selectedSpot, displayPlayer } = toRefs(props);
    watch([selectedSpot, displayPlayer], () => {
      clickedCellIndex.value = -1;
      context.emit("update-hover-content", null);
    });

    const cellDenominator = computed(() => {
      const ret = Array.from({ length: 13 * 13 }, () => 0);

      for (let card1 = 0; card1 < 52; ++card1) {
        if (props.currentBoard.includes(card1)) continue;
        for (let card2 = card1 + 1; card2 < 52; ++card2) {
          if (props.currentBoard.includes(card2)) continue;
          const rank1 = card1 >>> 2;
          const rank2 = card2 >>> 2;
          const isSuited = (card1 & 3) === (card2 & 3);
          if (isSuited) {
            const index = (12 - rank2) * 13 + (12 - rank1);
            ++ret[index];
          } else {
            const index = (12 - rank1) * 13 + (12 - rank2);
            ++ret[index];
          }
        }
      }

      return ret;
    });

    const maxWeight = computed(() => {
      let ret = 0;
      cellData.value.forEach((cell, i) => {
        const denominator = cell.length > 1 ? 1 : cellDenominator.value[i];
        for (const suit of cell) {
          const weight = suit.weight / denominator;
          if (weight > ret) ret = weight;
        }
      });

      return ret;
    });

    const cellContent = computed(() => {
      const results = props.results;
      const options = props.displayOptions;
      const playerIndex = props.displayPlayer === "oop" ? 0 : 1;

      const isEmpty = results.isEmpty;
      const eqrBase = results.eqrBase[playerIndex];
      const barHeight = options.barHeight;
      const isSuitIndividual = options.suit === "individual";

      let lowest = 0;
      let middle = 0;
      let highest = 0;

      if (numActions.value === 0) {
        if (options.contentBasics === "eq") {
          lowest = 0;
          middle = 0.5;
          highest = 1;
        } else if (options.contentBasics === "ev") {
          const amounts = props.totalBetAmount;
          const amountSum = Math.min(...amounts) + amounts[playerIndex];
          const pot = config.startingPot + amountSum;
          lowest = 0;
          middle = pot / 2;
          highest = pot;
        } else if (options.contentBasics === "eqr") {
          lowest = 0;
          middle = 1;
          highest = 2;
        }
      }

      return cellData.value.map((cell, i) => {
        const denominator = cell.length > 1 ? 1 : cellDenominator.value[i];

        return cell.map((suit) => {
          const weight = suit.weight;
          const normalizer = suit.normalizer;
          if (weight === 0) return null;

          let height;
          if (barHeight === "normalized") {
            height = weight / (denominator * maxWeight.value);
          } else if (barHeight === "absolute") {
            height = weight / denominator;
          } else {
            height = 1;
          }

          if (numActions.value === 0) {
            let color;
            if (isEmpty || options.contentBasics === "default") {
              color = yellow500;
            } else {
              let value: number;
              if (options.contentBasics === "eq") {
                value = suit.equity / normalizer;
              } else if (options.contentBasics === "ev") {
                value = suit.ev / normalizer;
              } else {
                value = suit.ev / (eqrBase * suit.equity);
              }
              color = getColor(value, lowest, middle, highest);
            }

            const bgImage = `linear-gradient(${color} 0% 100%)`;
            const bgSize = `100% ${height * 100}%`;
            return { bgImage, bgSize };
          }

          const spot = props.selectedSpot as SpotPlayer;
          const colors = spot.actions.map((action) => action.color);

          let bgImage = `linear-gradient(to ${
            isSuitIndividual ? "top" : "right"
          }`;
          const bgSize = `100% ${height * 100}%`;

          let prevPos = 0;
          for (let i = suit.strategy.length - 1; i >= 0; --i) {
            const pos = prevPos + suit.strategy[i] / normalizer;
            bgImage += `, ${colors[i]} ${prevPos * 100}% ${pos * 100}%`;
            prevPos = pos;
          }
          bgImage += ")";

          return { bgImage, bgSize };
        });
      });
    });

    const cellIndex = (row: number, col: number) => {
      return (row - 1) * 13 + col - 1;
    };

    const columns = (row: number, col: number) => {
      return cellContent.value[cellIndex(row, col)];
    };

    const hasWeight = (row: number, col: number) => {
      return cellData.value[cellIndex(row, col)].length > 0;
    };

    const cellText = (row: number, col: number) => {
      const r1 = 13 - Math.min(row, col);
      const r2 = 13 - Math.max(row, col);
      return ranks[r1] + ranks[r2] + ["s", "", "o"][Math.sign(row - col) + 1];
    };

    const cellValueText = computed(() => {
      return Array.from({ length: 13 * 13 }, (_, index) => {
        const displayOptions = props.displayOptions;
        if (
          displayOptions.strategy === "show" &&
          displayOptions.contentBasics === "default"
        ) {
          return "";
        }

        const data = cellData.value[index];
        if (data.length === 0) return "";

        let weightSum = 0;
        let normalizerSum = 0;
        let equitySum = 0;
        let evSum = 0;

        for (const suit of data) {
          weightSum += suit.weight;
          normalizerSum += suit.normalizer;
          equitySum += suit.equity;
          evSum += suit.ev;
        }

        let value;
        if (displayOptions.contentBasics === "default") {
          value = weightSum / cellDenominator.value[index];
        } else if (displayOptions.contentBasics === "eq") {
          value = equitySum / normalizerSum;
        } else if (displayOptions.contentBasics === "ev") {
          value = evSum / normalizerSum;
        } else {
          const playerIndex = props.displayPlayer === "oop" ? 0 : 1;
          const eqrBase = props.results.eqrBase[playerIndex];
          value = evSum / (eqrBase * equitySum);
        }

        if (
          displayOptions.contentBasics !== "default" &&
          props.results.isEmpty
        ) {
          return "-";
        } else if (displayOptions.contentBasics === "ev") {
          return Math.abs(value) >= 999.95
            ? value.toFixed(0)
            : toFixedEv.value(value);
        } else {
          return toFixed1(value * 100);
        }
      });
    });

    const onClickCell = (row: number, col: number) => {
      if (props.isCompareMode) return;
      const index = cellIndex(row, col);
      if (!hasWeight(row, col)) {
        clickedCellIndex.value = -1;
        context.emit("update-hover-content", null);
      } else if (clickedCellIndex.value === index) {
        clickedCellIndex.value = -1;
      } else {
        clickedCellIndex.value = -1;
        onMouseEnterCell(row, col);
        clickedCellIndex.value = index;
      }
    };

    const onMouseEnterCell = (row: number, col: number) => {
      if (props.isCompareMode || clickedCellIndex.value !== -1) return;
      if (hasWeight(row, col) && cellData.value) {
        const index = cellIndex(row, col);
        const indices = cellData.value[index].flatMap((suit) => suit.indices);
        context.emit("update-hover-content", {
          name: cellText(row, col),
          indices,
        });
      } else {
        context.emit("update-hover-content", null);
      }
    };

    const onMouseLeaveTable = () => {
      if (props.isCompareMode || clickedCellIndex.value !== -1) return;
      context.emit("update-hover-content", null);
    };

    onUnmounted(() => {
      if (props.isCompareMode || clickedCellIndex.value === -1) return;
      context.emit("update-hover-content", null);
    });

    return {
      clickedCellIndex,
      cellIndex,
      columns,
      hasWeight,
      cellText,
      cellValueText,
      onClickCell,
      onMouseEnterCell,
      onMouseLeaveTable,
      strTmp: "",
    };
  },
});
</script>

<style scoped>
.clicked-cell::before {
  content: "";
  @apply absolute -left-px -top-px z-10 border-2 border-red-500;
  width: calc(100% + 2px);
  height: calc(100% + 2px);
}
</style>
