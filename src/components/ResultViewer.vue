<template>
  <div
    v-if="!store.isSolverFinished"
    class="w-full max-w-screen-xl mx-auto py-6"
  >
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

  <div v-else class="flex flex-col h-full">
    <div class="flex gap-0.5 p-0.5 overflow-x-auto whitespace-nowrap snug">
      <div
        v-for="spot in spots"
        :key="spot.index"
        :class="
          'flex flex-col h-[9.75rem] p-0.5 justify-start cursor-pointer ' +
          'rounded-md shadow-md border-2  transition group ' +
          (spot.type === 'chance'
            ? 'hover:border-red-600 '
            : 'hover:border-blue-600 ') +
          (spot.index === selectedChance
            ? 'border-red-600'
            : spot.index === selectedSpot
            ? 'border-blue-600'
            : 'border-gray-500')
        "
        @click="selectSpot(spot.index, false)"
      >
        <!-- Root or Chance -->
        <template v-if="spot.type === 'root' || spot.type === 'chance'">
          <div
            :class="
              'px-1.5 pt-1 pb-0.5 font-bold group-hover:opacity-100 ' +
              (spot.index === selectedChance ? '' : 'opacity-70')
            "
          >
            {{ spot.player.toUpperCase() }}
          </div>
          <div
            class="flex flex-col flex-grow px-3 items-center justify-evenly font-bold"
          >
            <div>
              <span
                v-for="card of spotCards(spot)"
                :key="card.rank + card.suit"
                :class="card.colorClass"
              >
                {{ card.rank + card.suit }}
              </span>
            </div>
            <div
              :class="
                'group-hover:opacity-100 ' +
                (spot.index === selectedChance ? '' : 'opacity-70')
              "
            >
              <div>Pot {{ spot.pot }}</div>
              <div>Stack {{ spot.stack }}</div>
            </div>
          </div>
        </template>

        <!-- Player -->
        <template v-if="spot.type === 'player'">
          <div
            :class="
              'px-1.5 py-1 font-bold group-hover:opacity-100 ' +
              (spot.index === selectedSpot ? '' : 'opacity-70')
            "
          >
            {{ spot.player.toUpperCase() }}
          </div>
          <div class="flex-grow overflow-y-auto">
            <button
              v-for="action of spot.actions"
              :key="action.index"
              :class="
                'flex w-full px-1.5 rounded-md transition-colors hover:bg-blue-100 ' +
                (action.isSelected ? 'bg-blue-100 ' : '')
              "
              @click.prevent="play(spot.index, action.index)"
            >
              <span class="inline-block relative w-4 mr-0.5">
                <span
                  v-if="spot.index === Math.max(selectedSpot, 1)"
                  class="absolute top-[0.3125rem] left-0 w-3 h-3 rounded-sm"
                  :style="{ backgroundColor: action.color }"
                ></span>
                <span v-if="action.isSelected">
                  <CheckIcon
                    class="absolute top-[0.1875rem] -left-0.5 w-4 h-4"
                  />
                </span>
              </span>
              <span
                :class="
                  'pr-0.5 font-bold group-hover:opacity-100 ' +
                  (action.isSelected || spot.index === selectedSpot
                    ? ''
                    : 'opacity-70')
                "
              >
                {{ action.name + (action.amount ? ` ${action.amount}` : "") }}
              </span>
              <span
                v-if="spot.index === Math.max(selectedSpot, 1)"
                :class="
                  'ml-auto pl-1.5 group-hover:opacity-100 ' +
                  (action.isSelected || spot.index === selectedSpot
                    ? ''
                    : 'opacity-70')
                "
              >
                [xx.x%]
              </span>
            </button>
          </div>
        </template>

        <!-- Terminal -->
        <template v-else-if="spot.type === 'terminal'">
          <div
            :class="
              'px-1.5 pt-1 pb-0.5 font-bold group-hover:opacity-100 ' +
              (spot.index === selectedSpot ? '' : 'opacity-70')
            "
          >
            {{ spot.player.toUpperCase() }}
          </div>
          <div
            :class="
              'flex flex-col flex-grow items-center justify-evenly font-bold ' +
              (spot.index === selectedSpot ? '' : 'opacity-70')
            "
          >
            <div
              v-if="spot.equityOop === 0 || spot.equityOop === 1"
              class="px-3"
            >
              {{ ["IP", "OOP"][spot.equityOop] }} Wins
            </div>
            <div v-else-if="spot.equityOop !== -1" class="px-1.5">
              <div class="mb-0.5">Equity</div>
              <div class="flex w-full px-1.5">
                <span>OOP</span>
                <span class="ml-auto pl-2">
                  {{ (100 * spot.equityOop).toFixed(1) }}%
                </span>
              </div>
              <div class="flex w-full px-1.5">
                <span>IP</span>
                <span class="ml-auto pl-2">
                  {{ (100 * (1 - spot.equityOop)).toFixed(1) }}%
                </span>
              </div>
            </div>
            <div class="px-3">Pot {{ spot.pot }}</div>
          </div>
        </template>
      </div>
    </div>

    <button @click="reset">Reset</button>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import { useStore, useSavedConfigStore } from "../store";
