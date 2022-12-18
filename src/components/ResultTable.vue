<template>
  <div class="flex flex-col w-full overflow-x-auto">
    <div class="flex shrink-0 h-12 border-y border-gray-500">
      <div class="flex h-full px-4 items-center text-lg font-bold">Summary</div>
      <div class="flex h-full ml-auto px-4 items-center">
        <Tippy content="Export summary to CSV file">
          <a
            ref="exportSummaryButton"
            :class="
              'flex w-8 h-8 items-center justify-center border border-gray-600 bg-gray-200 ' +
              'rounded-lg shadow cursor-pointer transition-colors active:bg-gray-300'
            "
            download="summary.csv"
            @click="exportSummary"
          >
            <ArrowTopRightOnSquareIcon class="w-5 h-5" />
          </a>
        </Tippy>
      </div>
    </div>

    <div
      ref="tableDiv"
      class="flex-grow overflow-y-scroll will-change-scroll"
      @scroll.passive="onTableScroll"
    >
      <table class="w-full h-full text-sm text-center align-middle">
        <thead class="sticky top-0 z-30 bg-gray-100 shadow row-divider">
          <tr style="height: calc(1.9rem + 1px)">
            <th
              v-for="(text, i) in headers"
              :key="text"
              scope="col"
              :class="
                'whitespace-nowrap font-bold select-none ' +
                (i === 0 ? 'sticky left-0 z-40 bg-gray-100 ' : '') +
                (i !== 1 ? 'cursor-pointer' : '')
              "
              :style="{
                'min-width': ['4.5', '6.5', '4'][Math.min(i, 2)] + 'rem',
              }"
              @click="i !== 1 && sortBy(i)"
            >
              <span v-if="sortKey.key === i" class="inline-block text-xs">
                {{ sortKey.order === "asc" ? "▲" : "▼" }}
              </span>
              {{ text }}
            </th>
          </tr>

          <tr style="height: calc(1.9rem + 1px)">
            <th
              scope="col"
              class="sticky left-0 z-40 underline bg-gray-100 cursor-pointer"
              @click="sortBy(0)"
            >
              {{ hoverContent?.name ?? "Total" }}
            </th>
            <template v-if="summary == null">
              <th scope="col"></th>
              <th
                v-for="i in headers.length - 2"
                :key="i"
                scope="col"
                class="cursor-pointer"
              >
                -
              </th>
            </template>
            <template v-else>
              <th
                scope="col"
                class="pt-[0.3125rem] pb-1 px-1"
                style="height: calc(1.9rem + 1px)"
              >
                <div
                  v-if="headers[1] === 'Strategy'"
                  class="w-full h-full bg-neutral-800 bg-left bg-no-repeat"
                  :style="{
                    'background-image': strategyBarBgImage(summary.slice(6)),
                  }"
                ></div>
              </th>
              <th
                v-for="(value, i) in summary.slice(2)"
                :key="i"
                scope="col"
                class="relative cursor-pointer"
                @click="sortBy(i + 2)"
              >
                <div class="inline-block w-12 text-right">
                  <span
                    v-if="results?.isEmpty && 1 <= i && i <= 3"
                    class="inline-block w-full text-center"
                    >-</span
                  >
                  <span v-else-if="i === 0">
                    {{ toFixedAdaptive(value).split(".")[0] }}.<span
                      class="text-xs"
                      >{{ toFixedAdaptive(value).split(".")[1] }}</span
                    >
                  </span>
                  <span v-else-if="i === 2">
                    {{ toFixedEv(value).slice(0, -evDigits)
                    }}<span class="text-xs">{{
                      toFixedEv(value).slice(-evDigits)
                    }}</span>
                  </span>
                  <span v-else>
                    {{ toFixed1(value * 100).slice(0, -1)
                    }}<span class="text-xs"
                      >{{ toFixed1(value * 100).slice(-1) }}%</span
                    >
                  </span>
                </div>
                <div
                  v-if="i >= 4"
                  class="absolute w-12 h-1 bottom-0 inset-x-0 mx-auto"
                  :style="{
                    background: actionBarBg(i - 4, value),
                  }"
                ></div>
              </th>
            </template>
          </tr>
        </thead>

        <tbody>
          <!-- Top empty row -->
          <tr
            v-if="emptyBufferTop > 0"
            :style="{
              '--num-rows': emptyBufferTop,
              height: 'calc(var(--num-rows) * (1.9rem + 1px))',
            }"
          >
            <td :colspan="headers.length"></td>
          </tr>

          <!-- Body -->
          <tr
            v-for="item in resultRendered"
            :key="item[0]"
            class="relative bg-white row-divider"
            style="height: calc(1.9rem + 1px)"
          >
            <td class="sticky left-0 z-10 bg-white">
              <template
                v-for="card in textPair(item[0])"
                :key="card.rank + card.suit"
              >
                <span :class="card.colorClass">
                  {{ card.rank + card.suit }}
                </span>
              </template>
            </td>
            <td
              class="pt-[0.3125rem] pb-1 px-1"
              style="height: calc(1.9rem + 1px)"
            >
              <div
                class="w-full h-full bg-neutral-800 bg-left bg-no-repeat"
                :style="{
                  'background-image': strategyBarBgImage(item.slice(6)),
                  'background-size': `${item[2] * 100}%`,
                }"
              ></div>
            </td>
            <td v-for="(value, i) in item.slice(2)" :key="i" class="relative">
              <div class="inline-block w-12 text-right">
                <span
                  v-if="results?.isEmpty && 1 <= i && i <= 3"
                  class="inline-block w-full text-center"
                >
                  -</span
                >
                <span v-else-if="i === 2">
                  {{ toFixedEv(value).slice(0, -evDigits)
                  }}<span class="text-xs">{{
                    toFixedEv(value).slice(-evDigits)
                  }}</span>
                </span>
                <span v-else>
                  {{ toFixed1(value * 100).slice(0, -1)
                  }}<span class="text-xs"
                    >{{ toFixed1(value * 100).slice(-1) }}%</span
                  >
                </span>
              </div>
              <div
                v-if="i >= 4"
                class="absolute w-12 h-1 bottom-0 inset-x-0 mx-auto"
                :style="{
                  background: actionBarBg(i - 4, value),
                }"
              ></div>
            </td>
          </tr>

          <!-- No results -->
          <tr v-if="resultRendered.length === 0">
            <td
              class="relative bg-white row-divider"
              style="height: calc(1.9rem + 1px)"
              :colspan="headers.length"
            >
              No results
            </td>
          </tr>

          <!-- Bottom empty row -->
          <tr
            v-if="emptyBufferBottom > 0"
            :style="{
              '--num-rows': emptyBufferBottom,
              height: 'calc(var(--num-rows) * (1.9rem + 1px))',
            }"
          >
            <td :colspan="headers.length"></td>
          </tr>

          <!-- Spacer -->
          <tr>
            <td :colspan="headers.length"></td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, ref, toRefs, watch } from "vue";
