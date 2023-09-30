<template>
  <div class="flex flex-col w-full border-l border-gray-500 overflow-x-auto">
    <div class="flex shrink-0 h-12 border-b border-gray-500">
      <div class="flex h-full px-4 items-center text-lg font-semibold">
        Summary
      </div>

      <div class="flex h-full ml-auto pr-4 items-center gap-4 snug">
        <div class="flex flex-col items-start justify-center h-full">
          <div class="text-sm">Bar width:</div>
          <select
            v-model="displayOptions.barWidth"
            class="w-28 px-1 py-0.5 border-gray-600 bg-gray-200 rounded-lg shadow cursor-pointer bg-right"
            @change="updateDisplayOptions"
          >
            <option value="normalized">Normalized</option>
            <option v-if="tableMode !== 'chance'" value="absolute">
              Absolute
            </option>
            <option value="full">Full</option>
          </select>
        </div>

        <div
          v-if="tableMode !== 'chance'"
          class="flex flex-col items-start justify-center h-full"
        >
          <div class="text-sm">Display:</div>
          <select
            v-model="(displayOptions as DisplayOptionsBasics).content"
            class="w-28 px-1 py-0.5 border-gray-600 bg-gray-200 rounded-lg shadow cursor-pointer bg-right"
            @change="updateDisplayOptions"
          >
            <option value="percentage">Action %</option>
            <option value="ev">Action EV</option>
          </select>
        </div>

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
        <thead class="sticky top-0 z-30 bg-gray-100 shadow">
          <tr style="height: calc(1.9rem + 1px)">
            <th
              v-for="column in columns"
              :key="column.label"
              scope="col"
              :class="
                'whitespace-nowrap select-none ' +
                (column.type === 'card'
                  ? 'sticky left-0 z-40 bg-gray-100 '
                  : '') +
                (column.type !== 'bar' ? 'cursor-pointer ' : '') +
                (tableMode === 'chance' ? 'header-divider' : '')
              "
              :style="{
                'min-width':
                  (column.type === 'card'
                    ? '4'
                    : column.type === 'bar'
                    ? '6'
                    : '3.5') + 'rem',
              }"
              @click="column.type !== 'bar' && sortBy(columnIndex(column))"
            >
              <span
                v-if="sortKey.key === columnIndex(column)"
                class="inline-block text-xs pr-1"
              >
                {{ sortKey.order === "asc" ? "▲" : "▼" }}
              </span>
              <span>{{ column.label }}</span>
            </th>
          </tr>

          <tr v-if="tableMode !== 'chance'" style="height: calc(1.9rem + 1px)">
            <th
              v-for="column in columns"
              :key="column.label"
              scope="col"
              :class="
                'header-divider ' +
                (column.type === 'card'
                  ? 'sticky left-0 z-40 underline bg-gray-100 '
                  : 'relative ') +
                (column.type === 'bar'
                  ? 'pt-[0.3125rem] pb-1 px-1'
                  : 'cursor-pointer')
              "
              :style="{
                height: column.type === 'bar' ? 'calc(1.9rem + 1px)' : 'auto',
              }"
              @click="column.type !== 'bar' && sortBy(columnIndex(column))"
            >
              <template v-if="column.type === 'card'">
                <span>{{ hoverContent?.name ?? "Total" }}</span>
              </template>

              <template v-else-if="column.type === 'bar'">
                <div
                  v-if="summary && column.label === 'Strategy'"
                  class="w-full h-full bg-neutral-800 bg-left bg-no-repeat"
                  :style="{
                    'background-image': strategyBarBgImage(summary),
                  }"
                ></div>
              </template>

              <template
                v-else-if="
                  summary == null || isNaN(summary[columnIndex(column)])
                "
              >
                <span>-</span>
              </template>

              <template v-else>
                <div class="inline-block w-12 text-right">
                  <span
                    v-if="column.type === 'weight'"
                    :data-set="
                      (strTmp = toFixedAdaptive(summary[columnIndex(column)]))
                    "
                  >
                    <span>{{ strTmp.slice(0, strTmp.indexOf(".") + 1) }}</span>
                    <span class="text-xs">{{
                      strTmp.slice(strTmp.indexOf(".") + 1)
                    }}</span>
                  </span>
                  <span
                    v-else-if="
                      column.type === 'percentage' || column.type === 'action'
                    "
                    :data-set="
                      (strTmp = toFixed1(summary[columnIndex(column)] * 100))
                    "
                  >
                    <span>{{ strTmp.slice(0, -1) }}</span>
                    <span class="text-xs">{{ strTmp.slice(-1) }}%</span>
                  </span>
                  <span
                    v-else-if="column.type === 'action-ev'"
                    :data-set="
                      (strTmp = toFixed1(
                        summary[columnIndex(column) - 1] * 100
                      ))
                    "
                  >
                    <span>{{ strTmp.slice(0, -1) }}</span>
                    <span class="text-xs">{{ strTmp.slice(-1) }}%</span>
                  </span>
                  <span
                    v-else-if="column.type === 'ev'"
                    :data-set="
                      (strTmp = toFixed[evDigits - 1](
                        summary[columnIndex(column)]
                      ))
                    "
                  >
                    <span>{{ strTmp.slice(0, -evDigits) }}</span>
                    <span class="text-xs">{{ strTmp.slice(-evDigits) }}</span>
                  </span>
                </div>
              </template>

              <div
                v-if="
                  summary &&
                  (column.type === 'action' || column.type === 'action-ev')
                "
                class="absolute w-12 h-1 bottom-0 inset-x-0 mx-auto"
                :style="{
                  background: actionBarBg(column.index, summary),
                }"
              ></div>
            </th>
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
            <td :colspan="columns.length"></td>
          </tr>

          <!-- Body -->
          <tr
            v-for="item in resultsRendered"
            :key="item[0]"
            :class="
              'relative ' +
              (item[0] === scrollTarget ? 'bg-yellow-200' : 'bg-gray-50')
            "
            style="height: calc(1.9rem + 1px)"
          >
            <td
              v-for="column in columns"
              :key="column.label"
              :class="
                'row-divider ' +
                (column.type === 'card'
                  ? 'sticky left-0 z-10 ' +
                    (item[0] === scrollTarget
                      ? 'bg-yellow-200 '
                      : 'bg-gray-50 ')
                  : 'relative ') +
                (column.type === 'bar' ? 'pt-[0.3125rem] pb-1 px-1' : 'pt-0.5')
              "
              :style="{
                height: column.type === 'bar' ? 'calc(1.9rem + 1px)' : 'auto',
              }"
            >
              <template v-if="column.type === 'card'">
                <span
                  v-for="card in pairText(item[columnIndex(column)])"
                  :key="card.rank + card.suit"
                  :class="card.colorClass"
                >
                  {{ card.rank + card.suit }}
                </span>
              </template>

              <template v-else-if="column.type === 'bar'">
                <div
                  class="w-full h-full bg-neutral-800 bg-left bg-no-repeat"
                  :style="{
                    'background-image': strategyBarBgImage(item),
                    'background-size': strategyBarBgSize(item),
                  }"
                ></div>
              </template>

              <template v-else-if="isNaN(item[columnIndex(column)])">
                <span>-</span>
              </template>

              <template v-else>
                <div class="inline-block w-12 text-right">
                  <span
                    v-if="tableMode === 'chance' && column.type === 'weight'"
                    :data-set="
                      (strTmp = toFixedAdaptive(item[columnIndex(column)]))
                    "
                  >
                    <span>{{ strTmp.slice(0, strTmp.indexOf(".") + 1) }}</span>
                    <span class="text-xs">{{
                      strTmp.slice(strTmp.indexOf(".") + 1)
                    }}</span>
                  </span>
                  <span
                    v-else-if="
                      column.type === 'weight' ||
                      column.type === 'percentage' ||
                      column.type === 'action'
                    "
                    :data-set="
                      (strTmp = toFixed1(item[columnIndex(column)] * 100))
                    "
                  >
                    <span>{{ strTmp.slice(0, -1) }}</span>
                    <span class="text-xs">{{ strTmp.slice(-1) }}%</span>
                  </span>
                  <span
                    v-else-if="
                      column.type === 'ev' || column.type === 'action-ev'
                    "
                    :data-set="
                      (strTmp = toFixed[evDigits - 1](
                        item[columnIndex(column)]
                      ))
                    "
                  >
                    <span>{{ strTmp.slice(0, -evDigits) }}</span>
                    <span class="text-xs">{{ strTmp.slice(-evDigits) }}</span>
                  </span>
                </div>
              </template>

              <div
                v-if="column.type === 'action' || column.type === 'action-ev'"
                class="absolute w-12 h-1 bottom-0 inset-x-0 mx-auto"
                :style="{
                  background: actionBarBg(column.index, item),
                }"
              ></div>
            </td>
          </tr>

          <!-- No results -->
          <tr v-if="resultsRendered.length === 0">
            <td
              class="relative bg-gray-50 row-divider"
              style="height: calc(1.9rem + 1px)"
              :colspan="columns.length"
            >
              {{
                tableMode === "chance"
                  ? `${capitalize(chanceType)} reports not available`
                  : "No results"
              }}
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
            <td :colspan="columns.length" class="relative row-divider"></td>
          </tr>

          <!-- Spacer -->
          <tr>
            <td :colspan="columns.length" class="relative row-divider"></td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, reactive, ref, toRefs, watch } from "vue";