import { cardText } from "../utils";
import * as GlobalWorker from "../global-worker";

import { CheckIcon } from "@heroicons/vue/20/solid";

type SpotRoot = {
  type: "root";
  index: 0;
  player: "flop" | "turn" | "river";
  selectedIndex: -1;
  board: number[];
  pot: number;
  stack: number;
};

type SpotChance = {
  type: "chance";
  index: number;
  player: "turn" | "river";
  selectedIndex: number;
  cards: {
    card: number;
    isSelected: boolean;
    isDead: boolean;
  }[];
  pot: number;
  stack: number;
};

type SpotPlayer = {
  type: "player";
  index: number;
  player: "oop" | "ip";
  selectedIndex: number;
  actions: {
    index: number;
    name: string;
    amount: number;
    isSelected: boolean;
    color: string;
  }[];
};

type SpotTerminal = {
  type: "terminal";
  index: number;
  player: "end";
  selectedIndex: -1;
  equityOop: number;
  pot: number;
};

type Spot = SpotRoot | SpotChance | SpotPlayer | SpotTerminal;

const foldColor = "#3b82f6"; // blue-500
const checkColor = "#10b981"; // emerald-500
const callColor = "#10b981"; // emerald-500
const betColorGradient = [
  "#f59e0b", // amber-500
  "#f97316", // orange-500
  "#ef4444", // red-500
  "#f43f5e", // rose-500
  "#ec4899", // pink-500
  "#d946ef", // fuchsia-500
  "#a855f7", // purple-500
];

const actionColor = (
  name: string,
  index: number,
  numActions: number,
  numBetActions: number
) => {
  if (name === "Fold") return foldColor;
  if (name === "Check") return checkColor;
  if (name === "Call") return callColor;

  if (numBetActions === 1) return betColorGradient[0];
  if (index === numActions - 1) {
    return numBetActions === 2
      ? betColorGradient[(betColorGradient.length - 1) / 2]
      : betColorGradient[betColorGradient.length - 1];
  }

  const betIndex = index - (numActions - numBetActions);
  const colorRate = betIndex / (numBetActions - 1);

  const gradientRate = colorRate * (betColorGradient.length - 1);
  const gradientIndex = Math.floor(gradientRate);
  const gradientRemainder = gradientRate - gradientIndex;

  const color1 = betColorGradient[gradientIndex];
  const color2 = betColorGradient[gradientIndex + 1];

  let colorString = "#";
  for (let i = 1; i < 7; i += 2) {
    const color1Int = parseInt(color1.slice(i, i + 2), 16);
    const color2Int = parseInt(color2.slice(i, i + 2), 16);
    const colorInt = Math.round(
      color1Int * (1 - gradientRemainder) + color2Int * gradientRemainder
    );
    const colorHex = colorInt.toString(16).padStart(2, "0");
    colorString += colorHex;
  }

  return colorString;
};

