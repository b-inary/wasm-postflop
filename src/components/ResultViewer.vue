<template>
  <div v-if="!store.isSolverFinished" class="flex items-center">
    <span
      v-if="store.isSolverRunning || store.isFinalizing"
      class="spinner inline-block mr-3"
    ></span>
    {{
      !store.hasSolverRun
        ? "Solver has not run."
        : store.isSolverRunning
        ? "Solver running..."
        : store.isFinalizing
        ? "Finalizing..."
        : "Solver paused."
    }}
  </div>

  <div v-else>
    <div class="flex pl-0.5 overflow-x-auto">
      <div class="flex flex-col items-start">
        <div
          :class="
            'my-[3px] px-2 py-px border bg-white rounded-lg shadow select-none ' +
            (store.actions.length === 1
              ? 'ring-1 ring-red-600 border-red-600'
              : 'border-black cursor-pointer')
          "
        >
          <span class="inline-block mr-2 underline">Flop</span>
          <span
            v-for="item in boardTexts"
            :key="item.rank + item.suit"
            :class="item.colorClass"
          >
            {{ item.rank + item.suit }}
          </span>
        </div>
      </div>

      <div
        v-for="i in store.actions.length"
        :key="i"
        class="flex flex-col ml-2 items-start"
      >
        <template v-if="store.actions[i - 1].type !== 'Player'">
          <!-- Turn and River -->
        </template>
        <template v-else>
          <div
            v-for="action in store.actions[i - 1].candidates"
            :key="action.str"
            :class="
              'w-full my-[3px] px-2 py-px border rounded-lg shadow text-center select-none ' +
              (!action.isSelected && i < store.actions.length
                ? 'opacity-40 '
                : '') +
              (action.isSelected && i === store.actions.length - 1
                ? 'ring-1 ring-red-600 border-red-600 '
                : 'border-black cursor-pointer ') +
              (i === store.actions.length
                ? actionColorByStr[action.str]
                : 'bg-white')
            "
          >
            {{ action.str }}
          </div>
        </template>
      </div>
    </div>

    <div class="flex mt-5 items-start">
      <div class="shrink-0">
        <table class="ml-1 bg-gray-200 shadow" @mouseleave="onMouseLeave">
          <tr v-for="row in 13" :key="row" class="h-9">
            <td
              v-for="col in 13"
              :key="col"
              :class="
                'relative w-10 border-black select-none ' +
                (row === col ? 'border-2' : 'border')
              "
              @mouseover="onMouseOver(row, col)"
            >
              <div
                class="absolute bottom-0 left-0 w-full"
                :style="{ height: weightPercent(row, col) }"
              >
                <div
                  v-for="item in cellItems(row, col)"
                  :key="item.key"
                  :class="'absolute top-0 h-full ' + item.class"
                  :style="item.style"
                ></div>
              </div>
              <div class="absolute -top-px left-px z-10 text-sm">
                {{ cellText(row, col) }}
              </div>
              <div class="absolute -bottom-px right-px z-10 text-sm">
                {{ cellAuxText(row, col) }}
              </div>
            </td>
          </tr>
        </table>
      </div>

      <div
        class="ml-5 max-h-[470px] border border-gray-500 rounded-md shadow overflow-x-auto overflow-y-scroll"
      >
        <table class="relative divide-y divide-gray-300">
          <thead class="sticky top-0 bg-gray-100 shadow">
            <tr>
              <th
                v-for="text in headers"
                :key="text"
                scope="col"
                :class="
                  'px-3 py-[5px] whitespace-nowrap text-sm font-bold cursor-pointer select-none ' +
                  (text === 'Hand' ? 'text-center' : '')
                "
                @click="sortBy(text)"
              >
                <span
                  v-if="text === sortKey.key"
                  class="inline-block mr-0.5 text-xs"
                >
                  {{ sortKey.order === "asc" ? "▲" : "▼" }}
                </span>
                {{
                  text
                    .replace("Bet", "B")
                    .replace("Raise", "R")
                    .replace("All-in", "A")
                }}
              </th>
            </tr>
          </thead>

          <tbody class="bg-white divide-y divide-gray-300">
            <tr
              v-for="item in resultSorted"
              :key="item.card1 + '-' + item.card2"
              class="text-right text-sm"
            >
              <td class="px-3 py-[5px] text-center">
                <template
                  v-for="card in [item.card1, item.card2].map(cardText)"
                  :key="card.rank + card.suit"
                >
                  <span :class="card.colorClass">
                    {{ card.rank + card.suit }}
                  </span>
                </template>
              </td>
              <td class="px-3 py-[5px]">
                {{ (100 * item.weight).toFixed(1) }}%
              </td>
              <td class="px-3 py-[5px]">
                {{
                  (
                    item.expectedValue / item.weightNormalized +
                    store.startingPot / 2
                  ).toFixed(1)
                }}
              </td>
              <td
                v-for="i in item.strategy.length"
                :key="i"
                class="px-3 py-[5px]"
              >
                {{ (100 * item.strategy[i - 1]).toFixed(1) }}%
              </td>
            </tr>
          </tbody>

          <tfoot class="sticky bottom-0 font-bold bg-white shadow">
            <tr class="text-right text-sm">
              <th scope="col" class="px-3 py-[5px] text-center underline">
                {{ hoveredCellText }}
              </th>
              <th scope="col" class="px-3 py-[5px]">
                {{ resultAverage.combos }}
              </th>
              <th scope="col" class="px-3 py-[5px]">
                {{ resultAverage.expectedValue }}
              </th>
              <th
                v-for="i in resultAverage.strategy.length"
                :key="i"
                scope="col"
                class="px-3 py-[5px]"
              >
                {{ resultAverage.strategy[i - 1] }}
              </th>
            </tr>
          </tfoot>
        </table>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, ref } from "vue";
