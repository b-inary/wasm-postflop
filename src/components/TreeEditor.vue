<template>
  <div
    ref="divTreeNav"
    class="flex pl-0.5 overflow-x-auto divide-x divide-black"
  >
    <div
      v-for="street in streets"
      :key="street.name"
      :class="{
        'pl-3': street.name !== streets[0].name,
        'pr-3': street.name !== streets[streets.length - 1].name,
      }"
    >
      <p class="font-bold underline">{{ street.name }}</p>
      <div class="flex pt-2 gap-x-2">
        <div
          v-for="decision in street.decisions"
          :key="decision.index"
          class="flex flex-col items-start"
        >
          <button
            v-for="action in decision.actions"
            :key="action.name"
            :class="
              'w-full my-[0.1875rem] px-2 py-[0.0625rem] whitespace-nowrap border rounded-lg shadow text-center select-none ' +
              (!action.isSelected &&
              (street.index - selected.street ||
                decision.index - selected.decision) <= 0
                ? 'opacity-40 '
                : '') +
              (action.isSelected &&
              street.index === selected.street &&
              decision.index === selected.decision
                ? 'ring-1 ring-red-600 border-red-600 '
                : 'border-black')
            "
            @click="moveNode(street.index, decision.index, action.index)"
          >
            {{ action.name }}
          </button>
        </div>
      </div>
    </div>
  </div>

  <div class="mt-4">
    Decision: {{ ["-", "OOP", "IP"][nodeInfo.player + 1] }} / Pot:
    {{ nodeInfo.pot }} / Stack: {{ nodeInfo.stack }}
    {{ nodeInfo.toCall ? `/ To Call: ${nodeInfo.toCall}` : "" }}
  </div>

  <div
    v-if="invalidLinesArray.length > 0"
    class="flex mt-4 font-bold text-red-600"
  >
    <div class="underline">
      Invalid Terminal{{ invalidLinesArray.length > 1 ? "s" : "" }}:
    </div>
    <div class="flex flex-col ml-2">
      <div v-for="invalidLine in invalidLinesArray" :key="invalidLine">
        {{ invalidLine }}
      </div>
    </div>
  </div>

  <div class="flex my-5 justify-center">
    <hr class="border-gray-400 w-2/5" />
  </div>

  <div class="mt-5">
    <button
      class="button-base button-blue"
      :disabled="
        isSelectedTerminal ||
        maxAmount === 0 ||
        betAmount < minAmount ||
        betAmount > maxAmount ||
        betAmount % 1 !== 0 ||
        existingAmounts.includes(betAmount)
      "
      @click="addBetAction"
    >
      Add Bet Action
    </button>
    <span class="ml-3">
      Bet amount:
      <input
        v-model="betAmount"
        type="number"
        :class="
          'w-24 ml-2 px-2 py-1 rounded-lg text-sm text-center ' +
          (betAmount < minAmount ||
          betAmount > maxAmount ||
          betAmount % 1 !== 0 ||
          existingAmounts.includes(betAmount)
            ? 'input-error'
            : '')
        "
        :min="minAmount"
        :max="maxAmount"
        @keydown.enter="addBetAction"
      />
      <span v-if="maxAmount !== 0" class="ml-1">
        ({{ (100 * amountPercent).toFixed(1) }}% of the pot)
      </span>
    </span>
  </div>

  <div class="mt-3">
    <button
      class="button-base button-blue"
      :disabled="selected.street === 0 && selected.decision === 0"
      @click="removeSelectedNode"
    >
      Remove Selected Node
    </button>
  </div>

  <div class="flex my-5 justify-center">
    <hr class="border-gray-400 w-2/5" />
  </div>

  <div class="flex my-5 gap-3">
    <button
      class="button-base button-blue"
      :disabled="invalidLinesArray.length > 0"
      @click="saveEdit"
    >
      Save Edit
    </button>
    <button class="button-base button-red" @click="cancelEdit">Cancel</button>
  </div>

  <div v-if="addedLinesArray.length > 0" class="flex mt-3">
    <div class="font-bold underline w-32">
      Added line{{ addedLinesArray.length > 1 ? "s" : "" }}:
    </div>
    <div class="flex flex-col">
      <div v-for="(addedLine, index) in addedLinesArray" :key="addedLine">
        <button class="relative w-6 mr-2" @click="deleteAddedLine(index)">
          &nbsp;
          <div class="cross-icon"></div>
        </button>
        {{ addedLine }}
      </div>
    </div>
  </div>

  <div v-if="removedLinesArray.length > 0" class="flex mt-3">
    <div class="font-bold underline w-32">
      Removed line{{ removedLinesArray.length > 1 ? "s" : "" }}:
    </div>
    <div class="flex flex-col">
      <div v-for="(removedLine, index) in removedLinesArray" :key="removedLine">
        <button class="relative w-6 mr-2" @click="deleteRemovedLine(index)">
          &nbsp;
          <div class="cross-icon"></div>
        </button>
        {{ removedLine }}
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, nextTick, ref } from "vue";
import { useConfigStore, convertBetString, readableLineString } from "../store";
import { TreeManager } from "../../pkg/tree/tree";

