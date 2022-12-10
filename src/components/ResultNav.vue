<template>
  <div class="flex h-40 gap-0.5 p-0.5 overflow-x-auto whitespace-nowrap snug">
    <div
      v-for="spot in spots"
      :key="spot.index"
      :class="
        'flex flex-col h-full p-0.5 justify-start cursor-pointer ' +
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
                <CheckIcon class="absolute top-[0.1875rem] -left-0.5 w-4 h-4" />
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
          <div v-if="spot.equityOop === 0 || spot.equityOop === 1" class="px-3">
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
</template>

<script lang="ts">
import { defineComponent, toRefs, ref, watch } from "vue";
import { useSavedConfigStore } from "../store";
import { cardText, average } from "../utils";
import { handler, memory, ReadonlyBuffer } from "../global-worker";
import {
  Results,
  Spot,
  SpotRoot,
  SpotChance,
  SpotPlayer,
} from "../result-types";

import { CheckIcon } from "@heroicons/vue/20/solid";

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

  props: {
    isHandlerUpdated: {
      type: Boolean,
      required: true,
    },
    isLocked: {
      type: Boolean,
      required: true,
    },
    cardsLength: {
      type: Array as () => number[],
      required: true,
    },
  },

  emits: {
    "update:is-handler-updated": (_value: boolean) => true,
    "update:is-locked": (_value: boolean) => true,
    "trigger-update": () => true,
  },

  setup(props, context) {
    const config = useSavedConfigStore();

    const spots = ref<Spot[]>([]);
    const selectedSpot = ref(-1);
    const selectedChance = ref(-1);
    const pendingChance = ref(-1);

    const { isHandlerUpdated } = toRefs(props);

    let totalBetAmount = new Float32Array(2);
    let results: Results | null = null;

    watch(isHandlerUpdated, async () => {
      if (!isHandlerUpdated.value) return;

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

      await selectSpot(1, true);
      context.emit("update:is-handler-updated", false);
    });

    const selectSpot = async (spotIndex: number, needSplice: boolean) => {
      if (!handler || props.isLocked) return;
      if (spotIndex === selectedSpot.value && !needSplice) return;
      if (spotIndex === selectedChance.value && !needSplice) return;

      if (spotIndex === 0) {
        await selectSpot(1, true);
        return;
      }

      context.emit("update:is-locked", true);

      const history = new Uint32Array(
        spots.value.slice(1, spotIndex).map((spot) => spot.selectedIndex)
      );

      await handler.applyHistory(history);

      totalBetAmount = await handler.totalBetAmount();
      const currentPlayer = await handler.currentPlayer();
      const actions = (await handler.actions()).split("/");
      const numActions = ["terminal", "chance"].includes(currentPlayer)
        ? 0
        : actions.length;

      const buffer = await handler.getResults();
      results = await getResults(buffer, currentPlayer, numActions);

      const isChance = !needSplice && spots.value[spotIndex]?.type === "chance";
      if (currentPlayer === "terminal") {
        await updateResultsTerminal(spotIndex, needSplice);
      } else if (currentPlayer === "chance" || isChance) {
        await updateResultsChance(spotIndex, needSplice);
      } else {
        await updateResultsPlayer(spotIndex, needSplice, actions);
      }

      context.emit("trigger-update");
    };

    const getResults = async (
      resultsBuffer: ReadonlyBuffer,
      currentPlayer: string,
      numActions: number
    ): Promise<Results> => {
      if (!memory) throw new Error("something wrong");
      const buffer = await memory.buffer;

      const ptr = resultsBuffer.ptr;
      const lengthOop = props.cardsLength[0];
      const lengthIp = props.cardsLength[1];

      let offset = 0;

      const weightOop = new Float32Array(buffer, ptr + offset, lengthOop);
      offset += 4 * lengthOop;
      const weightIp = new Float32Array(buffer, ptr + offset, lengthIp);
      offset += 4 * lengthIp;

      const normalizedOop = new Float32Array(buffer, ptr + offset, lengthOop);
      offset += 4 * lengthOop;
      const normalizedIp = new Float32Array(buffer, ptr + offset, lengthIp);
      offset += 4 * lengthIp;

      const equityOop = new Float32Array(buffer, ptr + offset, lengthOop);
      offset += 4 * lengthOop;
      const equityIp = new Float32Array(buffer, ptr + offset, lengthIp);
      offset += 4 * lengthIp;

      const evOop = new Float32Array(buffer, ptr + offset, lengthOop);
      offset += 4 * lengthOop;
      const evIp = new Float32Array(buffer, ptr + offset, lengthIp);
      offset += 4 * lengthIp;

      const eqrOop = new Float32Array(buffer, ptr + offset, lengthOop);
      offset += 4 * lengthOop;
      const eqrIp = new Float32Array(buffer, ptr + offset, lengthIp);
      offset += 4 * lengthIp;

      let strategy = new Float32Array();
      let evDetail = new Float32Array();
      if (["oop", "ip"].includes(currentPlayer)) {
        const length = currentPlayer === "oop" ? lengthOop : lengthIp;
        strategy = new Float32Array(buffer, ptr + offset, numActions * length);
        offset += 4 * numActions * length;
        evDetail = new Float32Array(buffer, ptr + offset, numActions * length);
        offset += 4 * numActions * length;
      }

      console.log(4 * offset, resultsBuffer.byteLength);

      return {
        currentPlayer: currentPlayer as Results["currentPlayer"],
        numActions,
        isEmpty: [
          weightOop.every((v) => v < 0.0005), // 0.05%
          weightIp.every((v) => v < 0.0005),
        ],
        weights: [weightOop, weightIp],
        normalized: [normalizedOop, normalizedIp],
        equity: [equityOop, equityIp],
        ev: [evOop, evIp],
        eqr: [eqrOop, eqrIp],
        strategy,
        evDetail,
      };
    };

    const updateResultsTerminal = async (
      spotIndex: number,
      needSplice: boolean
    ) => {
      if (!handler || !results) return;

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
        const prevSpot = spots.value[spotIndex - 1] as SpotPlayer;
        const prevAction = prevSpot.actions[prevSpot.selectedIndex];

        let equityOop;
        if (prevAction.name === "Fold") {
          equityOop = prevSpot.player === "oop" ? 0 : 1;
        } else if (skippedChance || results.isEmpty[0] || results.isEmpty[1]) {
          equityOop = -1;
        } else {
          equityOop = average(results.equity[0], results.normalized[0]);
        }

        spots.value.splice(spotIndex, spots.value.length, {
          type: "terminal",
          index: spotIndex,
          player: "end",
          selectedIndex: -1,
          prevPlayer: prevSpot.player,
          equityOop,
          pot: config.startingPot + totalBetAmount[0] + totalBetAmount[1],
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
        const prevSpot = spots.value[spotIndex - 1] as SpotPlayer;
        const turnSpot = spots.value
          .slice(0, spotIndex)
          .find((spot) => spot.player === "turn") as SpotTurn | undefined;

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
            prevPlayer: prevSpot.player,
            cards: Array.from({ length: 52 }, (_, i) => ({
              card: i,
              isSelected: false,
              isDead: !(possibleCards & (1n << BigInt(i))),
            })),
            pot: config.startingPot + 2 * totalBetAmount[0],
            stack: config.effectiveStack - totalBetAmount[0],
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

    const spotCards = (spot: SpotRoot | SpotChance) => {
      if (spot.type === "root") {
        return spot.board.map((card) => cardText(card));
      } else if (spot.selectedIndex === -1) {
        return [{ rank: "?", suit: "", colorClass: "text-black" }];
      } else {
        return [cardText(spot.selectedIndex)];
      }
    };

    return {
      spots,
      selectedSpot,
      selectedChance,
      selectSpot,
      play,
      deal,
      spotCards,
    };
  },
});
</script>

<style scoped>
.snug * {
  @apply leading-tight;
}
</style>
