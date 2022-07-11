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
    <div ref="divResultNav" class="flex pl-0.5 overflow-x-auto">
      <div class="flex flex-col items-start">
        <button
          :class="
            'my-[0.1875rem] px-2 py-[0.0625rem] whitespace-nowrap border bg-white rounded-lg shadow select-none ' +
            (actionList.length === 1
              ? 'ring-1 ring-red-600 border-red-600'
              : 'border-black')
          "
          :disabled="actionList.length === 1"
          @click="moveResult(0, 0)"
        >
          <span class="inline-block mr-2 underline">
            {{ ["Flop", "Turn", "River"][board.length - 3] }}
          </span>
          <span
            v-for="item in board.map(cardText)"
            :key="item.rank + item.suit"
            :class="item.colorClass"
          >
            {{ item.rank + item.suit }}
          </span>
        </button>
      </div>

      <div
        v-for="item in actionList"
        :key="item.depth"
        class="flex flex-col ml-2 items-start"
      >
        <template v-if="item.type !== 'Player'">
          <button
            :class="
              'my-[0.1875rem] px-2 py-[0.0625rem] whitespace-nowrap border bg-white rounded-lg shadow select-none ' +
              (item.depth === actionList.length - 1
                ? 'ring-1 ring-red-600 border-red-600'
                : 'border-black')
            "
            :disabled="item.depth >= actionList.length - 1"
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
          </button>
        </template>
        <template v-else>
          <button
            v-for="action in item.actions"
            :key="action.str"
            :class="
              'w-full my-[0.1875rem] px-2 py-[0.0625rem] whitespace-nowrap border rounded-lg shadow text-center select-none ' +
              (!action.isSelected && item.depth < actionList.length
                ? 'opacity-40 '
                : '') +
              (action.isSelected && item.depth === actionList.length - 1
                ? 'ring-1 ring-red-600 border-red-600 '
                : 'border-black ') +
              (item.depth === actionList.length
                ? actionColorByStr[action.str].bg
                : 'bg-white')
            "
            :disabled="
              (action.isSelected && item.depth === actionList.length - 1) ||
              action.isTerminal
            "
            @click="moveResult(item.depth, action.index)"
          >
            {{ action.str }}
          </button>
        </template>
      </div>
    </div>

    <div
      v-if="
        actionList.length > 0 &&
        actionList[actionList.length - 1].type === 'Player'
      "
      class="flex mt-5 items-start"
    >
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
              @mouseenter="onMouseEnter(row, col)"
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

        <div class="mt-4">
          Player: {{ nodeInformation.player === 0 ? "OOP" : "IP" }} / Pot:
          {{ nodeInformation.pot }} / Stack: {{ nodeInformation.stack }}
          {{
            nodeInformation.toCall
              ? " / To Call: " + nodeInformation.toCall
              : ""
          }}
        </div>
      </div>

      <div class="pl-5 pb-1 overflow-x-auto">
        <div
          ref="divResultDetail"
          class="max-h-[30.25rem] border border-gray-500 rounded-md shadow overflow-y-scroll will-change-transform"
          @scroll.passive="onTableScroll"
        >
          <table class="align-middle divide-y divide-gray-300">
            <thead class="sticky top-0 bg-gray-100 shadow z-10">
              <tr style="height: calc(2rem + 1px)">
                <th
                  v-for="text in headers"
                  :key="text"
                  scope="col"
                  :class="
                    'px-1 whitespace-nowrap text-sm font-bold cursor-pointer select-none ' +
                    (text === 'Hand'
                      ? 'min-w-[4.9rem]'
                      : text === 'EV'
                      ? 'min-w-[3.3rem]'
                      : 'min-w-[3.6rem]')
                  "
                  @click="sortBy(text)"
                >
                  <span
                    v-if="text === sortKey.key"
                    class="inline-block text-xs"
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
              <!-- Top empty row -->
              <tr
                v-if="emptyBufferTop > 0"
                :style="{
                  '--num-rows': emptyBufferTop,
                  height: 'calc(var(--num-rows) * (2rem + 1px))',
                }"
              >
                <td :colspan="headers.length"></td>
              </tr>

              <!-- Body -->
              <tr
                v-for="item in resultRendered"
                :key="item.card1 + '-' + item.card2"
                class="text-right text-sm"
                style="height: calc(2rem + 1px)"
              >
                <td class="px-[0.5625rem] text-center">
                  <template
                    v-for="card in [item.card1, item.card2].map(cardText)"
                    :key="card.rank + card.suit"
                  >
                    <span :class="card.colorClass">
                      {{ card.rank + card.suit }}
                    </span>
                  </template>
                </td>
                <td class="px-[0.5625rem]">
                  {{ percentStr(item.weight) }}
                </td>
                <td class="px-[0.5625rem]">
                  {{ percentStr(item.equity) }}
                </td>
                <td class="px-[0.5625rem]">
                  {{ trimMinusZero(item.expectedValue.toFixed(1)) }}
                </td>
                <td
                  v-for="i in item.strategy.length"
                  :key="i"
                  class="relative px-[0.5625rem]"
                >
                  <span v-if="!showActionEv">
                    {{ percentStr(item.strategy[i - 1]) }}
                  </span>
                  <span v-else>
                    {{ trimMinusZero(item.actionEv[i - 1].toFixed(1)) }}
                  </span>
                  <div
                    class="absolute bottom-0 left-2 h-[0.3125rem] bg-gray-400"
                    style="width: calc(100% - 1rem)"
                  ></div>
                  <div
                    :class="
                      'absolute bottom-0 left-2 h-[0.3125rem] ' +
                      actionColor[i - 1].bar
                    "
                    :style="`width: calc((100% - 1rem) * ${
                      item.strategy[i - 1]
                    }`"
                  ></div>
                </td>
              </tr>

              <!-- Bottom empty row -->
              <tr
                v-if="emptyBufferBottom > 0"
                :style="{
                  '--num-rows': emptyBufferBottom,
                  height: 'calc(var(--num-rows) * (2rem + 1px))',
                }"
              >
                <td :colspan="headers.length"></td>
              </tr>
            </tbody>

            <tfoot class="sticky bottom-0 font-bold bg-white shadow">
              <tr class="text-right text-sm" style="height: calc(2rem + 1px)">
                <th scope="col" class="px-[0.5625rem] text-center underline">
                  {{ hoveredCellText }}
                </th>
                <th scope="col" class="px-[0.5625rem]">
                  {{ resultAverage.combos }}
                </th>
                <th scope="col" class="px-[0.5625rem]">
                  {{ resultAverage.equity }}
                </th>
                <th scope="col" class="px-[0.5625rem]">
                  {{ resultAverage.expectedValue }}
                </th>
                <th
                  v-for="i in resultAverage.strategy.length"
                  :key="i"
                  scope="col"
                  class="px-[0.5625rem]"
                >
                  {{ resultAverage.strategy[i - 1] }}
                </th>
              </tr>
            </tfoot>
          </table>
        </div>

        <div class="mt-4">
          <label class="cursor-pointer">
            <input
              v-model="showActionEv"
              type="checkbox"
              class="mr-1 align-middle rounded cursor-pointer"
            />
            Show action EV
          </label>
        </div>
      </div>
    </div>

    <div
      v-else-if="
        actionList.length > 0 &&
        actionList[actionList.length - 1].type !== 'Player'
      "
      class="flex mt-5 items-start"
    >
      <div class="shrink-0">
        <div v-for="suit in 4" :key="suit" class="flex">
          <board-selector-card
            v-for="rank in 13"
            :key="rank"
            class="m-1 disabled:opacity-40"
            :card-id="56 - 4 * rank - suit"
            :disabled="
              actionList[actionList.length - 1].actions[56 - 4 * rank - suit]
                .isTerminal
            "
            @click="moveResult(actionList.length, 56 - 4 * rank - suit)"
          />
        </div>

        <div class="mt-4">
          <label class="cursor-pointer">
            <input
              v-model="showOopStats"
              type="checkbox"
              class="mr-1 align-middle rounded cursor-pointer"
            />
            Show OOP statistics
          </label>
        </div>
      </div>

      <!-- Turn/River report -->
      <div v-if="showOopStats" class="pl-5 pb-1 overflow-x-auto">
        <span class="underline">OOP statistics:</span>
        <div
          class="mt-3 max-h-[24rem] border border-gray-500 rounded-md shadow overflow-y-scroll"
        >
          <table class="align-middle divide-y divide-gray-300">
            <thead class="sticky top-0 bg-gray-100 shadow z-10">
              <tr style="height: calc(2rem + 1px)">
                <th
                  v-for="text in headersChance"
                  :key="text"
                  scope="col"
                  :class="
                    'px-1 whitespace-nowrap text-sm font-bold cursor-pointer select-none ' +
                    (text === 'Card'
                      ? 'min-w-[3.5rem]'
                      : text === 'EV'
                      ? 'min-w-[3.3rem]'
                      : 'min-w-[3.6rem]')
                  "
                  @click="sortByChance(text)"
                >
                  <span
                    v-if="text === sortKeyChance.key"
                    class="inline-block text-xs"
                  >
                    {{ sortKeyChance.order === "asc" ? "▲" : "▼" }}
                  </span>
                  {{
                    text
                      .replace("Card", actionList[actionList.length - 1].type)
                      .replace("Bet", "B")
                      .replace("Raise", "R")
                      .replace("All-in", "A")
                  }}
                </th>
              </tr>
            </thead>

            <tbody class="bg-white divide-y divide-gray-300">
              <tr
                v-for="item in resultChanceSorted"
                :key="item.card"
                class="text-right text-sm"
                style="height: calc(2rem + 1px)"
              >
                <td class="px-[0.5625rem] text-center">
                  <template
                    v-for="card in [cardText(item.card)]"
                    :key="card.rank + card.suit"
                  >
                    <span :class="card.colorClass">
                      {{ card.rank + card.suit }}
                    </span>
                  </template>
                </td>
                <td class="px-[0.5625rem]">
                  {{ percentStr(item.equity) }}
                </td>
                <td class="px-[0.5625rem]">
                  {{ trimMinusZero(item.expectedValue.toFixed(1)) }}
                </td>
                <td
                  v-for="i in item.strategy.length"
                  :key="i"
                  class="px-[0.5625rem]"
                >
                  {{ percentStr(item.strategy[i - 1]) }}
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, nextTick, ref, watch } from "vue";
import { cardText, useStore, useSavedConfigStore } from "../store";
import * as GlobalWorker from "../global-worker";

import BoardSelectorCard from "./BoardSelectorCard.vue";

type Result = {
  card1: number;
  card2: number;
  weight: number;
  weightNormalized: number;
  equity: number;
  expectedValue: number;
  actionEv: number[];
  strategy: number[];
};

type ResultChance = {
  card: number;
  equity: number;
  expectedValue: number;
  strategy: number[];
};

const ranks = ["2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"];

export default defineComponent({
  components: {
    BoardSelectorCard,
  },

  setup() {
    const store = useStore();
    const savedConfig = useSavedConfigStore();

    const divResultNav = ref(null as HTMLDivElement | null);
    const divResultDetail = ref(null as HTMLDivElement | null);

    let isLocked = false;
    const isSolverFinished = ref(false);
    const handCards = ref([new Uint16Array(), new Uint16Array()]);

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

    const result = ref([] as Result[]);

    const resultCell = ref(
      [] as {
        count: number;
        weight: number;
        equity: number;
        expectedValue: number;
        actionEv: number[];
        strategy: number[];
      }[]
    );

    const resultChance = ref([] as ResultChance[]);

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
    const sortKeyChance = ref({ key: "Card", order: "desc" as Order });

    const showActionEv = ref(false);
    const showOopStats = ref(false);

    const chanceNextActions = ref([] as string[]);

    store.$subscribe(async (_, store) => {
      if (store.isSolverFinished !== isSolverFinished.value) {
        if ((isSolverFinished.value = store.isSolverFinished)) {
          isLocked = true;
          await updateResult(0, true);
        } else {
          clearResult();
        }
      }
    });

    const board = computed(() => savedConfig.board);

    const dealtTurn = computed(() => {
      const item = actionList.value.find((item) => item.type === "Turn");
      return item?.selectedIndex ?? -1;
    });

    const dealtRiver = computed(() => {
      const item = actionList.value.find((item) => item.type === "River");
      return item?.selectedIndex ?? -1;
    });

    const percentStr = (num: number) => {
      if (num >= 0.9995) {
        return "100%";
      } else {
        return Math.max(100 * num, 0).toFixed(1) + "%";
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
      const order: Order =
        key === sortKey.value.key && sortKey.value.order === "desc"
          ? "asc"
          : "desc";
      sortKey.value = { key, order };
    };

    const sortByChance = (key: string) => {
      const order: Order =
        key === sortKeyChance.value.key && sortKeyChance.value.order === "desc"
          ? "asc"
          : "desc";
      sortKeyChance.value = { key, order };
    };

    const clearResult = () => {
      handCards.value = [new Uint16Array(), new Uint16Array()];
      actionList.value = [];
      result.value = [];
      resultCell.value = [];
      resultChance.value = [];
    };

    const updateResult = async (depth: number, isFirstCall: boolean) => {
      const handler = await GlobalWorker.getHandler();

      if (isFirstCall) {
        const memory = await GlobalWorker.getMemory();
        const buffer = await memory.buffer;
        for (let player = 0; player < 2; ++player) {
          const cardsBuffer = await handler.privateHandCards(player);
          handCards.value[player] = new Uint16Array(
            buffer,
            cardsBuffer.ptr,
            cardsBuffer.byteLength / 2
          );
        }
      }

      const nextActions = (await handler.availableActions()).split("/");
      const numActions = nextActions.length;
      const isChance = nextActions[0] === "Chance";

      if (isChance) {
        const possibleCards = await handler.possibleCards();
        const next = (await handler.availableActionsAfterChance()).split("/");
        const results = await handler.chanceReport();

        // update actionList
        actionList.value.splice(depth, actionList.value.length, {
          type: "River",
          selectedIndex: -1,
          depth: depth + 1,
          actions: Array.from({ length: 52 }, (_, i) => ({
            index: i,
            str: i.toString(),
            isSelected: false,
            isTerminal: !(possibleCards & (1n << BigInt(i))),
          })),
        });

        if (dealtTurn.value === -1 && board.value.length === 3) {
          actionList.value[actionList.value.length - 1].type = "Turn";
        }

        // update resultChance
        const numActions = next.length;
        chanceNextActions.value = next.reverse();

        const equity = results.subarray(0, 52);
        const expectedValue = results.subarray(52, 52 * 2);
        const strategy = results.subarray(52 * 2);

        resultChance.value = [];
        for (let i = 0; i < 52; ++i) {
          if (!(possibleCards & (1n << BigInt(i)))) continue;
          resultChance.value.push({
            card: i,
            equity: equity[i],
            expectedValue: expectedValue[i],
            strategy: Array.from(
              { length: numActions },
              (_, j) => strategy[i + j * 52]
            ).reverse(),
          });
        }

        updateResultFinal();
        return;
      }

      // obtain the current state
      const isTerminalAction = await handler.isTerminalAction();

      const currentPlayer = await handler.currentPlayer();
      const cards = handCards.value[currentPlayer];

      const results = await handler.getResults();
      const weights = results.subarray(0, cards.length);
      const weightsNormalized = results.subarray(
        cards.length,
        2 * cards.length
      );
      const equity = results.subarray(2 * cards.length, 3 * cards.length);
      const actionEv = results.subarray(
        3 * cards.length,
        (3 + numActions) * cards.length
      );
      const strategy = results.subarray((3 + numActions) * cards.length);

      // update actionList
      actionList.value.splice(depth, actionList.value.length, {
        type: "Player",
        selectedIndex: -1,
        depth: depth + 1,
        actions: Array.from({ length: numActions }, (_, i) => ({
          index: i,
          str: nextActions[i],
          isSelected: false,
          isTerminal: !!(isTerminalAction & (1 << i)),
        })).reverse(),
      });

      // update result
      result.value = Array.from({ length: cards.length }, (_, i) => {
        const a = Array.from(
          { length: numActions },
          (_, j) => actionEv[i + j * cards.length]
        );

        const s = Array.from(
          { length: numActions },
          (_, j) => strategy[i + j * cards.length]
        );

        const ev = a.map((v, j) => v * s[j]).reduce((acc, v) => acc + v, 0);

        return {
          card1: cards[i] >> 8,
          card2: cards[i] & 0xff,
          weight: weights[i],
          weightNormalized: weightsNormalized[i],
          equity: equity[i],
          expectedValue: ev,
          actionEv: a.reverse(),
          strategy: s.reverse(),
        };
      });

      // update terminal action flag
      for (let i = 0; i < numActions; ++i) {
        const action = actionList.value[actionList.value.length - 1].actions[i];
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

      // update resultCell
      const weightSumCell = Array.from({ length: 13 * 13 }, () => 0);
      const weightNormalizedSumCell = Array.from({ length: 13 * 13 }, () => 0);
      const countCell = Array.from({ length: 13 * 13 }, () => 0);
      const equitySumCell = Array.from({ length: 13 * 13 }, () => 0);
      const expectedValueSumCell = Array.from({ length: 13 * 13 }, () => 0);
      const actionEvSumCell = Array.from({ length: 13 * 13 }, () =>
        Array.from({ length: numActions }, () => 0)
      );
      const strategySumCell = Array.from({ length: 13 * 13 }, () =>
        Array.from({ length: numActions }, () => 0)
      );

      for (let i = 0; i < cards.length; ++i) {
        const card1 = cards[i] >> 8;
        const card2 = cards[i] & 0xff;
        const rank1 = Math.floor(card1 / 4);
        const rank2 = Math.floor(card2 / 4);
        const suit1 = card1 % 4;
        const suit2 = card2 % 4;

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
          const w = weightsNormalized[i];
          weightSumCell[idx] += weights[i];
          weightNormalizedSumCell[idx] += w;
          countCell[idx] += 1;
          equitySumCell[idx] += w * equity[i];
          expectedValueSumCell[idx] += w * result.value[i].expectedValue;
          for (let j = 0; j < numActions; ++j) {
            actionEvSumCell[idx][j] += w * result.value[i].actionEv[j];
            strategySumCell[idx][j] += w * result.value[i].strategy[j];
          }
        }
      }

      resultCell.value = Array.from({ length: 13 * 13 }, (_, i) => ({
        weight: weightSumCell[i],
        count: countCell[i],
        equity: equitySumCell[i] / weightNormalizedSumCell[i],
        expectedValue: expectedValueSumCell[i] / weightNormalizedSumCell[i],
        actionEv: Array.from(
          { length: numActions },
          (_, j) => actionEvSumCell[i][j] / weightNormalizedSumCell[i]
        ),
        strategy: Array.from(
          { length: numActions },
          (_, j) => strategySumCell[i][j] / weightNormalizedSumCell[i]
        ),
      }));

      // update nodeInformation
      let player = 0;
      let lastBet = 0;
      const pot = [0, 0];

      for (let i = 0; i < actionList.value.length - 1; ++i) {
        const item = actionList.value[i];
        if (item.type !== "Player") {
          player = 0;
          lastBet = 0;
          continue;
        }

        const action = item.actions[item.selectedIndex];

        if (action.str === "Call") {
          pot[player] = pot[1 - player];
        } else if (action.str.slice(0, 3) === "Bet") {
          const bet = Number(action.str.slice(4));
          pot[player] += bet;
          lastBet = bet;
        } else if (
          action.str.slice(0, 5) === "Raise" ||
          action.str.slice(0, 6) === "All-in"
        ) {
          const bet = Number(action.str.slice(6).trimStart());
          const potDiff = pot[1 - player] - pot[player];
          pot[player] += bet - lastBet + potDiff;
          lastBet = bet;
        }

        player = 1 - player;
      }

      nodeInformation.value = {
        player,
        pot: savedConfig.startingPot + pot[0] + pot[1],
        stack: savedConfig.effectiveStack - pot[player],
        toCall: pot[1 - player] - pot[player],
      };

      await nextTick();
      if (divResultNav.value) {
        const div = divResultNav.value;
        div.scrollLeft = div.scrollWidth - div.clientWidth;
      }

      updateResultFinal();
    };

    const updateResultFinal = () => {
      isLocked = false;

      if (!["Hand", "Weight", "EQ", "EV"].includes(sortKey.value.key)) {
        sortKey.value = { key: "Hand", order: "desc" };
      }

      if (!["Card", "EQ", "EV"].includes(sortKeyChance.value.key)) {
        sortKeyChance.value = { key: "Card", order: "desc" };
      }
    };

    const moveResult = async (depth: number, index: number) => {
      if (
        isLocked ||
        index === -1 ||
        (depth === 0 && actionList.value.length === 1)
      )
        return;

      isLocked = true;

      if (depth > 0) {
        const item = actionList.value[depth - 1];
        const selectedIndex = item.actions.findIndex((a) => a.index === index);
        if (
          item.actions[selectedIndex].isTerminal ||
          (item.selectedIndex === selectedIndex &&
            depth === actionList.value.length - 1)
        ) {
          isLocked = false;
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

      const handler = await GlobalWorker.getHandler();
      await handler.applyHistory(history);
      await updateResult(depth, false);
    };

    const nextActionsStr = computed(() => {
      if (actionList.value.length === 0) return [];
      const lastItem = actionList.value[actionList.value.length - 1];
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
      const subtract = actions[actions.length - 1] === "Check" ? 1 : 2;

      for (let i = 0; i < actions.length; ++i) {
        let color;
        if (actions[i] === "Fold") {
          color = { bg: "bg-sky-300", bar: "bg-sky-400" };
        } else if (actions[i] === "Check" || actions[i] === "Call") {
          color = { bg: "bg-green-300", bar: "bg-green-400" };
        } else {
          color = [
            { bg: "bg-amber-300", bar: "bg-amber-400" },
            { bg: "bg-pink-300", bar: "bg-pink-400" },
            { bg: "bg-violet-300", bar: "bg-violet-400" },
          ][(actions.length - subtract - i - 1) % 3];
        }
        ret.push(color);
      }

      return ret;
    });

    const actionColorByStr = computed(() => {
      const ret = {} as { [key: string]: { bg: string; bar: string } };
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
              savedConfig.board.indexOf(c1) === -1 &&
              dealtTurn.value !== c1 &&
              dealtRiver.value !== c1 &&
              savedConfig.board.indexOf(c2) === -1 &&
              dealtTurn.value !== c2 &&
              dealtRiver.value !== c2
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
          class: actionColor.value[i].bg,
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

    const onMouseEnter = (row: number, col: number) => {
      hoveredCell.value = { row, col };
    };

    const onMouseLeave = () => {
      hoveredCell.value = { row: 0, col: 0 };
    };

    const hoveredCellText = computed(() => {
      const { row, col } = hoveredCell.value;
      const idx = (row - 1) * 13 + col - 1;
      if (
        idx < 0 ||
        resultCell.value.length === 0 ||
        resultCell.value[idx].count === 0
      ) {
        return "All";
      } else {
        return cellText(row, col);
      }
    });

    const headers = computed(() => {
      return ["Hand", "Weight", "EQ", "EV"].concat(nextActionsStr.value);
    });

    const headersChance = computed(() => {
      return ["Card", "EQ", "EV"].concat(chanceNextActions.value);
    });

    const resultFiltered = ref([] as Result[]);
    const resultSorted = ref([] as Result[]);
    const resultAverage = ref({
      combos: "-",
      equity: "-",
      expectedValue: "-",
      strategy: [] as string[],
    });

    const resultRendered = ref([] as Result[]);
    const bufferUnit = 13;
    const emptyBufferTop = ref(-2 * bufferUnit);
    const emptyBufferBottom = computed(
      () => resultFiltered.value.length - emptyBufferTop.value - 5 * bufferUnit
    );

    // update resultFiltered
    watch([hoveredCell, result, resultCell], (newValues, prevValues) => {
      const [hoveredCell, result, resultCell] = newValues;
      const [prevHoveredCell, prevResult, prevResultCell] = prevValues;

      if (resultCell.length === 0) return;

      const { row, col } = hoveredCell;
      const idx = (row - 1) * 13 + col - 1;
      const showAll = idx < 0 || resultCell[idx].count === 0;

      // skip recalculation if possible
      if (result === prevResult && resultCell === prevResultCell) {
        const { row: prevRow, col: prevCol } = prevHoveredCell;
        const prevIdx = (prevRow - 1) * 13 + prevCol - 1;
        const prevShowAll = prevIdx < 0 || resultCell[prevIdx].count === 0;
        if (showAll && prevShowAll) return;
      }

      if (showAll) {
        resultFiltered.value = result.filter(
          (r) => r.weightNormalized > 0 && r.weight >= 0.05 / 100
        );
      } else {
        const r1 = 13 - Math.min(row, col);
        const r2 = 13 - Math.max(row, col);
        const isSuited = row < col;
        resultFiltered.value = result.filter(
          (r) =>
            Math.floor(r.card1 / 4) === r1 &&
            Math.floor(r.card2 / 4) === r2 &&
            (r.card1 % 4 === r.card2 % 4) === isSuited &&
            r.weightNormalized > 0 &&
            r.weight >= 0.05 / 100
        );
      }
    });

    // update resultSorted
    watch([resultFiltered, sortKey], () => {
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

      const coef = sortKey.value.order === "asc" ? 1 : -1;
      const round = (x: number) => Math.round(10 * x);

      resultSorted.value = [...resultFiltered.value].sort((a, b) => {
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

      // scroll to top
      emptyBufferTop.value = -2 * bufferUnit;
      resultRendered.value = resultSorted.value.slice(0, 3 * bufferUnit);
      if (divResultDetail.value) {
        divResultDetail.value.scrollTop = 0;
      }
    });

    // update resultAverage
    watch(resultFiltered, () => {
      if (resultFiltered.value.length === 0) {
        resultAverage.value = {
          combos: "-",
          equity: "-",
          expectedValue: "-",
          strategy: nextActionsStr.value.map(() => "-"),
        };
        return;
      }
      let weightSum = 0;
      let combos = 0;
      let equity = 0;
      let expectedValue = 0;
      const strategy = nextActionsStr.value.map(() => 0);
      for (const r of resultFiltered.value) {
        weightSum += r.weightNormalized;
        combos += r.weight;
        equity += r.equity * r.weightNormalized;
        expectedValue += r.expectedValue * r.weightNormalized;
        for (let i = 0; i < nextActionsStr.value.length; i++) {
          strategy[i] += r.strategy[i] * r.weightNormalized;
        }
      }
      resultAverage.value = {
        combos: combos.toFixed(1),
        equity: percentStr(equity / weightSum),
        expectedValue: trimMinusZero((expectedValue / weightSum).toFixed(1)),
        strategy: strategy.map((x) => percentStr(x / weightSum)),
      };
    });

    const rem = Number(
      window
        .getComputedStyle(document.documentElement)
        .getPropertyValue("font-size")
        .replace("px", "")
    );

    const rowHeight = 2 * rem + 1;

    let ticking = false;

    // update resultRendered
    const onTableScroll = () => {
      if (ticking) return;
      ticking = true;

      requestAnimationFrame(() => {
        ticking = false;
        if (!divResultDetail.value) return;
        const { scrollTop } = divResultDetail.value;
        const topIndex = Math.max(scrollTop / rowHeight, 0);
        let update = false;
        if (topIndex < emptyBufferTop.value + bufferUnit) {
          update = true;
          emptyBufferTop.value =
            (Math.floor(topIndex / bufferUnit) - 1) * bufferUnit;
        } else if (topIndex > emptyBufferTop.value + 3 * bufferUnit) {
          update = true;
          emptyBufferTop.value =
            (Math.floor(topIndex / bufferUnit) - 2) * bufferUnit;
        }
        if (update) {
          resultRendered.value = resultSorted.value.slice(
            Math.max(emptyBufferTop.value, 0),
            emptyBufferTop.value + 5 * bufferUnit
          );
        }
      });
    };

    const resultChanceSorted = ref([] as ResultChance[]);

    // update resultChanceSorted
    watch([resultChance, sortKeyChance], () => {
      const rankComparator = (a: number, b: number, order: Order) => {
        const coef = order === "asc" ? 1 : -1;
        const ar = Math.floor(a / 4);
        const as = a % 4;
        const br = Math.floor(b / 4);
        const bs = b % 4;
        return coef * (ar - br) || bs - as;
      };

      const coef = sortKeyChance.value.order === "asc" ? 1 : -1;
      const round = (x: number) => Math.round(10 * x);

      resultChanceSorted.value = [...resultChance.value].sort((a, b) => {
        if (sortKeyChance.value.key === "Card") {
          return rankComparator(a.card, b.card, sortKeyChance.value.order);
        } else {
          const fallback = rankComparator(a.card, b.card, "desc");
          if (sortKeyChance.value.key === "EQ") {
            const av = round(100 * a.equity);
            const bv = round(100 * b.equity);
            return coef * (av - bv) || fallback;
          } else if (sortKeyChance.value.key === "EV") {
            const av = round(a.expectedValue);
            const bv = round(b.expectedValue);
            return coef * (av - bv) || fallback;
          } else {
            const idx = chanceNextActions.value.indexOf(
              sortKeyChance.value.key
            );
            const av = round(100 * a.strategy[idx]);
            const bv = round(100 * b.strategy[idx]);
            return coef * (av - bv) || fallback;
          }
        }
      });
    });

    return {
      cardText,
      store,
      divResultNav,
      divResultDetail,
      actionList,
      nodeInformation,
      sortKey,
      sortKeyChance,
      showActionEv,
      showOopStats,
      board,
      percentStr,
      trimMinusZero,
      sortBy,
      sortByChance,
      moveResult,
      actionColor,
      actionColorByStr,
      weightPercent,
      cellItems,
      cellText,
      cellAuxText,
      onMouseEnter,
      onMouseLeave,
      hoveredCellText,
      headers,
      headersChance,
      resultAverage,
      resultRendered,
      emptyBufferTop,
      emptyBufferBottom,
      onTableScroll,
      resultChanceSorted,
    };
  },
});
</script>