export default defineComponent({
  name: "TreeEditor",

  emits: ["save", "cancel"],

  setup(_, context) {
    const divTreeNav = ref(null as HTMLDivElement | null);

    const config = useConfigStore();

    const treeManager = TreeManager.new(
      config.expectedBoardLength,
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

    const streets = ref([
      {
        index: 0,
        name: ["Flop", "Turn", "River"][config.expectedBoardLength - 3],
        decisions: [
          {
            index: 0,
            actions: [
              {
                index: 0,
                name: "Root",
                isTerminal: false,
                isSelected: true,
              },
            ],
          },
        ],
      },
    ]);

    const selected = ref({
      street: 0,
      decision: 0,
      action: 0,
    });

    const isSelectedTerminal = computed(() => {
      if (selected.value.street === -1) return false;
      const street = streets.value[selected.value.street];
      const decision = street.decisions[selected.value.decision];
      const action = decision.actions[selected.value.action];
      return action.isTerminal;
    });

    const nodeInfo = ref({
      player: -1,
      pot: -1,
      stack: [-1, -1],
      toCall: -1,
      lastAmount: -1,
    });

    const betAmount = ref(0);

    const maxAmount = computed(() => {
      const info = nodeInfo.value;
      const minStack = Math.min(...info.stack);
      return minStack === 0 ? 0 : minStack + info.lastAmount;
    });

    const minAmount = computed(() => {
      const info = nodeInfo.value;
      return Math.min(
        maxAmount.value,
        Math.max(1, info.lastAmount + info.toCall)
      );
    });

    const amountPercent = computed(() => {
      const info = nodeInfo.value;
      const pot = info.pot + info.toCall;
      return (betAmount.value - info.lastAmount) / pot;
    });

    const existingAmounts = computed(() => {
      if (streets.value.length === 0) return [];
      const street = streets.value[streets.value.length - 1];
      if (street.decisions.length === 0) return [];
      const decision = street.decisions[street.decisions.length - 1];
      if (decision.actions.find((a) => a.isSelected)) return [];
      return decision.actions
        .filter((action) => {
          const name = action.name;
          return (
            name.slice(0, 3) === "Bet" ||
            name.slice(0, 5) === "Raise" ||
            name.slice(0, 6) === "All-in"
          );
        })
        .map((action) => Number(action.name.split(" ")[1]));
    });

    const updateNodeInfo = () => {
      let player = 0;
      let lastAmount = 0;
      const pot = [0, 0];

      for (const street of streets.value) {
        player = 0;

        for (const decision of street.decisions) {
          const action = decision.actions.find((a) => a.isSelected);
          if (!action) break;
          if (action.name === "Root") continue;

          if (action.name === "Check") {
            lastAmount = 0;
          } else if (action.name === "Call") {
            pot[player] = pot[1 - player];
            lastAmount = 0;
          } else if (action.name.slice(0, 3) === "Bet") {
            const amount = Number(action.name.slice(4));
            pot[player] += amount;
            lastAmount = amount;
          } else if (
            action.name.slice(0, 5) === "Raise" ||
            action.name.slice(0, 6) === "All-in"
          ) {
            const amount = Number(action.name.slice(6).trimStart());
            const potDiff = pot[1 - player] - pot[player];
            pot[player] += amount - lastAmount + potDiff;
            lastAmount = amount;
          }

          if (action.isTerminal) {
            player = -1;
            break;
          }

          player = 1 - player;
        }
      }

      nodeInfo.value = {
        player,
        pot: config.startingPot + pot[0] + pot[1],
        stack: [config.effectiveStack - pot[0], config.effectiveStack - pot[1]],
        toCall: Math.max(...pot) - Math.min(...pot),
        lastAmount,
      };

      betAmount.value = minAmount.value;
    };

    updateNodeInfo();

    const updateStreets = async () => {
      let decisions = streets.value[streets.value.length - 1].decisions;

      const isChance = treeManager.is_chance_node();
      if (isChance) {
        decisions = [];
        const curStreet = streets.value[streets.value.length - 1].name;
        const nextStreet = { Flop: "Turn", Turn: "River" }[curStreet] ?? "";
        streets.value.push({
          index: streets.value.length,
          name: nextStreet,
          decisions,
        });
      }

      const actionStr = treeManager.actions();
      if (actionStr === "") return;

      const isTerminal = treeManager.is_terminal_action();
      const actions = actionStr.split("/").map((action, index) => ({
        index,
        name: action,
        isTerminal: !!(isTerminal & (1 << index)),
        isSelected: false,
      }));

      decisions.push({ index: decisions.length, actions });

      await nextTick();
      if (divTreeNav.value) {
        const div = divTreeNav.value;
        div.scrollLeft = div.scrollWidth - div.clientWidth;
      }
    };

    updateStreets();

    const addedLines = ref(config.addedLines);
    const removedLines = ref(config.removedLines);
    const invalidLines = ref("");

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

    const play = (actionIndex: number) => {
      const sel = selected.value;
      let street, decision;

      if (sel.decision === streets.value[sel.street].decisions.length - 1) {
        street = sel.street + 1;
        decision = 0;
      } else {
        street = sel.street;
        decision = sel.decision + 1;
      }

      const selectedDecision = streets.value[street].decisions[decision];
      selectedDecision.actions[actionIndex].isSelected = true;

      selected.value = { street, decision, action: actionIndex };

      treeManager?.play(actionIndex);
      updateNodeInfo();
    };

    const undo = (doNotUpdate = false) => {
      const sel = selected.value;
      let street, decision;
      if (sel.decision === 0) {
        street = sel.street - 1;
        decision = streets.value[street].decisions.length - 1;
      } else {
        street = sel.street;
        decision = sel.decision - 1;
      }

      const oldDecision = streets.value[sel.street].decisions[sel.decision];
      oldDecision.actions[sel.action].isSelected = false;

      const newDecision = streets.value[street].decisions[decision];
      const actionIndex = newDecision.actions.findIndex((a) => a.isSelected);

      selected.value = { street, decision, action: actionIndex };

      if (!doNotUpdate) treeManager?.undo();
    };

    const moveNode = (street: number, decision: number, action: number) => {
      if (!treeManager) return;

      const compare = () =>
        street - selected.value.street || decision - selected.value.decision;

      while (compare() < 0) undo();

      if (compare() === 0) {
        if (action === selected.value.action) {
          if (treeManager.is_chance_node()) {
            streets.value.splice(street + 2);
            if (streets.value.length === street + 2) {
              streets.value[street + 1].decisions.splice(1);
            }
          } else {
            streets.value.splice(street + 1);
            streets.value[street].decisions.splice(decision + 2);
          }
          updateNodeInfo();
          return;
        }

        undo();
        streets.value.splice(street + 1);
        streets.value[street].decisions.splice(decision + 1);
      }

      play(action);

      if (isSelectedTerminal.value) return;
      updateStreets();
    };

    const addBetAction = () => {
      treeManager.add_bet_action(betAmount.value, nodeInfo.value.toCall > 0);
      addedLines.value = treeManager.added_lines();
      removedLines.value = treeManager.removed_lines();
      invalidLines.value = treeManager.invalid_terminals();
      if (treeManager.is_chance_node()) {
        streets.value.splice(streets.value.length - 1);
      } else {
        const decisions = streets.value[selected.value.street].decisions;
        decisions.splice(selected.value.decision + 1);
      }
      updateStreets();
    };

    const removeSelectedNode = () => {
      treeManager.remove_current_node();
      undo(true);
      addedLines.value = treeManager.added_lines();
      removedLines.value = treeManager.removed_lines();
      invalidLines.value = treeManager.invalid_terminals();
      const sel = selected.value;
      streets.value.splice(sel.street + 1);
      streets.value[sel.street].decisions.splice(sel.decision + 1);
      updateNodeInfo();
      updateStreets();
    };

    const saveEdit = () => {
      const addedLines = treeManager.added_lines();
      const removedLines = treeManager.removed_lines();
      context.emit("save", addedLines, removedLines);
    };

    const cancelEdit = () => {
      context.emit("cancel");
    };

    const popHistory = (): string[] => {
      const history = [];
      while (!(selected.value.street === 0 && selected.value.decision === 0)) {
        const sel = selected.value;
        const decision = streets.value[sel.street].decisions[sel.decision];
        history.push(decision.actions[sel.action].name);
        undo();
      }
      streets.value.splice(1);
      streets.value[0].decisions.splice(1);
      return history.reverse();
    };

    const applyHistory = (history: string[]) => {
      updateStreets();
      for (const action of history) {
        const sel = selected.value;
        const decisions = streets.value[sel.street].decisions;
        let actions;
        if (sel.decision === decisions.length - 1) {
          if (sel.street === streets.value.length - 1) return;
          if (streets.value[sel.street + 1].decisions.length === 0) return;
          actions = streets.value[sel.street + 1].decisions[0].actions;
        } else {
          actions = decisions[sel.decision + 1].actions;
        }
        const actionIndex = actions.findIndex((a) => a.name === action);
        if (actionIndex === -1) return;
        play(actionIndex);
        if (isSelectedTerminal.value) return;
        updateStreets();
      }
    };

    const deleteAddedLine = (index: number) => {
      const addedLinesArray = addedLines.value.split(",");
      const line = addedLinesArray[index];

      const history = popHistory();
      treeManager.delete_added_line(line);
      applyHistory(history);

      addedLines.value = treeManager.added_lines();
      removedLines.value = treeManager.removed_lines();
      invalidLines.value = treeManager.invalid_terminals();
    };

    const deleteRemovedLine = (index: number) => {
      const removedLinesArray = removedLines.value.split(",");
      const line = removedLinesArray[index];

      const history = popHistory();
      treeManager.delete_removed_line(line);
      applyHistory(history);

      addedLines.value = treeManager.added_lines();
      removedLines.value = treeManager.removed_lines();
      invalidLines.value = treeManager.invalid_terminals();
    };

    return {
      divTreeNav,
      streets,
      selected,
      isSelectedTerminal,
      nodeInfo,
      betAmount,
      maxAmount,
      minAmount,
      amountPercent,
      existingAmounts,
      addedLinesArray,
      removedLinesArray,
      invalidLinesArray,
      moveNode,
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
