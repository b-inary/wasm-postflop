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
    <div ref="resultNav" class="flex pl-0.5 overflow-x-auto">
      <div class="flex flex-col items-start">
        <div
          :class="
            'my-[3px] px-2 py-px whitespace-nowrap border bg-white rounded-lg shadow select-none ' +
            (actionList.length === 1
              ? 'ring-1 ring-red-600 border-red-600'
              : 'border-black cursor-pointer')
          "
          @click="moveResult(0, 0)"
        >
          <span class="inline-block mr-2 underline">Flop</span>
          <span
            v-for="item in flop.map(cardText)"
            :key="item.rank + item.suit"
            :class="item.colorClass"
          >
            {{ item.rank + item.suit }}
          </span>
        </div>
      </div>

      <div
        v-for="item in actionList"
        :key="item.depth"
        class="flex flex-col ml-2 items-start"
      >
        <template v-if="item.type !== 'Player'">
          <div
            :class="
              'my-[3px] px-2 py-px whitespace-nowrap border bg-white rounded-lg shadow select-none ' +
              (item.depth === actionList.length - 1
                ? 'ring-1 ring-red-600 border-red-600 '
                : 'border-black ') +
              (item.depth >= actionList.length - 1 ? '' : 'cursor-pointer')
            "
            @click="moveResult(item.depth, item.selectedIndex)"
          >
            <span class="inline-block mr-2 underline">
              {{ item.type }}
            </span>
            <span
              v-for="tmp in item.depth === actionList.length
                ? [{ rank: '?', suit: '', colorClass: 'text-black' }]
                : [cardText(item.selectedIndex)]"
              :key="tmp.rank + tmp.suit"
              :class="tmp.colorClass"
            >
              {{ tmp.rank + tmp.suit }}
            </span>
          </div>
        </template>
        <template v-else>
          <div
            v-for="action in item.actions"
            :key="action.str"
            :class="
              'w-full my-[3px] px-2 py-px whitespace-nowrap border rounded-lg shadow text-center select-none ' +
              (!action.isSelected && item.depth < actionList.length
                ? 'opacity-40 '
                : '') +
              (action.isSelected && item.depth === actionList.length - 1
                ? 'ring-1 ring-red-600 border-red-600 '
                : 'border-black ' +
                  (action.isTerminal ? '' : 'cursor-pointer ')) +
              (item.depth === actionList.length
                ? actionColorByStr[action.str]
                : 'bg-white')
            "
            @click="moveResult(item.depth, action.index)"
          >
            {{ action.str }}
          </div>
        </template>
      </div>
    </div>

    <div
      v-if="
        actionList.length === 0 || actionList.slice(-1)[0].type === 'Player'
      "
    >
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
                    :class="'absolute top-0 right-0 h-full ' + item.class"
                    :style="item.style"
                  ></div>
                </div>
                <div class="absolute -top-px left-px z-50 text-sm">
                  {{ cellText(row, col) }}
                </div>
                <div class="absolute -bottom-px right-px z-50 text-sm">
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
                    'py-[5px] whitespace-nowrap text-sm font-bold cursor-pointer select-none ' +
                    (text === 'Hand' ? 'text-center ' : '') +
                    (text === 'Weight' ? 'px-1.5' : 'px-2.5')
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
                <td class="px-2.5 py-[5px] text-center">
                  <template
                    v-for="card in [item.card1, item.card2].map(cardText)"
                    :key="card.rank + card.suit"
                  >
                    <span :class="card.colorClass">
                      {{ card.rank + card.suit }}
                    </span>
                  </template>
                </td>
                <td class="px-2.5 py-[5px]">
                  {{ percentStr(item.weight) }}
                </td>
                <td class="px-2.5 py-[5px]">
                  {{ percentStr(item.equity) }}
                </td>
                <td class="px-2.5 py-[5px]">
                  {{ trimMinusZero(item.expectedValue.toFixed(1)) }}
                </td>
                <td
                  v-for="i in item.strategy.length"
                  :key="i"
                  class="px-2.5 py-[5px]"
                >
                  {{ percentStr(item.strategy[i - 1]) }}
                </td>
              </tr>
            </tbody>

            <tfoot class="sticky bottom-0 font-bold bg-white shadow">
              <tr class="text-right text-sm">
                <th scope="col" class="px-2.5 py-[5px] text-center underline">
                  {{ hoveredCellText }}
                </th>
                <th scope="col" class="px-2.5 py-[5px]">
                  {{ resultAverage.combos }}
                </th>
                <th scope="col" class="px-2.5 py-[5px]">
                  {{ resultAverage.equity }}
                </th>
                <th scope="col" class="px-2.5 py-[5px]">
                  {{ resultAverage.expectedValue }}
                </th>
                <th
                  v-for="i in resultAverage.strategy.length"
                  :key="i"
                  scope="col"
                  class="px-2.5 py-[5px]"
                >
                  {{ resultAverage.strategy[i - 1] }}
                </th>
              </tr>
            </tfoot>
          </table>
        </div>
      </div>

      <div class="mt-4">
        Player: {{ nodeInformation.player === 0 ? "OOP" : "IP" }} / Pot:
        {{ nodeInformation.pot }} / Stack: {{ nodeInformation.stack }}
        {{
          nodeInformation.toCall ? " / To Call: " + nodeInformation.toCall : ""
        }}
      </div>
    </div>

    <div v-else class="mt-5">
      <div v-for="suit in 4" :key="suit" class="flex">
        <board-selector-card
          v-for="rank in 13"
          :key="rank"
          :class="
            'm-1 ' +
            (actionList.slice(-1)[0].actions[56 - 4 * rank - suit].isTerminal
              ? 'opacity-40 cursor-default'
              : '')
          "
          :card-id="56 - 4 * rank - suit"
          @click="moveResult(actionList.length, 56 - 4 * rank - suit)"
        />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, ref } from "vue";