import {
  ranks,
  suitLetters,
  cardText,
  cardPairOrder,
  toFixed1,
  toFixed,
  toFixedAdaptive,
  capitalize,
} from "../utils";

import {
  Results,
  ChanceReports,
  Spot,
  SpotPlayer,
  contentGraphsList,
  HoverContent,
  TableMode,
} from "../result-types";

import { Tippy } from "vue-tippy";
import { ArrowTopRightOnSquareIcon } from "@heroicons/vue/24/solid";

const barWidthListBasics = ["normalized", "absolute", "full"] as const;
const contentListBasics = ["percentage", "ev"] as const;
type DisplayOptionsBasics = {
  barWidth: (typeof barWidthListBasics)[number];
  content: (typeof contentListBasics)[number];
};

const barWidthListChance = ["normalized", "full"] as const;
type DisplayOptionsChance = {
  barWidth: (typeof barWidthListChance)[number];
};

type ColumnCard = {
  label: string;
  type: "card";
};

type ColumnBar = {
  label: string;
  type: "bar";
};

type ColumnWeight = {
  label: string;
  type: "weight";
};

type ColumnPercentage = {
  label: string;
  type: "percentage";
  index: number;
};

type ColumnEv = {
  label: string;
  type: "ev";
};

type ColumnAction = {
  label: string;
  type: "action";
  index: number;
};