export default defineComponent({
  components: {
    CheckIcon,
  },

  setup() {
    type Handler = Awaited<ReturnType<typeof GlobalWorker.getHandler>>;
    let handler: Handler | null = null;

    const store = useStore();
    const config = useSavedConfigStore();

    const spots = ref<Spot[]>([]);
    const selectedSpot = ref(-1);
    const selectedChance = ref(-1);
    const pendingChance = ref(-1);

    let cards = [new Uint16Array(), new Uint16Array()];

    const isSolverFinished = ref(false);
    store.$subscribe(async (_, store) => {
      if (isSolverFinished.value !== store.isSolverFinished) {
        if ((isSolverFinished.value = store.isSolverFinished)) {
          await initResults();
          await selectSpot(1, true);
        } else {
          clearResults();
        }
      }
    });

    const initResults = async () => {
      handler = await GlobalWorker.getHandler();
      if (!handler) return;

      const l = config.board.length;
      const spot: SpotRoot = {
        type: "root",
        index: 0,
        player: l === 3 ? "flop" : l === 4 ? "turn" : "river",
        selectedIndex: -1,
        board: config.board,
        pot: config.startingPot,
        stack: config.effectiveStack,
      };
      spots.value = [spot];

      const memory = await GlobalWorker.getMemory();
      const buffer = await memory.buffer;
      for (let player = 0; player < 2; ++player) {
        const cardsBuffer = await handler.privateCards(player);
        cards[player] = new Uint16Array(
          buffer,
          cardsBuffer.ptr,
          cardsBuffer.byteLength / 2
        );
      }
    };

    let isLocked = false;
    const selectSpot = async (spotIndex: number, needSplice: boolean) => {
      if (!handler || isLocked) return;
      if (spotIndex === selectedSpot.value && !needSplice) return;
      if (spotIndex === selectedChance.value && !needSplice) return;

      if (spotIndex === 0) {
        await selectSpot(1, true);
        return;
      }

      isLocked = true;

      const history = new Uint32Array(
        spots.value.slice(1, spotIndex).map((spot) => spot.selectedIndex)
      );

      await handler.applyHistory(history);
      const actions = (await handler.actions()).split("/");

      const isChance = !needSplice && spots.value[spotIndex]?.type === "chance";
      if (actions[0] === "Terminal") {
        await updateResultsTerminal(spotIndex, needSplice);
      } else if (actions[0] === "Chance" || isChance) {
        await updateResultsChance(spotIndex, needSplice);
      } else {
        await updateResultsPlayer(spotIndex, needSplice, actions);
      }

      isLocked = false;
    };

    const updateResultsTerminal = async (
      spotIndex: number,
      needSplice: boolean
    ) => {
      if (!handler) return;

      // handler operations

      const findUnselectedChance = (startIndex: number) =>
        spots.value
          .slice(startIndex, spotIndex)
          .find(
            (spot: Spot) => spot.type === "chance" && spot.selectedIndex === -1
          ) as SpotChance | undefined;

      const skippedChance = findUnselectedChance(3);
      const skippedRiver = skippedChance
        ? findUnselectedChance(skippedChance.index + 3)
        : undefined;

      if (needSplice) {
        const lastChance = spots.value
          .slice(3)
          .reverse()
          .find((spot) => spot.type === "chance") as SpotChance | undefined;

        const lastSpot = spots.value[spotIndex - 1] as SpotPlayer;
        const lastAction = lastSpot.actions[lastSpot.selectedIndex];

        let equityOop;
        if (lastAction.name === "Fold") {
          equityOop = lastSpot.player === "oop" ? 0 : 1;
        } else if (skippedChance) {
          equityOop = -1;
        } else {
          equityOop = await handler.equity(0);
        }

        let pot = lastChance?.pot ?? config.startingPot;

        if (lastAction.name === "Fold") {
          for (const i of [spotIndex - 2, spotIndex - 3]) {
            const spot = spots.value[i];
            if (spot.type === "player") {
              const action = spot.actions[spot.selectedIndex];
              pot += action.amount;
            }
          }
        } else if (lastAction.name === "Call") {
          const lastBetSpot = spots.value[spotIndex - 2] as SpotPlayer;
          pot += 2 * lastBetSpot.actions[lastBetSpot.selectedIndex].amount;
        }

        spots.value.splice(spotIndex, spots.value.length, {
          type: "terminal",
          index: spotIndex,
          player: "end",
          selectedIndex: -1,
          equityOop,
          pot,
        });
      }

      if (spotIndex <= pendingChance.value) {
        pendingChance.value = -1;
      }
      if (spotIndex <= selectedChance.value) {
        selectedChance.value = pendingChance.value;
        pendingChance.value = -1;
      }
      if (spotIndex <= selectedChance.value) {
        selectedChance.value = -1;
      }
      if (skippedChance && selectedChance.value === -1) {
        selectedChance.value = skippedChance.index;
      }
      if (skippedRiver && pendingChance.value === -1) {
        pendingChance.value = skippedRiver.index;
      }

      selectedSpot.value = spotIndex;

      // display operations
    };

    const updateResultsChance = async (
      spotIndex: number,
      needSplice: boolean
    ) => {
      if (!handler) return;

      // handler operations

      if (needSplice) {
        type SpotTurn = SpotRoot | SpotChance;
        const lastBetSpot = spots.value[spotIndex - 2] as SpotPlayer;
        const turnSpot = spots.value
          .slice(0, spotIndex)
          .find((spot) => spot.player === "turn") as SpotTurn | undefined;

        const amount = lastBetSpot.actions[lastBetSpot.selectedIndex].amount;
        const pot = (turnSpot?.pot ?? config.startingPot) + 2 * amount;
        const stack = (turnSpot?.stack ?? config.effectiveStack) - amount;

        const possibleCards = await handler.possibleCards();
        const nextActions = (await handler.actionsAfterChance()).split("/");

        let numBetActions = nextActions.length;
        while (
          numBetActions > 0 &&
          nextActions[nextActions.length - numBetActions].split(":")[1] === "0"
        ) {
          --numBetActions;
        }

        spots.value.splice(
          spotIndex,
          spots.value.length,
          {
            type: "chance",
            index: spotIndex,
            player: turnSpot ? "river" : "turn",
            selectedIndex: -1,
            cards: Array.from({ length: 52 }, (_, i) => ({
              card: i,
              isSelected: false,
              isDead: !(possibleCards & (1n << BigInt(i))),
            })),
            pot,
            stack,
          },
          {
            type: "player",
            index: spotIndex + 1,
            player: "oop",
            selectedIndex: -1,
            actions: nextActions.map((action, i) => {
              const [name, amount] = action.split(":");
              return {
                index: i,
                name,
                amount: Number(amount),
                isSelected: false,
                color: actionColor(name, i, nextActions.length, numBetActions),
              };
            }),
          }
        );

        selectedSpot.value = spotIndex + 1;
      }

      if (
        needSplice &&
        selectedChance.value !== -1 &&
        selectedChance.value < spotIndex
      ) {
        pendingChance.value = spotIndex;
      } else if (spotIndex !== selectedChance.value) {
        pendingChance.value = selectedChance.value;
        selectedChance.value = spotIndex;
      }

      // display operations
    };

    const updateResultsPlayer = async (
      spotIndex: number,
      needSplice: boolean,
      actions: string[]
    ) => {
      if (!handler) return;

      const findUnselectedChance = (startIndex: number) =>
        spots.value
          .slice(startIndex, spotIndex)
          .find(
            (spot: Spot) => spot.type === "chance" && spot.selectedIndex === -1
          ) as SpotChance | undefined;

      const skippedChance = findUnselectedChance(3);
      const skippedRiver = skippedChance
        ? findUnselectedChance(skippedChance.index + 3)
        : undefined;

      if (needSplice) {
        const lastSpot = spots.value[spotIndex - 1];
        const player = lastSpot.player === "oop" ? "ip" : "oop";

        let numBetActions = actions.length;
        if (actions[0].split(":")[1] === "0") --numBetActions;
        if (actions[1]?.split(":")[1] === "0") --numBetActions;

        spots.value.splice(spotIndex, spots.value.length, {
          type: "player",
          index: spotIndex,
          player,
          selectedIndex: -1,
          actions: actions.map((action, i) => {
            const [name, amount] = action.split(":");
            return {
              index: i,
              name,
              amount: Number(amount),
              isSelected: false,
              color: actionColor(name, i, actions.length, numBetActions),
            };
          }),
        });
      }

      if (spotIndex <= pendingChance.value) {
        pendingChance.value = -1;
      }
      if (spotIndex <= selectedChance.value) {
        selectedChance.value = pendingChance.value;
        pendingChance.value = -1;
      }
      if (spotIndex <= selectedChance.value) {
        selectedChance.value = -1;
      }
      if (skippedChance && selectedChance.value === -1) {
        selectedChance.value = skippedChance.index;
      }
      if (skippedRiver && pendingChance.value === -1) {
        pendingChance.value = skippedRiver.index;
      }

      selectedSpot.value = spotIndex;

      // display operations
    };

    const play = async (spotIndex: number, actionIndex: number) => {
      const spot = spots.value[spotIndex] as SpotPlayer;

      if (spot.selectedIndex !== -1) {
        spot.actions[spot.selectedIndex].isSelected = false;
      }
      spot.actions[actionIndex].isSelected = true;
      spot.selectedIndex = actionIndex;

      await selectSpot(spotIndex + 1, true);
    };

    const deal = async (card: number) => {
      const spot = spots.value[selectedChance.value] as SpotChance;
      if (spot.selectedIndex === card) return;

      if (spot.selectedIndex !== -1) {
        spot.cards[spot.selectedIndex].isSelected = false;
      }
      spot.cards[card].isSelected = true;
      spot.selectedIndex = card;

      selectedChance.value = pendingChance.value;
      pendingChance.value = -1;

      await selectSpot(selectedSpot.value, false);
    };

    const clearResults = () => {
      handler = null;
      spots.value = [];
      selectedSpot.value = -1;
      selectedChance.value = -1;
      pendingChance.value = -1;
      cards = [new Uint16Array(), new Uint16Array()];
    };

    const spotCards = (spot: SpotRoot | SpotChance) => {
      if (spot.type === "root") {
        return spot.board.map((card) => cardText(card));
      } else if (spot.selectedIndex === -1) {
        return [{ rank: "?", suit: "", colorClass: "text-black" }];
      } else {
        return [cardText(spot.selectedIndex)];
      }
    };

    const reset = async () => {
      await initResults();
      await selectSpot(1, true);
    };

    return {
      store,
      config,
      spots,
      selectedSpot,
      selectedChance,
      selectSpot,
      play,
      deal,
      spotCards,
      reset,
    };
  },
});
</script>

<style scoped>
.snug * {
  @apply leading-tight;
}
</style>
