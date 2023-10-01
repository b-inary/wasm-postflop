<template>
  <!-- Error -->

  <div v-if="isTreeError">
    Error: Failed to build tree (loaded broken tree?)
  </div>

  <!-- Navigation -->

  <div
    v-if="!isTreeError"
    ref="navDiv"
    class="flex h-[10.5rem] gap-1 p-1 overflow-x-auto whitespace-nowrap snug"
  >
    <div
      v-for="spot in spots"
      :key="spot.index"
      :class="
        'flex flex-col h-full px-1 py-0.5 justify-start ' +
        'rounded-lg shadow-md border-[3px] transition group ' +
        (spot.type === 'chance'
          ? 'hover:border-red-600 '
          : 'hover:border-blue-600 ') +
        (spot.index === selectedSpotIndex
          ? 'border-blue-600 cursor-default'
          : 'border-gray-400 cursor-pointer')
      "
      @click="selectSpot(spot.index, false, false, true)"
    >
      <!-- Root or Chance -->
      <template v-if="spot.type === 'root' || spot.type === 'chance'">
        <div
          class="px-1.5 pt-1 pb-0.5 font-semibold group-hover:opacity-100 opacity-70"
        >
          {{ spot.player.toUpperCase() }}
        </div>
        <div
          class="flex flex-col flex-grow px-3 items-center justify-evenly font-semibold"
        >
          <div class="group-hover:opacity-100 opacity-70">
            <div>Pot {{ spot.pot }}</div>
            <div>Stack {{ spot.stack }}</div>
          </div>
        </div>
      </template>

      <!-- Player -->
      <template v-if="spot.type === 'player'">
        <div
          :class="
            'px-1.5 py-1 font-semibold group-hover:opacity-100 ' +
            (spot.index === selectedSpotIndex ? '' : 'opacity-70')
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
            @click.stop="play(spot.index, action.index)"
          >
            <span class="inline-block relative w-4 mr-0.5">
              <span v-if="action.isSelected">
                <CheckIcon class="absolute top-[0.1875rem] -left-0.5 w-4 h-4" />
              </span>
            </span>
            <span
              :class="
                'pr-0.5 font-semibold group-hover:opacity-100 ' +
                (action.isSelected || spot.index === selectedSpotIndex
                  ? ''
                  : 'opacity-70')
              "
            >
              {{ action.name }}
              {{ action.amount === "0" ? "" : action.amount }}
            </span>
          </button>
          <div
            v-if="spot.actions.length === 0"
            :class="
              'flex w-full px-1.5 font-semibold group-hover:opacity-100 ' +
              (spot.index === selectedSpotIndex ? '' : 'opacity-70')
            "
          >
            (No actions)
          </div>
        </div>
      </template>

      <!-- Terminal -->
      <template v-else-if="spot.type === 'terminal'">
        <div
          :class="
            'px-1.5 pt-1 pb-0.5 font-semibold group-hover:opacity-100 ' +
            (spot.index === selectedSpotIndex ? '' : 'opacity-70')
          "
        >
          {{ spot.player.toUpperCase() }}
        </div>
        <div
          :class="
            'flex flex-col flex-grow items-center justify-evenly font-semibold group-hover:opacity-100 ' +
            (spot.index === selectedSpotIndex ? '' : 'opacity-70')
          "
        >
          <div v-if="spot.equityOop === 0 || spot.equityOop === 1" class="px-3">
            {{ ["IP", "OOP"][spot.equityOop] }} Wins
          </div>
          <div class="px-3">Pot {{ spot.pot }}</div>
        </div>
      </template>
    </div>
  </div>

  <!-- Invalid lines error -->

  <div
    v-if="!isTreeError && invalidLinesArray.length > 0"
    class="flex mt-4 font-semibold text-red-500"
  >
    <div class="underline">
      Invalid Terminal{{ invalidLinesArray.length > 1 ? "s" : "" }}:
    </div>
    <div class="ml-2">
      <div v-for="invalidLine in invalidLinesArray" :key="invalidLine">
        {{ invalidLine }}
      </div>
    </div>
  </div>

  <div v-if="!isTreeError" class="flex mx-6 my-6 justify-center">
    <hr class="border-gray-400 w-full" />
  </div>

  <!-- Edit -->

  <div v-if="!isTreeError" class="flex gap-3">
    <button
      class="button-base button-blue"
      :disabled="
        isSelectedTerminal ||
        isAfterAllin ||
        betAmount < minAmount ||
        betAmount > maxAmount ||
        betAmount % 1 !== 0 ||
        existingAmounts.includes(betAmount)
      "
      @click="addBetAction"
    >
      Add Bet Action
    </button>

    <button
      class="button-base button-red"
      :disabled="selectedSpotIndex === 1"
      @click="removeSelectedNode"
    >
      Remove Selected Node
    </button>

    <div class="pl-3">
      Bet amount:
      <input
        v-model="betAmount"
        type="number"
        :class="
          'w-24 ml-2 px-2 py-1 rounded-lg text-sm text-center ' +
          (betAmount < minAmount || betAmount > maxAmount || betAmount % 1 !== 0
            ? 'input-error'
            : '')
        "
        :min="minAmount"
        :max="maxAmount"
        @keydown.enter="addBetAction"
      />
      <span v-if="!isSelectedTerminal && !isAfterAllin" class="ml-2">
        ({{ (amountRate * 100).toFixed(1) }}% of the pot)
      </span>
    </div>
  </div>

  <div class="flex mx-6 my-6 justify-center">
    <hr class="border-gray-400 w-full" />
  </div>

  <!-- Save/Cancel -->

  <div class="flex my-6 gap-3">
    <button
      class="button-base button-blue"
      :disabled="isTreeError || invalidLinesArray.length > 0"
      @click="saveEdit"
    >
      Save Edit
    </button>

    <button class="button-base button-red" @click="cancelEdit">
      Cancel Edit
    </button>
  </div>

  <!-- Lines -->

  <div
    v-if="
      !isTreeError &&
      (addedLinesArray.length > 0 || removedLinesArray.length > 0)
    "
  >
    <div v-if="addedLinesArray.length > 0" class="flex">
      <div class="font-semibold underline w-[7.75rem]">
        Added line{{ addedLinesArray.length > 1 ? "s" : "" }}:
      </div>
      <div class="flex flex-col">
        <div
          v-for="(addedLine, index) in addedLinesArray"
          :key="addedLine"
          class="flex items-center"
        >
          <button class="mr-2" @click="deleteAddedLine(index)">
            <TrashIcon class="w-5 h-5 text-gray-600" />
          </button>
          <span>{{ addedLine }}</span>
        </div>
      </div>
    </div>

    <div v-if="removedLinesArray.length > 0" class="flex mt-2">
      <div class="font-semibold underline w-[7.75rem]">
        Removed line{{ removedLinesArray.length > 1 ? "s" : "" }}:
      </div>
      <div class="flex flex-col">
        <div
          v-for="(removedLine, index) in removedLinesArray"
          :key="removedLine"
          class="flex items-center"
        >
          <button class="mr-2" @click="deleteRemovedLine(index)">
            <TrashIcon class="w-5 h-5 text-gray-600" />
          </button>
          <span>{{ removedLine }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, nextTick, ref } from "vue";