import { cardText, useStore } from "../store";

const ranks = ["2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"];

export default defineComponent({
  setup() {
    const store = useStore();

    const hoveredCell = ref({ row: 0, col: 0 });

    type Order = "asc" | "desc";
    const sortKey = ref({ key: "Hand", order: "desc" as Order });

    const boardTexts = computed(() => store.board.map(cardText));

    const nextActionsStr = computed(() => {
      const actions = store.actions.slice(-1)[0];
      if (actions.type === "Player") {
        return actions.candidates.map((c) => c.str);
      } else {
        return [];
      }
    });

    const actionColor = computed(() => {
      if (store.actions.slice(-1)[0].type !== "Player") {
        return [];
      }

      const ret = [];
      const actions = nextActionsStr.value;
      const subtract = actions.slice(-1)[0] === "Check" ? 1 : 2;

      for (let i = 0; i < actions.length; ++i) {
        let color;
        if (actions[i] === "Fold") {
          color = "bg-sky-300";
        } else if (actions[i] === "Check" || actions[i] === "Call") {
          color = "bg-green-300";
        } else {
          color = ["bg-amber-300", "bg-pink-300", "bg-violet-300"][
            (actions.length - subtract - i - 1) % 3
          ];
        }
        ret.push(color);
      }

      return ret;
    });

    const actionColorByStr = computed(() => {
      const ret = {} as { [key: string]: string };
      const actions = nextActionsStr.value;
      for (let i = 0; i < actionColor.value.length; ++i) {
        ret[actions[i]] = actionColor.value[i];
      }
      return ret;
    });

    const weightPercent = (row: number, col: number) => {
      const idx = (row - 1) * 13 + col - 1;
      return (100 * store.resultSummary[idx].weight).toFixed(1) + "%";
    };

    const cellItems = (row: number, col: number) => {
      const idx = (row - 1) * 13 + col - 1;
      const strategy = store.resultSummary[idx].strategy;
      const ret = [];
      for (let i = 0; i < strategy.length; ++i) {
        ret.push({
          key: i,
          class: actionColor.value[i],
          style: {
            width: (100 * strategy[i]).toFixed(1) + "%",
            left:
              (100 * strategy.slice(0, i).reduce((a, b) => a + b, 0)).toFixed(
                1
              ) + "%",
          },
        });
      }
      return ret;
    };

    const cellText = (row: number, col: number) => {
      const r1 = 13 - Math.min(row, col);
      const r2 = 13 - Math.max(row, col);
      return ranks[r1] + ranks[r2] + ["s", "", "o"][Math.sign(row - col) + 1];
    };

    const cellAuxText = (row: number, col: number) => {
      const idx = (row - 1) * 13 + col - 1;
      if (!store.resultSummary[idx].enabled) return "";
      if (sortKey.value.key === "EV") {
        const ev =
          store.resultSummary[idx].expectedValue + store.startingPot / 2;
        return (ev >= 0 ? "+" : "") + ev.toFixed(0);
      } else {
        const i = nextActionsStr.value.indexOf(sortKey.value.key);
        if (i !== -1) {
          return (100 * store.resultSummary[idx].strategy[i]).toFixed(0) + "%";
        }
      }
      return "";
    };

    const onMouseOver = (row: number, col: number) => {
      hoveredCell.value = { row, col };
    };

    const onMouseLeave = () => {
      hoveredCell.value = { row: 0, col: 0 };
    };

    const hoveredCellText = computed(() => {
      const { row, col } = hoveredCell.value;
      const idx = (row - 1) * 13 + col - 1;
      if (idx < 0 || !store.resultSummary[idx].enabled) return "All";
      return cellText(row, col);
    });

    const headers = computed(() => {
      return ["Hand", "Weight", "EV"].concat(nextActionsStr.value);
    });

    const resultFiltered = computed(() => {
      const { row, col } = hoveredCell.value;
      const idx = (row - 1) * 13 + col - 1;
      if (idx < 0 || !store.resultSummary[idx].enabled) {
        return store.result.filter((r) => r.weight >= 0.5 / 100);
      }
      const r1 = 13 - Math.min(row, col);
      const r2 = 13 - Math.max(row, col);
      const isSuited = row < col;
      return store.result.filter((r) => {
        return (
          r.weight >= 0.5 / 100 &&
          Math.floor(r.card1 / 4) === r1 &&
          Math.floor(r.card2 / 4) === r2 &&
          (r.card1 % 4 === r.card2 % 4) === isSuited
        );
      });
    });

    const sortBy = (key: string) => {
      if (sortKey.value.key === key) {
        sortKey.value.order = sortKey.value.order === "asc" ? "desc" : "asc";
      } else {
        sortKey.value.key = key;
        sortKey.value.order = "desc";
      }
    };

    const resultSorted = computed(() => {
      const rankComparator = (
        a1: number,
        a2: number,
        b1: number,
        b2: number,
        order: Order
      ) => {
        const coef = order === "asc" ? 1 : -1;
        const ar1 = Math.floor(a1 / 4);
        const as1 = a1 % 4;
        const ar2 = Math.floor(a2 / 4);
        const as2 = a2 % 4;
        const br1 = Math.floor(b1 / 4);
        const bs1 = b1 % 4;
        const br2 = Math.floor(b2 / 4);
        const bs2 = b2 % 4;
        return (
          coef * (ar1 - br1) ||
          coef * (ar2 - br2) ||
          Number(bs1 === bs2) - Number(as1 === as2) ||
          bs1 - as1 ||
          bs2 - as2
        );
      };

      const round = (x: number) => Math.round(10 * x);

      return [...resultFiltered.value].sort((a, b) => {
        const coef = sortKey.value.order === "asc" ? 1 : -1;
        if (sortKey.value.key === "Hand") {
          return rankComparator(
            a.card1,
            a.card2,
            b.card1,
            b.card2,
            sortKey.value.order
          );
        } else {
          const fallback = rankComparator(
            a.card1,
            a.card2,
            b.card1,
            b.card2,
            "desc"
          );
          if (sortKey.value.key === "Weight") {
            return (
              coef * (round(100 * a.weight) - round(100 * b.weight)) || fallback
            );
          } else if (sortKey.value.key === "EV") {
            return (
              coef *
                (round(a.expectedValue / a.weightNormalized) -
                  round(b.expectedValue / b.weightNormalized)) || fallback
            );
          } else {
            const idx = nextActionsStr.value.indexOf(sortKey.value.key);
            return (
              coef *
                (round(100 * a.strategy[idx]) - round(100 * b.strategy[idx])) ||
              fallback
            );
          }
        }
      });
    });

    const resultAverage = computed(() => {
      const result = resultFiltered.value;
      if (result.length === 0) {
        return {
          combos: "-",
          expectedValue: "-",
          strategy: nextActionsStr.value.map(() => "-"),
        };
      }
      let weightSum = 0;
      let combos = 0;
      let expectedValue = 0;
      let strategy = nextActionsStr.value.map(() => 0);
      for (const r of result) {
        weightSum += r.weightNormalized;
        combos += r.weight;
        expectedValue += r.expectedValue;
        for (let i = 0; i < nextActionsStr.value.length; i++) {
          strategy[i] += r.strategy[i] * r.weightNormalized;
        }
      }
      return {
        combos: combos.toFixed(1),
        expectedValue: (
          expectedValue / weightSum +
          store.startingPot / 2
        ).toFixed(1),
        strategy: strategy.map((x) => ((100 * x) / weightSum).toFixed(1) + "%"),
      };
    });

    return {
      store,
      cardText,
      boardTexts,
      actionColorByStr,
      weightPercent,
      cellItems,
      cellText,
      cellAuxText,
      onMouseOver,
      onMouseLeave,
      hoveredCellText,
      headers,
      sortBy,
      sortKey,
      resultSorted,
      resultAverage,
    };
  },
});
</script>
