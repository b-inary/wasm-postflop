<template>
  <table
    class="w-full h-full table-fixed select-none snug"
    :style="{
      '--font-scale': isCompareMode ? 0.875 : 1,
      '--pair-font-size':
        'calc(max(1rem, min(1.35vw, 2.25vh)) * var(--font-scale))',
      '--value-font-size':
        'calc(max(0.889rem, min(1.2vw, 2vh)) * var(--font-scale))',
    }"
  >
    <tr v-for="row in 13" :key="row">
      <td v-for="col in 13" :key="col" class="relative border border-black">
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
        >
          {{ cellValueText(row, col) }}
        </div>
      </td>
    </tr>
  </table>
</template>

<script lang="ts">
import { computed, defineComponent } from "vue";
import { useSavedConfigStore } from "../store";
import { ranks, cardPairCellIndex, colorString } from "../utils";

import {
  Results,
  Spot,
  SpotChance,
  SpotPlayer,
  DisplayOptions,
} from "../result-types";

const amber500 = "#f59e0b";
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
      type: Array as () => Uint16Array[],
      required: true,
    },
    selectedSpot: {
      type: Object as () => Spot | null,
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
      type: Object as () => Results | null,
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

  setup(props) {
    const config = useSavedConfigStore();

    const cellData = computed(() => {
      const selectedSpot = props.selectedSpot;
      const results = props.results;
      if (!selectedSpot || !results) return null;

      const options = props.displayOptions;
      const player = props.displayPlayer;

      const showStrategy =
        options.strategy === "show" &&
        !props.selectedChance &&
        player === selectedSpot.player;
      const numActions = showStrategy
        ? (selectedSpot as SpotPlayer).actions.length
        : 0;

      const isSuitIndividual = options.suit === "individual";

      const data = Array.from({ length: 13 * 13 }, (_, i) => {
        const row = Math.floor(i / 13);
        const col = i % 13;
        const len = !isSuitIndividual ? 1 : row == col ? 6 : row < col ? 4 : 12;
        return Array.from({ length: len }, () => ({
          weight: 0,
          normalizer: 0,
          value: 0,
          strategy: Array.from({ length: numActions }, () => 0),
        }));
      });

      const playerIndex = player === "oop" ? 0 : 1;
      const cardsLength = props.cards[playerIndex].length;
      const isEmpty = props.results.isEmpty;

      for (let i = 0; i < cardsLength; ++i) {
        const weight = results.weights[playerIndex][i];
        const normalizer = results.normalizer[playerIndex][i];
        if (weight < 0.0005 || normalizer === 0) continue;

        const pair = props.cards[playerIndex][i];
        const card1 = pair & 0xff;
        const card2 = pair >> 8;
        const { row, col, index } = cardPairCellIndex(card1, card2);
        const cellIndex = row * 13 + col;
        const suitIndex = isSuitIndividual ? index : 0;
        const target = data[cellIndex][suitIndex];

        target.weight += weight;
        target.normalizer += normalizer;

        if (!isEmpty) {
          if (options.contentBasics === "eq") {
            target.value += results.equity[playerIndex][i] * normalizer;
          } else if (options.contentBasics === "ev") {
            target.value += results.ev[playerIndex][i] * normalizer;
          } else if (options.contentBasics === "eqr") {
            target.value += results.eqr[playerIndex][i] * normalizer;
          } else {
            target.value += weight;
          }
        }

        if (showStrategy) {
          for (let j = 0; j < numActions; ++j) {
            const k = j * cardsLength + i;
            target.strategy[j] += results.strategy[k] * normalizer;
          }
        }
      }

      return data;
    });

    const cellDenominator = computed(() => {
      const ret = Array.from({ length: 13 * 13 }, () => 0);

      for (let card1 = 0; card1 < 52; ++card1) {
        if (props.currentBoard.includes(card1)) continue;
        for (let card2 = card1 + 1; card2 < 52; ++card2) {
          if (props.currentBoard.includes(card2)) continue;
          const rank1 = Math.floor(card1 / 4);
          const rank2 = Math.floor(card2 / 4);
          const isSuited = card1 % 4 === card2 % 4;
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
      if (!cellData.value) return 0;

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
      if (!cellData.value || !results) return null;

      const options = props.displayOptions;
      const barHeight = options.barHeight;
      const isSuitIndividual = options.suit === "individual";
      const playerIndex = props.displayPlayer === "oop" ? 0 : 1;
      const showStrategy = cellData.value[0][0].strategy.length > 0;
      const isEmpty = results.isEmpty;

      let lowest = 0;
      let middle = 0;
      let highest = 0;

      if (!showStrategy) {
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
          const selectedSpot = props.selectedSpot;
          const weight = suit.weight;
          const normalizer = suit.normalizer;
          if (!selectedSpot || weight === 0) return null;

          let height;
          if (barHeight === "normalized") {
            height = weight / (denominator * maxWeight.value);
          } else if (barHeight === "absolute") {
            height = weight / denominator;
          } else {
            height = 1;
          }

          if (!showStrategy) {
            let color;
            if (isEmpty || options.contentBasics === "default") {
              color = amber500;
            } else {
              const value = suit.value / normalizer;
              color = getColor(value, lowest, middle, highest);
            }

            const bgImage = `linear-gradient(${color} 0% 100%)`;
            const bgSize = `auto ${height * 100}%`;
            return { bgImage, bgSize };
          }

          const spot = selectedSpot as SpotPlayer;
          const colors = spot.actions.map((action) => action.color);

          let bgImage, bgSize;
          if (!isSuitIndividual) {
            bgImage = "";
            bgSize = "";
            let prevPos = 0;
            for (let i = suit.strategy.length - 1; i >= 0; --i) {
              const pos = prevPos + suit.strategy[i] / normalizer;
              bgImage += `linear-gradient(${colors[i]} 0% 100%)`;
              bgSize += `${pos * 100}% ${height * 100}%`;
              if (i > 0) (bgImage += ", "), (bgSize += ", ");
              prevPos = pos;
            }
          } else {
            bgImage = "linear-gradient(to top";
            bgSize = `auto ${height * 100}%`;
            let prevPos = 0;
            for (let i = suit.strategy.length - 1; i >= 0; --i) {
              const pos = prevPos + suit.strategy[i] / normalizer;
              bgImage += `, ${colors[i]} ${prevPos * 100}% ${pos * 100}%`;
              prevPos = pos;
            }
            bgImage += ")";
          }

          return { bgImage, bgSize };
        });
      });
    });

    const columns = (row: number, col: number) => {
      const cellIndex = (row - 1) * 13 + col - 1;
      if (!cellContent.value) return null;
      return cellContent.value[cellIndex];
    };

    const hasWeight = (row: number, col: number) => {
      const cellIndex = (row - 1) * 13 + col - 1;
      if (!cellData.value) return false;
      return cellData.value[cellIndex].some((suit) => suit.weight > 0);
    };

    const cellText = (row: number, col: number) => {
      const r1 = 13 - Math.min(row, col);
      const r2 = 13 - Math.max(row, col);
      return ranks[r1] + ranks[r2] + ["s", "", "o"][Math.sign(row - col) + 1];
    };

    const cellValueText = (row: number, col: number) => {
      const cellIndex = (row - 1) * 13 + col - 1;
      if (!cellData.value) return "";

      const displayOptions = props.displayOptions;
      if (
        displayOptions.strategy === "show" &&
        displayOptions.contentBasics === "default"
      ) {
        return "";
      }

      const data = cellData.value[cellIndex];
      const isEmpty = data.every((suit) => suit.weight === 0);
      if (isEmpty) return "";
      if (props.results?.isEmpty) return "-";

      let valueSum = 0;
      let normalizerSum = 0;
      for (const suit of data) {
        valueSum += suit.value;
        normalizerSum += suit.normalizer;
      }

      let value;
      if (displayOptions.contentBasics === "default") {
        value = valueSum / cellDenominator.value[cellIndex];
      } else {
        value = valueSum / normalizerSum;
      }

      if (displayOptions.contentBasics === "ev") {
        const ret = value >= 999.95 ? value.toFixed(0) : value.toFixed(1);
        return ret === "-0.0" ? "0.0" : ret;
      } else {
        const ret = (value * 100).toFixed(1);
        return ret === "-0.0" ? "0.0" : ret;
      }
    };

    return {
      columns,
      hasWeight,
      cellText,
      cellValueText,
    };
  },
});
</script>