import { cardText, useStore } from "../store";
import * as GlobalWorker from "../global-worker";

import BoardSelectorCard from "./BoardSelectorCard.vue";

const ranks = ["2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"];

export default defineComponent({
  components: {
    BoardSelectorCard,
  },

  setup() {
    const store = useStore();
    const resultNav = ref(null as HTMLDivElement | null);

    const isSolverFinished = ref(false);
    const flop = ref([] as number[]);
    const startingPot = ref(0);
    const effectiveStack = ref(0);
    const handCards = ref([[], []] as number[][][]);

    const actionList = ref(
      [] as {
        type: "Player" | "Turn" | "River";
        depth: number;
        selectedIndex: number;
        actions: {
          index: number;
          str: string;
          isSelected: boolean;
          isTerminal: boolean;
        }[];
      }[]
    );

    const result = ref(
      [] as {
        card1: number;
        card2: number;
        weight: number;
        weightNormalized: number;
        equity: number;
        expectedValue: number;
        strategy: number[];
      }[]
    );

    const resultCell = ref(
      [] as {
        count: number;
        weight: number;
        equity: number;
        expectedValue: number;
        strategy: number[];
      }[]
    );

    const nodeInformation = ref(
      {} as {
        player: number;
        pot: number;
        stack: number;
        toCall: number;
      }
    );

    const hoveredCell = ref({ row: 0, col: 0 });

    type Order = "asc" | "desc";
    const sortKey = ref({ key: "Hand", order: "desc" as Order });

    store.$subscribe(async (_, store) => {
      if (store.isSolverFinished !== isSolverFinished.value) {
        if ((isSolverFinished.value = store.isSolverFinished)) {
          await updateResult(0, true);
        } else {
          await clearResult();
        }
      }
    });

    const turn = computed(() => {
      const item = actionList.value.find((item) => item.type === "Turn");
      return item?.selectedIndex ?? -1;
    });

    const river = computed(() => {
      const item = actionList.value.find((item) => item.type === "River");
      return item?.selectedIndex ?? -1;
    });

    const percentStr = (num: number) => {
      if (num >= 0.9995) {
        return "100%";
      } else {
        return (100 * num).toFixed(1) + "%";
      }
    };

    const trimMinusZero = (str: string) => {
      if (Number(str) === 0 && str[0] === "-") {
        return str.slice(1);
      } else {
        return str;
      }
    };

    const sortBy = (key: string) => {
      if (sortKey.value.key === key) {
        sortKey.value.order = sortKey.value.order === "asc" ? "desc" : "asc";
      } else {
        sortKey.value = { key, order: "desc" };
      }
    };

    const clearResult = async () => {
      handCards.value = [[], []];
      actionList.value = [];
      result.value = [];
      resultCell.value = [];
    };

    const updateResult = async (depth: number, isFirstCall: boolean) => {
      const handler = await GlobalWorker.getHandler();

      if (isFirstCall) {
        flop.value = [...store.board];
        startingPot.value = store.startingPot;
        effectiveStack.value = store.effectiveStack;
        for (let player = 0; player < 2; ++player) {
          const cards = await handler.privateHandCards(player);
          handCards.value[player] = Array.from(
            { length: cards.length / 2 },
            (_, i) => [cards[2 * i + 1], cards[2 * i]]
          );
        }
      }

      const nextActions = (await handler.getActions()).split("/");
      const numActions = nextActions.length;
      const isChance = nextActions[0] === "Chance";

      if (isChance) {
        const isPossibleChance = await handler.isPossibleChance();

        actionList.value.splice(depth, actionList.value.length, {
          type: "River",
          selectedIndex: -1,
          depth: depth + 1,
          actions: Array.from({ length: 52 }, (_, i) => {
            return {
              index: i,
              str: i.toString(),
              isSelected: false,
              isTerminal: !isPossibleChance[i],
            };
          }),
        });

        if (turn.value === -1) {
          actionList.value.slice(-1)[0].type = "Turn";
        }

        return;
      }

      const isTerminalAction = await handler.isTerminalAction();
      actionList.value.splice(depth, actionList.value.length, {
        type: "Player",
        selectedIndex: -1,
        depth: depth + 1,
        actions: Array.from({ length: numActions }, (_, i) => {
          return {
            index: i,
            str: nextActions[i],
            isSelected: false,
            isTerminal: !!isTerminalAction[i],
          };
        }).reverse(),
      });

      const currentPlayer = await handler.currentPlayer();
      const cards = handCards.value[currentPlayer];

      const results = await handler.getResults();
      const weights = results.subarray(0, cards.length);
      const weightsNormalized = results.subarray(
        cards.length,
        2 * cards.length
      );
      const expectedValues = results.subarray(
        2 * cards.length,
        3 * cards.length
      );
      const equity = results.subarray(3 * cards.length, 4 * cards.length);
      const strategy = results.subarray(4 * cards.length);

      if (resultNav.value) {
        const div = resultNav.value;
        div.scrollLeft = div.scrollWidth - div.clientWidth;
      }

      let factor = store.normalizer;
      if (turn.value !== -1) factor *= 45;
      if (river.value !== -1) factor *= 44;

      result.value = cards.map((_, i) => {
        return {
          card1: cards[i][0],
          card2: cards[i][1],
          weight: weights[i],
          weightNormalized: weightsNormalized[i],
          equity: (equity[i] / weightsNormalized[i]) * factor + 0.5,
          expectedValue:
            (expectedValues[i] / weightsNormalized[i]) * factor +
            startingPot.value / 2,
          strategy: Array.from(
            { length: numActions },
            (_, j) => strategy[j * cards.length + i]
          ).reverse(),
        };
      });

      for (let i = 0; i < numActions; ++i) {
        const action = actionList.value.slice(-1)[0].actions[i];
        if (action.isTerminal) continue;
        let invalid = true;
        for (let j = 0; j < cards.length; ++j) {
          if (
            weightsNormalized[j] > 0 &&
            weights[j] * result.value[j].strategy[i] >= 0.05 / 100
          ) {
            invalid = false;
            break;
          }
        }
        action.isTerminal = invalid;
      }

      const weightSumCell = Array.from({ length: 13 * 13 }, () => 0);
      const weightNormalizedSumCell = Array.from({ length: 13 * 13 }, () => 0);
      const countCell = Array.from({ length: 13 * 13 }, () => 0);
      const equitySumCell = Array.from({ length: 13 * 13 }, () => 0);
      const expectedValueSumCell = Array.from({ length: 13 * 13 }, () => 0);
      const strategySumCell = Array.from({ length: 13 * 13 }, () =>
        Array.from({ length: numActions }, () => 0)
      );

      const strategySum = Array.from({ length: numActions }, () => 0);

      for (let i = 0; i < cards.length; ++i) {
        const rank1 = Math.floor(cards[i][0] / 4);
        const suit1 = cards[i][0] % 4;
        const rank2 = Math.floor(cards[i][1] / 4);
        const suit2 = cards[i][1] % 4;

        let row, col;
        if (rank1 === rank2) {
          row = 12 - rank1;
          col = 12 - rank1;
        } else if (suit1 === suit2) {
          row = 12 - rank1;
          col = 12 - rank2;
        } else {
          row = 12 - rank2;
          col = 12 - rank1;
        }

        const idx = row * 13 + col;

        if (weightsNormalized[i] > 0 && weights[i] >= 0.05 / 100) {
          weightSumCell[idx] += weights[i];
          weightNormalizedSumCell[idx] += weightsNormalized[i];
          countCell[idx] += 1;
          equitySumCell[idx] += equity[i] * factor;
          expectedValueSumCell[idx] += expectedValues[i] * factor;
          for (let j = 0; j < numActions; ++j) {
            const s = weightsNormalized[i] * result.value[i].strategy[j];
            strategySumCell[idx][j] += s;
            strategySum[j] += s;
          }
        }
      }

      resultCell.value = Array.from({ length: 13 * 13 }, (_, i) => {
        return {
          weight: weightSumCell[i],
          count: countCell[i],
          equity: equitySumCell[i] / weightNormalizedSumCell[i] + 0.5,
          expectedValue:
            expectedValueSumCell[i] / weightNormalizedSumCell[i] +
            startingPot.value / 2,
          strategy: Array.from(
            { length: numActions },
            (_, j) => strategySumCell[i][j] / weightNormalizedSumCell[i]
          ),
        };
      });

      let player = 0;
      let pot = startingPot.value;
      let stack = [effectiveStack.value, effectiveStack.value];
      let lastBet = 0;

      for (let i = 0; i < actionList.value.length - 1; ++i) {
        const item = actionList.value[i];
        if (item.type !== "Player") {
          player = 0;
          lastBet = 0;
          continue;
        }

        const action = item.actions[item.selectedIndex];

        if (action.str === "Call") {
          pot += stack[player] - stack[1 - player];
          stack[player] = stack[1 - player];
        } else if (action.str.slice(0, 3) === "Bet") {
          const bet = Number(action.str.slice(4));
          pot += bet;
          stack[player] -= bet;
          lastBet = bet;
        } else if (
          action.str.slice(0, 5) === "Raise" ||
          action.str.slice(0, 6) === "All-in"
        ) {
          const bet = Number(action.str.slice(6).trimStart());
          const stackDiff = stack[player] - stack[1 - player];
          pot += bet - lastBet + stackDiff;
          stack[player] -= bet - lastBet + stackDiff;
          lastBet = bet;
        }

        player = 1 - player;
      }

      nodeInformation.value = {
        player,
        pot,
        stack: stack[player],
        toCall: stack[player] - stack[1 - player],
      };
    };

    const moveResult = async (depth: number, index: number) => {
      if (depth === 0) {
        if (actionList.value.length === 1) return;
      } else if (index === -1) {
        return;
      } else {
        const item = actionList.value[depth - 1];
        const selectedIndex = item.actions.findIndex((a) => a.index === index);
        if (
          item.actions[selectedIndex].isTerminal ||
          (item.selectedIndex === selectedIndex &&
            depth === actionList.value.length - 1)
        ) {
          return;
        }
        item.selectedIndex = selectedIndex;
        item.actions.forEach((a) => (a.isSelected = a.index === index));
      }

      const history = new Uint32Array(
        actionList.value
          .slice(0, depth)
          .map((item) => item.actions[item.selectedIndex].index)
      );

      if (!["Hand", "Weight", "EQ", "EV"].includes(sortKey.value.key)) {
        sortKey.value = { key: "Hand", order: "desc" };
      }

      const handler = await GlobalWorker.getHandler();
      await handler.applyHistory(history);
      await updateResult(depth, false);
    };

    const nextActionsStr = computed(() => {
      if (actionList.value.length === 0) return [];
      const lastItem = actionList.value.slice(-1)[0];
      if (lastItem.type === "Player") {
        return lastItem.actions.map((a) => a.str);
      } else {
        return [];
      }
    });

    const actionColor = computed(() => {
      const actions = nextActionsStr.value;
      if (actions.length === 0) return [];

      const ret = [];
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
      for (let i = 0; i < actions.length; ++i) {
        ret[actions[i]] = actionColor.value[i];
      }
      return ret;
    });

    const weightPercent = (row: number, col: number) => {
      if (resultCell.value.length === 0) return "0%";

      const r1 = 13 - row;
      const r2 = 13 - col;

      let denom = 0;
      for (let s1 = 0; s1 < 4; ++s1) {
        for (let s2 = 0; s2 < 4; ++s2) {
          if (
            (row === col && s1 < s2) ||
            (row !== col && row < col === (s1 === s2))
          ) {
            const c1 = r1 * 4 + s1;
            const c2 = r2 * 4 + s2;
            if (
              flop.value.indexOf(c1) === -1 &&
              turn.value !== c1 &&
              river.value !== c1 &&
              flop.value.indexOf(c2) === -1 &&
              turn.value !== c2 &&
              river.value !== c2
            ) {
              denom += 1;
            }
          }
        }
      }

      const idx = (row - 1) * 13 + col - 1;
      return percentStr(resultCell.value[idx].weight / denom);
    };

    const cellItems = (row: number, col: number) => {
      if (resultCell.value.length === 0) return [];
      const idx = (row - 1) * 13 + col - 1;
      const strategy = resultCell.value[idx].strategy;
      return strategy.map((_, i) => {
        const left = strategy.slice(0, i).reduce((a, b) => a + b, 0);
        return {
          key: i,
          class: actionColor.value[i],
          style: {
            width: percentStr(1 - left),
            zIndex: i,
          },
        };
      });
    };

    const cellText = (row: number, col: number) => {
      const r1 = 13 - Math.min(row, col);
      const r2 = 13 - Math.max(row, col);
      return ranks[r1] + ranks[r2] + ["s", "", "o"][Math.sign(row - col) + 1];
    };

    const cellAuxText = (row: number, col: number) => {
      const idx = (row - 1) * 13 + col - 1;
      if (resultCell.value.length === 0 || resultCell.value[idx].count === 0) {
        return "";
      }
      if (sortKey.value.key === "EQ") {
        return (100 * resultCell.value[idx].equity).toFixed(0) + "%";
      } else if (sortKey.value.key === "EV") {
        const ev = resultCell.value[idx].expectedValue;
        return -0.5 <= ev && ev < 0.5
          ? "0"
          : (ev >= 0 ? "+" : "") + ev.toFixed(0);
      } else {
        const i = nextActionsStr.value.indexOf(sortKey.value.key);
        if (i !== -1) {
          return (100 * resultCell.value[idx].strategy[i]).toFixed(0) + "%";
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
      if (idx < 0 || resultCell.value[idx].count === 0) return "All";
      return cellText(row, col);
    });

    const headers = computed(() => {
      return ["Hand", "Weight", "EQ", "EV"].concat(nextActionsStr.value);
    });

    const resultFiltered = computed(() => {
      const { row, col } = hoveredCell.value;
      const idx = (row - 1) * 13 + col - 1;
      if (idx < 0 || resultCell.value[idx].count === 0) {
        return result.value.filter(
          (r) => r.weightNormalized > 0 && r.weight >= 0.05 / 100
        );
      }
      const r1 = 13 - Math.min(row, col);
      const r2 = 13 - Math.max(row, col);
      const isSuited = row < col;
      return result.value.filter((r) => {
        return (
          r.weightNormalized > 0 &&
          r.weight >= 0.05 / 100 &&
          Math.floor(r.card1 / 4) === r1 &&
          Math.floor(r.card2 / 4) === r2 &&
          (r.card1 % 4 === r.card2 % 4) === isSuited
        );
      });
    });

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
          } else if (sortKey.value.key === "EQ") {
            return (
              coef * (round(100 * a.equity) - round(100 * b.equity)) || fallback
            );
          } else if (sortKey.value.key === "EV") {
            return (
              coef * (round(a.expectedValue) - round(b.expectedValue)) ||
              fallback
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
          equity: "-",
          expectedValue: "-",
          strategy: nextActionsStr.value.map(() => "-"),
        };
      }
      let weightSum = 0;
      let combos = 0;
      let equity = 0;
      let expectedValue = 0;
      let strategy = nextActionsStr.value.map(() => 0);
      for (const r of result) {
        weightSum += r.weightNormalized;
        combos += r.weight;
        equity += r.equity * r.weightNormalized;
        expectedValue += r.expectedValue * r.weightNormalized;
        for (let i = 0; i < nextActionsStr.value.length; i++) {
          strategy[i] += r.strategy[i] * r.weightNormalized;
        }
      }
      return {
        combos: combos.toFixed(1),
        equity: percentStr(equity / weightSum),
        expectedValue: trimMinusZero((expectedValue / weightSum).toFixed(1)),
        strategy: strategy.map((x) => percentStr(x / weightSum)),
      };
    });

    return {
      store,
      cardText,
      resultNav,
      flop,
      actionList,
      sortKey,
      sortBy,
      percentStr,
      trimMinusZero,
      moveResult,
      actionColorByStr,
      weightPercent,
      cellItems,
      cellText,
      cellAuxText,
      onMouseOver,
      onMouseLeave,
      hoveredCellText,
      headers,
      resultSorted,
      resultAverage,
      nodeInformation,
    };
  },
});
</script>