import { useConfigStore } from "../store";
import { convertBetString, readableLineString } from "../utils";
import { Spot, SpotRoot, SpotChance, SpotPlayer } from "../result-types";
import { TreeManager } from "../../pkg/tree/tree";

import { CheckIcon } from "@heroicons/vue/20/solid";
import { TrashIcon } from "@heroicons/vue/24/outline";

export default defineComponent({
  components: {
    CheckIcon,
    TrashIcon,
  },

  emits: {
    save: (_addedLines: string, _removedLines: string) => true,
    cancel: () => true,
  },

  setup(_, context) {
    const navDiv = ref(null as HTMLDivElement | null);

    const config = useConfigStore();

    const boardLength = config.expectedBoardLength;
    const treeManager = TreeManager.new(
      boardLength,
      config.startingPot,
      config.effectiveStack,
      config.donkOption,
      convertBetString(config.oopFlopBet),
      convertBetString(config.oopFlopRaise),
      convertBetString(config.oopTurnBet),
      convertBetString(config.oopTurnRaise),
      config.donkOption ? convertBetString(config.oopTurnDonk) : "",
      convertBetString(config.oopRiverBet),
      convertBetString(config.oopRiverRaise),
      config.donkOption ? convertBetString(config.oopRiverDonk) : "",
      convertBetString(config.ipFlopBet),
      convertBetString(config.ipFlopRaise),
      convertBetString(config.ipTurnBet),
      convertBetString(config.ipTurnRaise),
      convertBetString(config.ipRiverBet),
      convertBetString(config.ipRiverRaise),
      config.addAllInThreshold / 100,
      config.forceAllInThreshold / 100,
      config.mergingThreshold / 100,
      config.addedLines,
      config.removedLines
    );

    const isTreeError = treeManager.is_error();

    const rootSpot: SpotRoot = {
      type: "root",
      index: 0,
      player: boardLength === 3 ? "flop" : boardLength === 4 ? "turn" : "river",
      selectedIndex: -1,
      board: config.board,
      pot: config.startingPot,
      stack: config.effectiveStack,
    };
    const spots = ref<Spot[]>([rootSpot]);
    const selectedSpotIndex = ref(-1);

    const isSelectedTerminal = computed(() => {
      if (selectedSpotIndex.value === -1) return false;
      const selectedSpot = spots.value[selectedSpotIndex.value];
      return selectedSpot.type === "terminal";
    });

    const betAmount = ref(0);
    const totalBetAmount = ref([0, 0]);
    const prevBetAmount = ref(0);

    const isAfterAllin = computed(() => {
      const maxTotalBetAmount = Math.max(...totalBetAmount.value);
      return maxTotalBetAmount === config.effectiveStack;
    });

    const maxAmount = computed(() => {
      if (isSelectedTerminal.value) return 0;
      const maxTotalBetAmount = Math.max(...totalBetAmount.value);
      return config.effectiveStack - (maxTotalBetAmount - prevBetAmount.value);
    });

    const minAmount = computed(() => {
      const betMinus = config.effectiveStack - maxAmount.value;
      const min = Math.min(...totalBetAmount.value) - betMinus;
      const max = Math.max(...totalBetAmount.value) - betMinus;
      return Math.min(Math.max(2 * max - min, 1), maxAmount.value);
    });

    const amountRate = computed(() => {
      const pot = config.startingPot + 2 * Math.max(...totalBetAmount.value);
      const amount = betAmount.value - prevBetAmount.value;
      return amount / pot;
    });

    const existingAmounts = computed(() => {
      if (selectedSpotIndex.value === -1) return [];
      const ret: number[] = [];
      const spot = spots.value[selectedSpotIndex.value] as SpotPlayer;
      for (const action of spot.actions) {
        if (action.amount !== "0") {
          ret.push(Number(action.amount));
        }
      }
      return ret;
    });

    const addedLines = ref(treeManager.added_lines());
    const removedLines = ref(treeManager.removed_lines());
    const invalidLines = ref(treeManager.invalid_terminals());

    const addedLinesArray = computed(() =>
      addedLines.value === ""
        ? []
        : addedLines.value.split(",").map(readableLineString)
    );

    const removedLinesArray = computed(() =>
      removedLines.value === ""
        ? []
        : removedLines.value.split(",").map(readableLineString)
    );

    const invalidLinesArray = computed(() =>
      invalidLines.value === ""
        ? []
        : invalidLines.value.split(",").map(readableLineString)
    );

    const encodeLine = (spotIndex: number) => {
      const ret: string[] = [];
      for (let i = 1; i < spotIndex; ++i) {
        const spot = spots.value[i];
        if (spot.type === "player") {
          const action = spot.actions[spot.selectedIndex];
          if (action.name === "Fold") {
            ret.push("F");
          } else if (action.name === "Check") {
            ret.push("X");
          } else if (action.name === "Call") {
            ret.push("C");
          } else if (action.name === "Bet") {
            ret.push("B" + action.amount);
          } else if (action.name === "Raise") {
            ret.push("R" + action.amount);
          } else if (action.name === "Allin") {
            ret.push("A" + action.amount);
          }
        }
      }
      return ret;
    };

    const selectSpot = (
      spotIndex: number,
      needSplice: boolean,
      needRebuild: boolean,
      needAmountUpdate: boolean
    ) => {
      if (
        !needSplice &&
        !needRebuild &&
        spotIndex === selectedSpotIndex.value
      ) {
        return;
      }

      if (spotIndex === 0) {
        selectSpot(1, true, false, selectedSpotIndex.value !== 1);
        return;
      }

      if (!needSplice && spots.value[spotIndex]?.type === "chance") {
        selectSpot(spotIndex + 1, false, false, true);
        return;
      }

      if (needRebuild) {
        const selectedSpotIndexTmp = selectedSpotIndex.value;
        const line = encodeLine(spots.value.length - 1);
        spots.value = [rootSpot];

        selectedSpotIndex.value = 1;
        totalBetAmount.value = [0, 0];

        treeManager.back_to_root();
        pushResultsPlayer();

        for (let i = 0; i < line.length; ++i) {
          const index = treeManager.play(line[i]);
          if (index === -1) {
            needAmountUpdate = true;
            break;
          }

          const spot = spots.value[selectedSpotIndex.value] as SpotPlayer;
          const action = spot.actions[index];
          spot.selectedIndex = index;
          action.isSelected = true;

          ++selectedSpotIndex.value;
          totalBetAmount.value = Array.from(treeManager.total_bet_amount());

          if (treeManager.is_terminal_node()) {
            pushResultsTerminal();
          } else if (treeManager.is_chance_node()) {
            pushResultsChance();
            ++selectedSpotIndex.value;
          } else {
            pushResultsPlayer();
          }
        }

        if (selectedSpotIndexTmp < selectedSpotIndex.value) {
          selectedSpotIndex.value = selectedSpotIndexTmp;
        }
      } else {
        selectedSpotIndex.value = spotIndex;
      }

      const line = encodeLine(selectedSpotIndex.value);
      treeManager.apply_history(line.join("-"));
      totalBetAmount.value = Array.from(treeManager.total_bet_amount());

      if (needSplice) {
        spots.value.splice(selectedSpotIndex.value);
        if (treeManager.is_terminal_node()) {
          pushResultsTerminal();
        } else if (treeManager.is_chance_node()) {
          pushResultsChance();
          ++selectedSpotIndex.value;
        } else {
          pushResultsPlayer();
        }
      }

      const prev = spots.value[selectedSpotIndex.value - 1];
      if (prev.type === "player") {
        prevBetAmount.value = Number(prev.actions[prev.selectedIndex].amount);
      } else {
        prevBetAmount.value = 0;
      }

      if (needAmountUpdate) {
        betAmount.value = minAmount.value;
      }

      autoScrollNav();
    };

    const autoScrollNav = async () => {
      await nextTick();
      if (navDiv.value) {
        const selectedChild = navDiv.value.children[selectedSpotIndex.value];
        if (selectedChild) {
          selectedChild.scrollIntoView({
            behavior: "smooth",
            inline: "center",
          });
        }
      }
    };

    const pushResultsTerminal = () => {
      const prevSpot = spots.value[selectedSpotIndex.value - 1] as SpotPlayer;
      const prevAction = prevSpot.actions[prevSpot.selectedIndex];

      let equityOop = -1;
      if (prevAction.name === "Fold") {
        equityOop = prevSpot.player === "oop" ? 0 : 1;
      }

      spots.value.push({
        type: "terminal",
        index: selectedSpotIndex.value,
        player: "end",
        selectedIndex: -1,
        prevPlayer: prevSpot.player,
        equityOop,
        pot:
          config.startingPot +
          totalBetAmount.value[0] +
          totalBetAmount.value[1],
      });
    };

    const pushResultsChance = () => {
      type SpotTurn = SpotRoot | SpotChance;
      const prevSpot = spots.value[selectedSpotIndex.value - 1] as SpotPlayer;
      const turnSpot = spots.value.find((spot) => spot.player === "turn") as
        | SpotTurn
        | undefined;

      const nextActions = treeManager.actions().split("/");
      if (nextActions[0] === "") nextActions.pop();

      spots.value.push(
        {
          type: "chance",
          index: selectedSpotIndex.value,
          player: turnSpot ? "river" : "turn",
          selectedIndex: -1,
          prevPlayer: prevSpot.player,
          cards: Array.from({ length: 52 }, (_, i) => ({
            card: i,
            isSelected: false,
            isDead: true,
          })),
          pot: config.startingPot + 2 * totalBetAmount.value[0],
          stack: config.effectiveStack - totalBetAmount.value[0],
        },
        {
          type: "player",
          index: selectedSpotIndex.value + 1,
          player: "oop",
          selectedIndex: -1,
          actions: nextActions.map((action, i) => {
            const [name, amount] = action.split(":");
            return {
              index: i,
              name,
              amount,
              rate: -1,
              isSelected: false,
              color: "#000",
            };
          }),
        }
      );
    };

    const pushResultsPlayer = () => {
      const prevSpot = spots.value[selectedSpotIndex.value - 1];
      const player = prevSpot.player === "oop" ? "ip" : "oop";

      const actions = treeManager.actions().split("/");
      if (actions[0] === "") actions.pop();

      spots.value.push({
        type: "player",
        index: selectedSpotIndex.value,
        player,
        selectedIndex: -1,
        actions: actions.map((action, i) => {
          const [name, amount] = action.split(":");
          return {
            index: i,
            name,
            amount,
            isSelected: false,
            color: "#000",
          };
        }),
      });
    };

    const play = (spotIndex: number, actionIndex: number) => {
      const spot = spots.value[spotIndex] as SpotPlayer;

      if (spot.selectedIndex !== -1) {
        spot.actions[spot.selectedIndex].isSelected = false;
      }
      spot.actions[actionIndex].isSelected = true;
      spot.selectedIndex = actionIndex;

      selectSpot(spotIndex + 1, true, false, true);
    };

    const addBetAction = () => {
      const isRaise = totalBetAmount.value[0] !== totalBetAmount.value[1];
      treeManager.add_bet_action(betAmount.value, isRaise);
      selectSpot(selectedSpotIndex.value, false, true, false);
      addedLines.value = treeManager.added_lines();
      removedLines.value = treeManager.removed_lines();
      invalidLines.value = treeManager.invalid_terminals();
    };

    const removeSelectedNode = () => {
      treeManager.remove_current_node();
      let prevIndex = selectedSpotIndex.value - 1;
      if (spots.value[prevIndex].type === "chance") --prevIndex;
      selectSpot(prevIndex, false, true, true);
      addedLines.value = treeManager.added_lines();
      removedLines.value = treeManager.removed_lines();
      invalidLines.value = treeManager.invalid_terminals();
    };

    const saveEdit = () => {
      context.emit("save", addedLines.value, removedLines.value);
    };

    const cancelEdit = () => {
      context.emit("cancel");
    };

    const deleteAddedLine = (index: number) => {
      const addedLinesArray = addedLines.value.split(",");
      const line = addedLinesArray[index];

      treeManager.delete_added_line(line);
      selectSpot(selectedSpotIndex.value, false, true, false);

      addedLines.value = treeManager.added_lines();
      removedLines.value = treeManager.removed_lines();
      invalidLines.value = treeManager.invalid_terminals();
    };

    const deleteRemovedLine = (index: number) => {
      const removedLinesArray = removedLines.value.split(",");
      const line = removedLinesArray[index];

      treeManager.delete_removed_line(line);
      selectSpot(selectedSpotIndex.value, false, true, false);

      addedLines.value = treeManager.added_lines();
      removedLines.value = treeManager.removed_lines();
      invalidLines.value = treeManager.invalid_terminals();
    };

    selectSpot(0, true, false, true);

    return {
      navDiv,
      isTreeError,
      spots,
      selectedSpotIndex,
      isSelectedTerminal,
      betAmount,
      isAfterAllin,
      maxAmount,
      minAmount,
      amountRate,
      existingAmounts,
      addedLinesArray,
      removedLinesArray,
      invalidLinesArray,
      selectSpot,
      play,
      addBetAction,
      removeSelectedNode,
      saveEdit,
      cancelEdit,
      deleteAddedLine,
      deleteRemovedLine,
    };
  },
});
</script>