type ColumnActionEV = {
  label: string;
  type: "action-ev";
  index: number;
};

type Column =
  | ColumnCard
  | ColumnBar
  | ColumnWeight
  | ColumnPercentage
  | ColumnEv
  | ColumnAction
  | ColumnActionEV;

const INDEX_CARD_PAIR = 0;
const INDEX_WEIGHT = 1;
const INDEX_NORMALIZER = 2;
const INDEX_EQUITY = 3;
const INDEX_EV = 4;
const INDEX_EQR = 5;
const INDEX_STRATEGY_BASE = 6;

const columnIndex = (column: Column) => {
  switch (column.type) {
    case "card":
      return INDEX_CARD_PAIR;
    case "bar":
      return -1;
    case "weight":
      return INDEX_WEIGHT;
    case "percentage":
      return column.index;
    case "ev":
      return INDEX_EV;
    case "action":
      return INDEX_STRATEGY_BASE + column.index * 2;
    case "action-ev":
      return INDEX_STRATEGY_BASE + column.index * 2 + 1;
  }
};

const yellow500 = "#eab308";
const neutral800 = "#262626";

const cardStr = (card: number) => {
  const rank = ranks[card >>> 2];
  const suit = suitLetters[card & 3];
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
    graphType: {
      type: String as () => (typeof contentGraphsList)[number],
      default: "eq",
    },
    chanceType: {
      type: String as () => "turn" | "river",
      default: "turn",
    },
    cards: {
      type: Array as () => number[][] | null,
      default: null,
    },
    selectedSpot: {
      type: Object as () => Spot,
      required: true,
    },
    results: {
      type: Object as () => Results | null,
      default: null,
    },
    chanceReports: {
      type: Object as () => ChanceReports | null,
      default: null,
    },
    displayPlayer: {
      type: String as () => "oop" | "ip",
      required: true,
    },
    hoverContent: {
      type: Object as () => HoverContent | null,
      default: null,
    },
    scrollTarget: {
      type: Number as () => number | null,
      default: null,
    },
  },

  setup(props) {
    const displayOptions =
      props.tableMode !== "chance"
        ? reactive<DisplayOptionsBasics>({
            barWidth: "normalized",
            content: "percentage",
          })
        : reactive<DisplayOptionsChance>({
            barWidth: "normalized",
          });

    const optionsStorageKey = `display-options-table-${props.tableMode}`;
    const savedDisplayOptions = localStorage.getItem(optionsStorageKey);

    if (savedDisplayOptions) {
      if (props.tableMode !== "chance") {
        const saved = JSON.parse(savedDisplayOptions) as DisplayOptionsBasics;
        const options = displayOptions as DisplayOptionsBasics;
        if (barWidthListBasics.includes(saved.barWidth)) {
          options.barWidth = saved.barWidth;
        }
        if (contentListBasics.includes(saved.content)) {
          options.content = saved.content;
        }
      } else {
        const saved = JSON.parse(savedDisplayOptions) as DisplayOptionsChance;
        const options = displayOptions as DisplayOptionsChance;
        if (barWidthListChance.includes(saved.barWidth)) {
          options.barWidth = saved.barWidth;
        }
      }
    }

    const updateDisplayOptions = () => {
      localStorage.setItem(optionsStorageKey, JSON.stringify(displayOptions));
    };

    const tableDiv = ref<HTMLDivElement | null>(null);
    const tableHeight = ref(0);

    const assignTableHeight = () => {
      if (tableDiv.value) {
        tableHeight.value = tableDiv.value.clientHeight;
      }
    };

    watch(tableDiv, assignTableHeight);
    window.addEventListener("resize", assignTableHeight);

    const pairText = (pair: number) => {
      const card1 = pair & 0xff;
      const card2 = pair >>> 8;
      if (card2 !== 0xff) {
        return [cardText(card2), cardText(card1)];
      } else {
        return [cardText(card1)];
      }
    };

    const defaultSortKey = computed(() => {
      if (props.tableMode !== "graphs") return INDEX_CARD_PAIR;
      if (props.graphType === "eq") return INDEX_EQUITY;
      if (props.graphType === "ev") return INDEX_EV;
      return INDEX_EQR;
    });

    type Key = { key: number; order: "asc" | "desc" };
    const sortKey = ref<Key>({ key: defaultSortKey.value, order: "desc" });

    const sortStorageKey = `sort-key-table-${props.tableMode}`;
    const savedSortKey = sessionStorage.getItem(sortStorageKey);

    if (savedSortKey) {
      const saved = JSON.parse(savedSortKey) as Key;
      if (saved.key < INDEX_STRATEGY_BASE) {
        sortKey.value = saved;
      }
    }

    const sortBy = (key: number) => {
      const order =
        key === sortKey.value.key && sortKey.value.order === "desc"
          ? "asc"
          : "desc";
      sortKey.value = { key, order };
      sessionStorage.setItem(sortStorageKey, JSON.stringify(sortKey.value));
    };

    const resetSortKey = () => {
      if (sortKey.value.key >= INDEX_STRATEGY_BASE) {
        sortBy(defaultSortKey.value);
      }
    };

    const { selectedSpot, displayPlayer } = toRefs(props);
    watch([selectedSpot, displayPlayer], resetSortKey);

    if (props.tableMode !== "chance") {
      watch(
        () => (displayOptions as DisplayOptionsBasics).content,
        resetSortKey
      );
    }

    if (props.tableMode === "graphs") {
      watch(defaultSortKey, (newValue, oldValue) => {
        if (sortKey.value.key === oldValue) {
          const order = sortKey.value.order;
          sortKey.value = { key: newValue, order };
          sessionStorage.setItem(sortStorageKey, JSON.stringify(sortKey.value));
        }
      });
    }

    const maxEv = computed(() => {
      const playerIndex = props.displayPlayer === "oop" ? 0 : 1;
      if (props.tableMode !== "chance") {
        const results = props.results;
        if (!results || results.isEmpty) return 0;
        return Math.max(...results.ev[playerIndex].map((v) => Math.abs(v)));
      } else {
        const reports = props.chanceReports;
        if (!reports || reports.status.every((s) => s <= 1)) return 0;
        return Math.max(...reports.ev[playerIndex].map((v) => Math.abs(v)));
      }
    });

    const evDigits = computed(() => {
      return maxEv.value < 9.9995 ? 3 : maxEv.value < 99.995 ? 2 : 1;
    });

    const numActions = computed(() => {
      let data: Results | ChanceReports | null;
      if (props.tableMode !== "chance") {
        data = props.results;
      } else {
        data = props.chanceReports;
      }

      const spot = props.selectedSpot;
      return (
        (spot.type === "player" &&
          spot.player === props.displayPlayer &&
          data &&
          data.numActions) ||
        0
      );
    });

    const columns = computed(() => {
      const ret: Column[] = [];

      if (props.tableMode !== "chance") {
        ret.push({ label: "Hand", type: "card" });
        ret.push({
          label: numActions.value > 0 ? "Strategy" : "Weight (Bar)",
          type: "bar",
        });

        ret.push({ label: "Weight", type: "weight" });
        ret.push({ label: "EQ", type: "percentage", index: INDEX_EQUITY });
        ret.push({ label: "EV", type: "ev" });
        ret.push({ label: "EQR", type: "percentage", index: INDEX_EQR });

        const options = displayOptions as DisplayOptionsBasics;

        if (numActions.value > 0) {
          const spot = props.selectedSpot as SpotPlayer;
          for (let i = 0; i < numActions.value; ++i) {
            const j = numActions.value - i - 1; // reverse order
            const action = spot.actions[j];
            const label =
              action.amount === "0"
                ? action.name
                : `${action.name[0]} ${action.amount}`;
            if (options.content === "percentage") {
              ret.push({ label, type: "action", index: i });
            } else {
              ret.push({ label, type: "action-ev", index: i });
            }
          }
        }
      } else {
        ret.push({ label: capitalize(props.chanceType), type: "card" });
        ret.push({
          label: numActions.value > 0 ? "Strategy" : "Combos (Bar)",
          type: "bar",
        });

        ret.push({ label: "Combos", type: "weight" });
        ret.push({ label: "EQ", type: "percentage", index: INDEX_EQUITY });
        ret.push({ label: "EV", type: "ev" });
        ret.push({ label: "EQR", type: "percentage", index: INDEX_EQR });

        if (numActions.value > 0) {
          const spot = props.selectedSpot as SpotPlayer;
          for (let i = 0; i < numActions.value; ++i) {
            const j = numActions.value - i - 1;
            const action = spot.actions[j];
            const label =
              action.amount === "0"
                ? action.name
                : `${action.name[0]} ${action.amount}`;
            ret.push({ label, type: "action", index: i });
          }
        }
      }

      return ret;
    });

    const resultsFiltered = computed(() => {
      const ret: number[][] = [];

      const playerIndex = props.displayPlayer === "oop" ? 0 : 1;

      let cards: number[];
      if (props.tableMode !== "chance" && props.cards) {
        cards = props.cards[playerIndex];
      } else {
        cards = Array.from({ length: 52 }, (_, i) => 0xff00 + i);
      }

      let dataRow: (index: number) => number[] | null;

      if (props.tableMode !== "chance") {
        dataRow = (index: number) => {
          const results = props.results;
          if (!results) return null;

          const weight = results.weights[playerIndex][index];
          const normalizer = results.normalizer[playerIndex][index];
          if (weight === 0 || normalizer === 0) return null;

          const ret: number[] = [];

          ret.push(cards[index]);
          ret.push(results.weights[playerIndex][index]);
          ret.push(results.normalizer[playerIndex][index]);
          ret.push(results.equity[playerIndex][index] ?? Number.NaN);
          ret.push(results.ev[playerIndex][index] ?? Number.NaN);

          const eqr = results.eqr[playerIndex][index] ?? Number.NaN;
          ret.push(isFinite(eqr) ? eqr : Number.NaN);

          for (let i = numActions.value - 1; i >= 0; --i) {
            const j = i * cards.length + index;
            ret.push(results.strategy[j]);
            ret.push(results.actionEv[j] ?? 0);
          }

          return ret;
        };
      } else {
        dataRow = (index: number) => {
          const reports = props.chanceReports;
          if (!reports) return null;

          const status = reports.status[index];
          if (status === 0) return null;

          const ret: number[] = [];

          ret.push(cards[index]);
          ret.push(reports.combos[playerIndex][index]);
          ret.push(Number.NaN);

          if (status === 1) {
            ret.push(Number.NaN, Number.NaN, Number.NaN);
          } else {
            ret.push(reports.equity[playerIndex][index]);
            ret.push(reports.ev[playerIndex][index]);
            ret.push(reports.eqr[playerIndex][index]);
          }

          for (let i = numActions.value - 1; i >= 0; --i) {
            const j = i * cards.length + index;
            ret.push(reports.strategy[j]);
            ret.push(Number.NaN);
          }

          return ret;
        };
      }

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

    const resultsSorted = computed(() => {
      const { key, order } = sortKey.value;
      const ret = [...resultsFiltered.value];

      if (key === 0) {
        ret.sort((a, b) => {
          return cardPairOrder(a[0]) - cardPairOrder(b[0]);
        });
      } else {
        ret.sort((a, b) => {
          if (!isNaN(a[key]) && !isNaN(b[key])) {
            return a[key] - b[key] || cardPairOrder(a[0]) - cardPairOrder(b[0]);
          } else if (!isNaN(a[key])) {
            return -1;
          } else if (!isNaN(b[key])) {
            return 1;
          } else {
            return cardPairOrder(a[0]) - cardPairOrder(b[0]);
          }
        });
      }

      if (order === "desc") ret.reverse();
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
        resultsFiltered.value.length -
        (emptyBufferTop.value + numDisplayedRows.value + 4 * bufferUnit)
    );

    watch(resultsSorted, () => {
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

    const resultsRendered = computed(() => {
      return resultsSorted.value.slice(
        Math.max(emptyBufferTop.value, 0),
        emptyBufferTop.value + numDisplayedRows.value + 4 * bufferUnit
      );
    });

    const { scrollTarget } = toRefs(props);
    watch(scrollTarget, () => {
      if (!tableDiv.value || !scrollTarget.value) return;
      const scrollIndex = resultsSorted.value.findIndex(
        (row) => row[0] === scrollTarget.value
      );
      if (scrollIndex === -1) return;
      const scrollTop =
        scrollIndex * rowHeight - (tableHeight.value - 3 * rowHeight) / 2;
      tableDiv.value.scrollTop = Math.max(scrollTop, 0);
    });

    const summary = computed(() => {
      const results = resultsFiltered.value;
      if (!props.results || results.length === 0) return null;

      let normalizer = 0;
      const ret = results[0].map(() => 0);

      for (const row of results) {
        const n = row[INDEX_NORMALIZER];
        normalizer += n;
        ret[INDEX_WEIGHT] += row[INDEX_WEIGHT];
        for (let i = INDEX_EQUITY; i < row.length; ++i) {
          ret[i] += row[i] * n;
        }
      }

      for (let i = INDEX_EQUITY; i < ret.length; ++i) {
        ret[i] /= normalizer;
      }

      // EQR is not the average
      const playerIndex = props.displayPlayer === "oop" ? 0 : 1;
      const eqrBase = props.results.eqrBase[playerIndex];
      ret[INDEX_EQR] = ret[INDEX_EV] / (eqrBase * ret[INDEX_EQUITY]);
      if (!isFinite(ret[INDEX_EQR])) ret[INDEX_EQR] = Number.NaN;

      return ret;
    });

    const actionColors = computed(() => {
      if (numActions.value === 0) return [];
      const spot = props.selectedSpot as SpotPlayer;
      return spot.actions.map((a) => a.color).reverse();
    });

    const maxWeight = computed(() => {
      return Math.max(...resultsFiltered.value.map((r) => r[INDEX_WEIGHT]));
    });

    const strategyBarBgImage = (row: number[]) => {
      if (!row || row.length === INDEX_STRATEGY_BASE) {
        return `linear-gradient(${yellow500} 0% 100%)`;
      }

      let pos = 0;
      let ret = "linear-gradient(to right";

      for (let i = 0; i < numActions.value; ++i) {
        const width = row[INDEX_STRATEGY_BASE + i * 2] * 100;
        const color = actionColors.value[i];
        ret += `, ${color} ${pos}% ${pos + width}%`;
        pos += width;
      }

      ret += ")";
      return ret;
    };

    const strategyBarBgSize = (row: number[]) => {
      const weight = row[INDEX_WEIGHT];
      if (weight === 0) {
        return "0% 100%";
      } else if (displayOptions.barWidth === "normalized") {
        return `${(weight / maxWeight.value) * 100}% 100%`;
      } else if (displayOptions.barWidth === "absolute") {
        return `${weight * 100}% 100%`;
      } else {
        return "100% 100%";
      }
    };

    const actionBarBg = (index: number, row: number[]) => {
      const color = actionColors.value[index];
      const value = row[INDEX_STRATEGY_BASE + index * 2];
      return `linear-gradient(to right, ${color} 0% ${
        value * 100
      }%, ${neutral800} ${value * 100}% 100%)`;
    };

    const exportSummaryButton = ref<HTMLAnchorElement | null>(null);

    const exportSummary = () => {
      if (!exportSummaryButton.value) return;

      const data: string[] = [];

      if (props.tableMode !== "chance") {
        if (!props.results) return;

        const ary = ["Hand", "Weight", "Combos"];

        if (!props.results.isEmpty) {
          ary.push("Equity", "EV", "EQR");
        }

        if (numActions.value > 0) {
          const actions = (props.selectedSpot as SpotPlayer).actions;
          for (let i = actions.length - 1; i >= 0; --i) {
            const action = actions[i];
            const amount = action.amount === "0" ? "" : ` ${action.amount}`;
            ary.push(`${action.name}${amount} %`);
            ary.push(`${action.name}${amount} EV`);
          }
        }

        data.push(ary.join(","));

        const startIndex = props.results.isEmpty
          ? INDEX_STRATEGY_BASE
          : INDEX_EQUITY;

        for (const row of resultsSorted.value) {
          const card1 = row[0] & 0xff;
          const card2 = row[0] >>> 8;
          const pairStr = cardStr(card2) + cardStr(card1);
          const ary = [
            pairStr,
            row[INDEX_WEIGHT],
            row[INDEX_NORMALIZER],
            ...row.slice(startIndex),
          ];
          data.push(ary.join(","));
        }
      } else {
        if (!props.chanceReports) return;

        const ary = [columns.value[0].label, "Combos", "Equity", "EV", "EQR"];

        if (numActions.value > 0) {
          const actions = (props.selectedSpot as SpotPlayer).actions;
          for (let i = actions.length - 1; i >= 0; --i) {
            const action = actions[i];
            const amount = action.amount === "0" ? "" : ` ${action.amount}`;
            ary.push(`${action.name}${amount} %`);
          }
        }

        data.push(ary.join(","));

        for (const row of resultsSorted.value) {
          const ary = [
            cardStr(row[0] & 0xff),
            row[INDEX_WEIGHT],
            isNaN(row[INDEX_EQUITY]) ? "-" : row[INDEX_EQUITY],
            isNaN(row[INDEX_EV]) ? "-" : row[INDEX_EV],
            isNaN(row[INDEX_EQR]) ? "-" : row[INDEX_EQR],
            ...row.slice(INDEX_STRATEGY_BASE).filter((_, i) => i % 2 === 0),
          ];
          data.push(ary.join(","));
        }
      }

      const blob = new Blob([data.join("\n")], { type: "text/csv" });
      const url = URL.createObjectURL(blob);
      exportSummaryButton.value.href = url;
    };

    return {
      toFixed1,
      toFixed,
      toFixedAdaptive,
      capitalize,
      columnIndex,
      displayOptions,
      updateDisplayOptions,
      tableDiv,
      pairText,
      sortKey,
      sortBy,
      evDigits,
      columns,
      onTableScroll,
      resultsRendered,
      emptyBufferTop,
      emptyBufferBottom,
      summary,
      strategyBarBgImage,
      strategyBarBgSize,
      actionBarBg,
      exportSummaryButton,
      exportSummary,
      strTmp: "",
    };
  },
});
</script>

<style scoped>
.header-divider::before {
  content: "";
  @apply absolute left-0 -bottom-px w-full border-b border-gray-300;
}

.row-divider::before {
  content: "";
  @apply absolute left-0 top-0 w-full border-t border-gray-300;
}
</style>