import {
  ranks,
  cardText,
  cardPairOrder,
  toFixed1,
  toFixed,
  toFixedAdaptive,
} from "../utils";

import {
  Results,
  Spot,
  SpotChance,
  SpotPlayer,
  HoverContent,
  TableMode,
} from "../result-types";

import { Tippy } from "vue-tippy";
import { ArrowTopRightOnSquareIcon } from "@heroicons/vue/24/solid";

const amber500 = "#f59e0b";
const neutral800 = "#262626";

const suits = ["c", "d", "h", "s"];

const cardStr = (card: number) => {
  const rank = ranks[card >>> 2];
  const suit = suits[card & 3];
  return rank + suit;
};

export default defineComponent({
  components: {
    Tippy,
    ArrowTopRightOnSquareIcon,
  },

  props: {
    tableMode: {
      type: String as () => TableMode,
      required: true,
    },
    cards: {
      type: Array as () => Uint16Array[] | null,
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
      type: Object as () => Results | null,
      required: true,
    },
    displayPlayer: {
      type: String as () => "oop" | "ip",
      required: true,
    },
    hoverContent: {
      type: Object as () => HoverContent | null,
      required: true,
    },
  },

  setup(props) {
    const tableDiv = ref<HTMLDivElement | null>(null);
    const tableHeight = ref(0);

    const assignTableHeight = () => {
      if (tableDiv.value) {
        tableHeight.value = tableDiv.value.clientHeight;
      }
    };

    watch(tableDiv, assignTableHeight);
    window.addEventListener("resize", assignTableHeight);

    const textPair = (pair: number) => {
      const card1 = pair & 0xff;
      const card2 = pair >>> 8;
      if (card2 !== 0xff) {
        return [cardText(card2), cardText(card1)];
      } else {
        return [cardText(card1)];
      }
    };

    type Key = { key: number; order: "asc" | "desc" };
    const sortKey = ref<Key>({ key: 0, order: "desc" });

    const sortBy = (key: number) => {
      const order =
        key === sortKey.value.key && sortKey.value.order === "desc"
          ? "asc"
          : "desc";
      sortKey.value = { key, order };
    };

    const { selectedSpot, displayPlayer } = toRefs(props);
    watch([selectedSpot, displayPlayer], () => {
      if (sortKey.value.key >= 6) {
        sortKey.value = { key: 0, order: "desc" };
      }
    });

    const evDigits = computed(() => {
      const results = props.results;
      if (!results || results.isEmpty) return 1;
      const playerIndex = props.displayPlayer === "oop" ? 0 : 1;
      const maxEv = Math.max(...results.ev[playerIndex]);
      return maxEv < 10 ? 3 : maxEv < 100 ? 2 : 1;
    });

    const toFixedEv = computed(() => {
      return toFixed[evDigits.value - 1];
    });

    const hasStrategy = computed(() => {
      const spot = props.selectedSpot;
      return (
        spot.type === "player" &&
        spot.player === props.displayPlayer &&
        !props.selectedChance
      );
    });

    const headers = computed(() => {
      let firstColumn = "";
      if (props.tableMode === "basics") {
        firstColumn = "Hand";
      } else if (props.tableMode === "turn") {
        firstColumn = "Turn";
      } else {
        firstColumn = "River";
      }

      const secondColumn = hasStrategy.value ? "Strategy" : "Weight (Bar)";
      const ret = [firstColumn, secondColumn, "Weight", "EQ", "EV", "EQR"];

      if (hasStrategy.value) {
        const spot = props.selectedSpot as SpotPlayer;
        for (let i = spot.actions.length - 1; i >= 0; --i) {
          const action = spot.actions[i];
          ret.push(
            action.amount === "0"
              ? action.name
              : `${action.name[0]} ${action.amount}`
          );
        }
      }

      return ret;
    });

    const resultFiltered = computed(() => {
      const ret: number[][] = [];

      const playerIndex = props.displayPlayer === "oop" ? 0 : 1;

      let cards = new Uint16Array();
      if (props.cards) {
        cards = props.cards[playerIndex];
      } else {
        const cardsArray = Array.from({ length: 52 }, (_, i) => 0xff00 + i);
        cards = new Uint16Array(cardsArray);
      }

      const dataRow = (index: number) => {
        const results = props.results;
        if (!results) return null;

        const weight = results.weights[playerIndex][index];
        const normalizer = results.normalizer[playerIndex][index];
        if (weight < 0.0005 || normalizer === 0) return null;

        const ret: number[] = [];
        ret.push(cards[index]);
        ret.push(results.normalizer[playerIndex][index]); // utilize strategy's position
        ret.push(results.weights[playerIndex][index]);
        ret.push(results.equity[playerIndex][index] ?? 0);
        ret.push(results.ev[playerIndex][index] ?? 0);
        ret.push(results.eqr[playerIndex][index] ?? 0);

        if (hasStrategy.value) {
          const spot = props.selectedSpot as SpotPlayer;
          const numActions = spot.actions.length;
          for (let i = numActions - 1; i >= 0; --i) {
            ret.push(results.strategy[i * cards.length + index]);
          }
        }

        return ret;
      };

      if (props.hoverContent) {
        for (const index of props.hoverContent.indices) {
          const data = dataRow(index);
          if (data) ret.push(data);
        }
      } else {
        for (let i = 0; i < cards.length; ++i) {
          const data = dataRow(i);
          if (data) ret.push(data);
        }
      }

      return ret;
    });

    const resultSorted = computed(() => {
      const { key, order } = sortKey.value;
      const coef = order === "asc" ? 1 : -1;
      const ret = [...resultFiltered.value];

      if (key === 0) {
        ret.sort((a, b) => {
          return coef * (cardPairOrder(a[0]) - cardPairOrder(b[0]));
        });
      } else {
        ret.sort((a, b) => {
          return (
            coef *
            (a[key] - b[key] || cardPairOrder(a[0]) - cardPairOrder(b[0]))
          );
        });
      }

      return ret;
    });

    const rem = Number(
      getComputedStyle(document.documentElement).fontSize.slice(0, -2)
    );
    const rowHeight = 1.9 * rem + 1;

    const bufferUnit = 10;
    const numDisplayedRows = computed(() =>
      Math.ceil(tableHeight.value / rowHeight)
    );

    const emptyBufferTop = ref(-2 * bufferUnit);
    const emptyBufferBottom = computed(
      () =>
        resultFiltered.value.length -
        (emptyBufferTop.value + numDisplayedRows.value + 4 * bufferUnit)
    );

    watch(resultSorted, () => {
      if (!tableDiv.value) return;
      tableDiv.value.scrollTop = 0;
      emptyBufferTop.value = -2 * bufferUnit;
    });

    let ticking = false;

    const onTableScroll = async () => {
      if (ticking) return;
      ticking = true;

      requestAnimationFrame(() => {
        ticking = false;
        if (!tableDiv.value) return;

        const { scrollTop } = tableDiv.value;
        const topIndex = Math.max(scrollTop / rowHeight, 0);
        if (topIndex < emptyBufferTop.value + bufferUnit) {
          emptyBufferTop.value =
            (Math.floor(topIndex / bufferUnit) - 1) * bufferUnit;
        } else if (topIndex > emptyBufferTop.value + 3 * bufferUnit) {
          emptyBufferTop.value =
            (Math.floor(topIndex / bufferUnit) - 2) * bufferUnit;
        }
      });
    };

    const resultRendered = computed(() => {
      return resultSorted.value.slice(
        Math.max(emptyBufferTop.value, 0),
        emptyBufferTop.value + numDisplayedRows.value + 4 * bufferUnit
      );
    });

    const summary = computed(() => {
      if (!props.results) return;
      if (resultFiltered.value.length === 0) return null;

      let weightSum = 0;
      const ret = resultFiltered.value[0].map(() => 0);

      for (const row of resultFiltered.value) {
        ret[0] += row[0];
        weightSum += row[1];
        ret[2] += row[2];
        for (let i = 3; i < row.length; ++i) {
          ret[i] += row[i] * row[1];
        }
      }

      for (let i = 3; i < ret.length; ++i) {
        ret[i] /= weightSum;
      }

      const playerIndex = props.displayPlayer === "oop" ? 0 : 1;
      ret[5] = ret[4] / (props.results.eqrBase[playerIndex] * ret[3]);

      return ret;
    });

    const actionColors = computed(() => {
      if (!hasStrategy.value) return [];
      const spot = props.selectedSpot as SpotPlayer;
      return spot.actions.map((a) => a.color).reverse();
    });

    const strategyBarBgImage = (strategy: number[]) => {
      if (strategy.length === 0) {
        return `linear-gradient(${amber500} 0% 100%)`;
      }
      let pos = 0;
      let ret = "linear-gradient(to right";
      for (let i = 0; i < strategy.length; ++i) {
        const width = strategy[i] * 100;
        const color = actionColors.value[i];
        ret += `, ${color} ${pos}% ${pos + width}%`;
        pos += width;
      }
      ret += ")";
      return ret;
    };

    const actionBarBg = (index: number, value: number) => {
      const color = actionColors.value[index];
      return `linear-gradient(to right, ${color} 0% ${
        value * 100
      }%, ${neutral800} ${value * 100}% 100%)`;
    };

    const exportSummaryButton = ref<HTMLAnchorElement | null>(null);

    const exportSummary = () => {
      if (!props.results || !exportSummaryButton.value) return;

      const h = headers.value;
      const startIndex = props.results.isEmpty ? 6 : 3;
      const ary = [h[0], "Weight", "Combinations", ...h.slice(startIndex)];
      const data = [ary.join(",")];

      for (const row of resultSorted.value) {
        const card1 = row[0] & 0xff;
        const card2 = row[0] >>> 8;
        const pairStr = cardStr(card2) + cardStr(card1);
        const ary = [pairStr, row[2], row[1], ...row.slice(startIndex)];
        data.push(ary.join(","));
      }

      const blob = new Blob([data.join("\n")], { type: "text/csv" });
      const url = URL.createObjectURL(blob);
      exportSummaryButton.value.href = url;
    };

    return {
      toFixed1,
      toFixedAdaptive,
      textPair,
      tableDiv,
      sortKey,
      sortBy,
      evDigits,
      toFixedEv,
      headers,
      onTableScroll,
      resultRendered,
      emptyBufferTop,
      emptyBufferBottom,
      summary,
      strategyBarBgImage,
      actionBarBg,
      exportSummaryButton,
      exportSummary,
    };
  },
});
</script>

<style scoped>
.row-divider::after {
  content: "";
  @apply absolute left-0 -bottom-px w-full z-20 border-b border-gray-300;
}
</style>
