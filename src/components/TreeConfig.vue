<template>
  <div v-if="!isEditMode">
    <div class="flex">
      <div class="shrink-0">
        <div class="flex my-1 gap-8">
          <div>
            <div class="my-1">
              <span class="inline-block w-[7.25rem]">Starting pot:</span>
              <input
                v-model="config.startingPot"
                type="number"
                :class="
                  'w-24 px-2 py-1 rounded-lg text-sm text-center ' +
                  (config.startingPot <= 0 ||
                  config.startingPot > 100000 ||
                  config.startingPot % 1 !== 0
                    ? 'input-error'
                    : '')
                "
                :disabled="hasEdit"
                min="0"
                max="100000"
              />
            </div>

            <div class="my-1">
              <span class="inline-block w-[7.25rem]">Effective stack:</span>
              <input
                v-model="config.effectiveStack"
                type="number"
                :class="
                  'w-24 px-2 py-1 rounded-lg text-sm text-center ' +
                  (config.effectiveStack <= 0 ||
                  config.effectiveStack > 100000 ||
                  config.effectiveStack % 1 !== 0
                    ? 'input-error'
                    : '')
                "
                :disabled="hasEdit"
                min="0"
                max="100000"
              />
            </div>
          </div>

          <div>
            <div class="my-1">
              <span class="inline-block w-20">Rake:</span>
              <input
                v-model="config.rakePercent"
                type="number"
                :class="
                  'w-24 px-2 py-1 rounded-lg text-sm text-center ' +
                  (config.rakePercent < 0 || config.rakePercent > 100
                    ? 'input-error'
                    : '')
                "
                :disabled="hasEdit"
                min="0"
                max="100"
                step="0.5"
              />
              %
            </div>

            <div class="my-1">
              <span class="inline-block w-20">Rake cap:</span>
              <input
                v-model="config.rakeCap"
                type="number"
                :class="
                  'w-24 px-2 py-1 rounded-lg text-sm text-center ' +
                  (config.rakeCap < 0 || config.rakeCap > 100000
                    ? 'input-error'
                    : '')
                "
                :disabled="hasEdit"
                min="0"
                max="100000"
              />
            </div>
          </div>

          <div class="ml-auto p-1">
            <button class="button-base button-blue" @click="clearConfig">
              Clear
            </button>
          </div>
        </div>

        <div class="mt-6">
          <p>
            <span class="font-bold">OOP bet sizes</span>
            <label class="inline-block ml-8">
              <input
                v-model="config.donkOption"
                type="checkbox"
                class="mr-1 align-middle rounded cursor-pointer peer"
                :disabled="hasEdit"
              />
              <span class="cursor-pointer peer-disabled:cursor-not-allowed">
                Use different sizes for donk bets
              </span>
            </label>
          </p>

          <div class="flex gap-5">
            <div>
              <p class="my-1 underline">Flop</p>
              <p class="my-1">
                <span class="inline-block w-14">Bet:</span>
                <input
                  v-model="config.oopFlopBet"
                  type="text"
                  :class="
                    'w-24 px-2 py-1 rounded-lg text-sm ' +
                    (!config.oopFlopBetSanitized.valid ? 'input-error' : '')
                  "
                  :disabled="hasEdit"
                  @change="
                    config.oopFlopBetSanitized.valid &&
                      (config.oopFlopBet = config.oopFlopBetSanitized.s)
                  "
                />
                %
              </p>
              <p class="my-1">
                <span class="inline-block w-14">Raise:</span>
                <input
                  v-model="config.oopFlopRaise"
                  type="text"
                  :class="
                    'w-24 px-2 py-1 rounded-lg text-sm ' +
                    (!config.oopFlopRaiseSanitized.valid ? 'input-error' : '')
                  "
                  :disabled="hasEdit"
                  @change="
                    config.oopFlopRaiseSanitized.valid &&
                      (config.oopFlopRaise = config.oopFlopRaiseSanitized.s)
                  "
                />
                %
              </p>
            </div>

            <div>
              <p class="my-1 underline">Turn</p>
              <p class="my-1">
                <span class="inline-block w-14">Bet:</span>
                <input
                  v-model="config.oopTurnBet"
                  type="text"
                  :class="
                    'w-24 px-2 py-1 rounded-lg text-sm ' +
                    (!config.oopTurnBetSanitized.valid ? 'input-error' : '')
                  "
                  :disabled="hasEdit"
                  @change="
                    config.oopTurnBetSanitized.valid &&
                      (config.oopTurnBet = config.oopTurnBetSanitized.s)
                  "
                />
                %
              </p>
              <p class="my-1">
                <span class="inline-block w-14">Raise:</span>
                <input
                  v-model="config.oopTurnRaise"
                  type="text"
                  :class="
                    'w-24 px-2 py-1 rounded-lg text-sm ' +
                    (!config.oopTurnRaiseSanitized.valid ? 'input-error' : '')
                  "
                  :disabled="hasEdit"
                  @change="
                    config.oopTurnRaiseSanitized.valid &&
                      (config.oopTurnRaise = config.oopTurnRaiseSanitized.s)
                  "
                />
                %
              </p>
              <p v-if="config.donkOption" class="my-1">
                <span class="inline-block w-14">Donk:</span>
                <input
                  v-model="config.oopTurnDonk"
                  type="text"
                  :class="
                    'w-24 px-2 py-1 rounded-lg text-sm ' +
                    (!config.oopTurnDonkSanitized.valid ? 'input-error' : '')
                  "
                  :disabled="hasEdit"
                  @change="
                    config.oopTurnDonkSanitized.valid &&
                      (config.oopTurnDonk = config.oopTurnDonkSanitized.s)
                  "
                />
                %
              </p>
            </div>

            <div>
              <p class="my-1 underline">River</p>
              <p class="my-1">
                <span class="inline-block w-14">Bet:</span>
                <input
                  v-model="config.oopRiverBet"
                  type="text"
                  :class="
                    'w-24 px-2 py-1 rounded-lg text-sm ' +
                    (!config.oopRiverBetSanitized.valid ? 'input-error' : '')
                  "
                  :disabled="hasEdit"
                  @change="
                    config.oopRiverBetSanitized.valid &&
                      (config.oopRiverBet = config.oopRiverBetSanitized.s)
                  "
                />
                %
              </p>
              <p class="my-1">
                <span class="inline-block w-14">Raise:</span>
                <input
                  v-model="config.oopRiverRaise"
                  type="text"
                  :class="
                    'w-24 px-2 py-1 rounded-lg text-sm ' +
                    (!config.oopRiverRaiseSanitized.valid ? 'input-error' : '')
                  "
                  :disabled="hasEdit"
                  @change="
                    config.oopRiverRaiseSanitized.valid &&
                      (config.oopRiverRaise = config.oopRiverRaiseSanitized.s)
                  "
                />
                %
              </p>
              <p v-if="config.donkOption" class="my-1">
                <span class="inline-block w-14">Donk:</span>
                <input
                  v-model="config.oopRiverDonk"
                  type="text"
                  :class="
                    'w-24 px-2 py-1 rounded-lg text-sm ' +
                    (!config.oopRiverDonkSanitized.valid ? 'input-error' : '')
                  "
                  :disabled="hasEdit"
                  @change="
                    config.oopRiverDonkSanitized.valid &&
                      (config.oopRiverDonk = config.oopRiverDonkSanitized.s)
                  "
                />
                %
              </p>
            </div>
          </div>
        </div>

        <div>
          <div class="flex">
            <p class="mt-6 font-bold">IP bet sizes</p>
            <div class="flex flex-grow items-center justify-center gap-6">
              <button
                class="mt-3 button-base button-blue button-arrow"
                :disabled="isIpError || hasEdit"
                @click="ipToOop"
              >
                ↑
              </button>
              <button
                class="mt-3 button-base button-blue button-arrow"
                :disabled="isOopError || hasEdit"
                @click="oopToIp"
              >
                ↓
              </button>
            </div>
          </div>

          <div class="flex gap-5">
            <div>
              <p class="my-1 underline">Flop</p>
              <p class="my-1">
                <span class="inline-block w-14">Bet:</span>
                <input
                  v-model="config.ipFlopBet"
                  type="text"
                  :class="
                    'w-24 px-2 py-1 rounded-lg text-sm ' +
                    (!config.ipFlopBetSanitized.valid ? 'input-error' : '')
                  "
                  :disabled="hasEdit"
                  @change="
                    config.ipFlopBetSanitized.valid &&
                      (config.ipFlopBet = config.ipFlopBetSanitized.s)
                  "
                />
                %
              </p>
              <p class="my-1">
                <span class="inline-block w-14">Raise:</span>
                <input
                  v-model="config.ipFlopRaise"
                  type="text"
                  :class="
                    'w-24 px-2 py-1 rounded-lg text-sm ' +
                    (!config.ipFlopRaiseSanitized.valid ? 'input-error' : '')
                  "
                  :disabled="hasEdit"
                  @change="
                    config.ipFlopRaiseSanitized.valid &&
                      (config.ipFlopRaise = config.ipFlopRaiseSanitized.s)
                  "
                />
                %
              </p>
            </div>

            <div>
              <p class="my-1 underline">Turn</p>
              <p class="my-1">
                <span class="inline-block w-14">Bet:</span>
                <input
                  v-model="config.ipTurnBet"
                  type="text"
                  :class="
                    'w-24 px-2 py-1 rounded-lg text-sm ' +
                    (!config.ipTurnBetSanitized.valid ? 'input-error' : '')
                  "
                  :disabled="hasEdit"
                  @change="
                    config.ipTurnBetSanitized.valid &&
                      (config.ipTurnBet = config.ipTurnBetSanitized.s)
                  "
                />
                %
              </p>
              <p class="my-1">
                <span class="inline-block w-14">Raise:</span>
                <input
                  v-model="config.ipTurnRaise"
                  type="text"
                  :class="
                    'w-24 px-2 py-1 rounded-lg text-sm ' +
                    (!config.ipTurnRaiseSanitized.valid ? 'input-error' : '')
                  "
                  :disabled="hasEdit"
                  @change="
                    config.ipTurnRaiseSanitized.valid &&
                      (config.ipTurnRaise = config.ipTurnRaiseSanitized.s)
                  "
                />
                %
              </p>
            </div>

            <div>
              <p class="my-1 underline">River</p>
              <p class="my-1">
                <span class="inline-block w-14">Bet:</span>
                <input
                  v-model="config.ipRiverBet"
                  type="text"
                  :class="
                    'w-24 px-2 py-1 rounded-lg text-sm ' +
                    (!config.ipRiverBetSanitized.valid ? 'input-error' : '')
                  "
                  :disabled="hasEdit"
                  @change="
                    config.ipRiverBetSanitized.valid &&
                      (config.ipRiverBet = config.ipRiverBetSanitized.s)
                  "
                />
                %
              </p>
              <p class="my-1">
                <span class="inline-block w-14">Raise:</span>
                <input
                  v-model="config.ipRiverRaise"
                  type="text"
                  :class="
                    'w-24 px-2 py-1 rounded-lg text-sm ' +
                    (!config.ipRiverRaiseSanitized.valid ? 'input-error' : '')
                  "
                  :disabled="hasEdit"
                  @change="
                    config.ipRiverRaiseSanitized.valid &&
                      (config.ipRiverRaise = config.ipRiverRaiseSanitized.s)
                  "
                />
                %
              </p>
            </div>
          </div>
        </div>

        <div class="flex mt-6 gap-4">
          <div>
            <p class="my-1">
              <span class="inline-block w-[10.5rem]">
                Add all-in threshold:
              </span>
              <input
                v-model="config.addAllInThreshold"
                type="number"
                :class="
                  'w-24 px-2 py-1 rounded-lg text-sm text-center ' +
                  (config.addAllInThreshold < 0 ? 'input-error' : '')
                "
                :disabled="hasEdit"
                min="0"
                max="10000000"
              />
              %
            </p>

            <p class="my-1">
              <span class="inline-block w-[10.5rem]">
                Force all-in threshold:
              </span>
              <input
                v-model="config.forceAllInThreshold"
                type="number"
                :class="
                  'w-24 px-2 py-1 rounded-lg text-sm text-center ' +
                  (config.forceAllInThreshold < 0 ? 'input-error' : '')
                "
                :disabled="hasEdit"
                min="0"
                max="10000000"
              />
              %
            </p>

            <p class="my-1">
              <span class="inline-block w-[10.5rem]">Merging threshold:</span>
              <input
                v-model="config.mergingThreshold"
                type="number"
                :class="
                  'w-24 px-2 py-1 rounded-lg text-sm text-center ' +
                  (config.mergingThreshold < 0 ? 'input-error' : '')
                "
                :disabled="hasEdit"
                min="0"
                max="10000000"
              />
              %
            </p>
          </div>

          <div class="flex justify-center flex-grow">
            <div class="flex flex-col justify-center gap-3">
              <button
                class="button-base button-blue"
                :disabled="!isInputValid"
                @click="startEdit"
              >
                Preview & Edit Tree
              </button>

              <button
                v-if="hasEdit"
                class="button-base button-red"
                @click="clearEdit"
              >
                Clear Edit & Unlock
              </button>
            </div>
          </div>
        </div>
      </div>

      <DbItemPicker
        class="mt-1 ml-6"
        store-name="configurations"
        :value="dbValue"
        :allow-save="isInputValid"
        @load-item="loadConfig"
      />
    </div>

    <div
      v-if="addedLinesArray.length > 0 || removedLinesArray.length > 0"
      class="mt-6"
    >
      <div v-if="addedLinesArray.length > 0" class="flex mt-3">
        <div class="font-bold underline w-32">
          Added line{{ addedLinesArray.length > 1 ? "s" : "" }}:
        </div>
        <div class="flex flex-col">
          <div v-for="addedLine in addedLinesArray" :key="addedLine">
            {{ addedLine }}
          </div>
        </div>
      </div>

      <div v-if="removedLinesArray.length > 0" class="flex mt-3">
        <div class="font-bold underline w-32">
          Removed line{{ removedLinesArray.length > 1 ? "s" : "" }}:
        </div>
        <div class="flex flex-col">
          <div v-for="removedLine in removedLinesArray" :key="removedLine">
            {{ removedLine }}
          </div>
        </div>
      </div>
    </div>
  </div>

  <div v-else>
    <TreeEditor @save="saveEdit" @cancel="cancelEdit" />
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, ref } from "vue";
import { useConfigStore } from "../store";
import { readableLineString } from "../utils";

import DbItemPicker from "./DbItemPicker.vue";
import TreeEditor from "./TreeEditor.vue";

type ConfigValue = {
  startingPot: number;
  effectiveStack: number;
  rakePercent: number;
  rakeCap: number;
  donkOption: number;
  oopFlopBet: string;
  oopFlopRaise: string;
  oopTurnBet: string;
  oopTurnRaise: string;
  oopTurnDonk: string;
  oopRiverBet: string;
  oopRiverRaise: string;
  oopRiverDonk: string;
  ipFlopBet: string;
  ipFlopRaise: string;
  ipTurnBet: string;
  ipTurnRaise: string;
  ipRiverBet: string;
  ipRiverRaise: string;
  addAllInThreshold: number;
  forceAllInThreshold: number;
  mergingThreshold: number;
  expectedBoardLength: number;
  addedLines: string;
  removedLines: string;
};

export default defineComponent({
  components: {
    DbItemPicker,
    TreeEditor,
  },

  setup() {
    const config = useConfigStore();

    const isEditMode = ref(false);

    const hasEdit = computed(
      () => config.addedLines.length > 0 || config.removedLines.length > 0
    );

    const addedLinesArray = computed(() =>
      config.addedLines === ""
        ? []
        : config.addedLines.split(",").map(readableLineString)
    );

    const removedLinesArray = computed(() =>
      config.removedLines === ""
        ? []
        : config.removedLines.split(",").map(readableLineString)
    );

    const isOopError = computed(
      () =>
        !config.oopFlopBetSanitized.valid ||
        !config.oopFlopRaiseSanitized.valid ||
        !config.oopTurnBetSanitized.valid ||
        !config.oopTurnRaiseSanitized.valid ||
        !config.oopRiverBetSanitized.valid ||
        !config.oopRiverRaiseSanitized.valid ||
        (config.donkOption && !config.oopTurnDonkSanitized.valid) ||
        (config.donkOption && !config.oopRiverDonkSanitized.valid)
    );

    const isIpError = computed(
      () =>
        !config.ipFlopBetSanitized.valid ||
        !config.ipFlopRaiseSanitized.valid ||
        !config.ipTurnBetSanitized.valid ||
        !config.ipTurnRaiseSanitized.valid ||
        !config.ipRiverBetSanitized.valid ||
        !config.ipRiverRaiseSanitized.valid
    );

    const isInputValid = computed(
      () =>
        config.startingPot > 0 &&
        config.startingPot <= 100000 &&
        config.startingPot % 1 === 0 &&
        config.effectiveStack > 0 &&
        config.effectiveStack <= 100000 &&
        config.effectiveStack % 1 === 0 &&
        config.rakePercent >= 0 &&
        config.rakePercent <= 100 &&
        config.rakeCap >= 0 &&
        config.rakeCap <= 100000 &&
        !isOopError.value &&
        !isIpError.value &&
        config.addAllInThreshold >= 0 &&
        config.forceAllInThreshold >= 0 &&
        config.mergingThreshold >= 0
    );

    const clearConfig = () => {
      config.startingPot = 0;
      config.effectiveStack = 0;
      config.rakePercent = 0;
      config.rakeCap = 0;
      config.donkOption = false;
      config.oopFlopBet = "";
      config.oopFlopRaise = "";
      config.oopTurnBet = "";
      config.oopTurnRaise = "";
      config.oopTurnDonk = "";
      config.oopRiverBet = "";
      config.oopRiverRaise = "";
      config.oopRiverDonk = "";
      config.ipFlopBet = "";
      config.ipFlopRaise = "";
      config.ipTurnBet = "";
      config.ipTurnRaise = "";
      config.ipRiverBet = "";
      config.ipRiverRaise = "";
      config.addAllInThreshold = 0;
      config.forceAllInThreshold = 0;
      config.mergingThreshold = 0;
      config.expectedBoardLength = 0;
      config.addedLines = "";
      config.removedLines = "";
    };

    const oopToIp = () => {
      config.ipFlopBet = config.oopFlopBet;
      config.ipFlopRaise = config.oopFlopRaise;
      config.ipTurnBet = config.oopTurnBet;
      config.ipTurnRaise = config.oopTurnRaise;
      config.ipRiverBet = config.oopRiverBet;
      config.ipRiverRaise = config.oopRiverRaise;
    };

    const ipToOop = () => {
      config.oopFlopBet = config.ipFlopBet;
      config.oopFlopRaise = config.ipFlopRaise;
      config.oopTurnBet = config.ipTurnBet;
      config.oopTurnRaise = config.ipTurnRaise;
      config.oopRiverBet = config.ipRiverBet;
      config.oopRiverRaise = config.ipRiverRaise;
    };

    const dbValue = computed(
      (): ConfigValue => ({
        startingPot: config.startingPot,
        effectiveStack: config.effectiveStack,
        rakePercent: config.rakePercent,
        rakeCap: config.rakeCap,
        donkOption: Number(config.donkOption),
        oopFlopBet: config.oopFlopBet,
        oopFlopRaise: config.oopFlopRaise,
        oopTurnBet: config.oopTurnBet,
        oopTurnRaise: config.oopTurnRaise,
        oopTurnDonk: config.donkOption ? config.oopTurnDonk : "",
        oopRiverBet: config.oopRiverBet,
        oopRiverRaise: config.oopRiverRaise,
        oopRiverDonk: config.donkOption ? config.oopRiverDonk : "",
        ipFlopBet: config.ipFlopBet,
        ipFlopRaise: config.ipFlopRaise,
        ipTurnBet: config.ipTurnBet,
        ipTurnRaise: config.ipTurnRaise,
        ipRiverBet: config.ipRiverBet,
        ipRiverRaise: config.ipRiverRaise,
        addAllInThreshold: config.addAllInThreshold,
        forceAllInThreshold: config.forceAllInThreshold,
        mergingThreshold: config.mergingThreshold,
        expectedBoardLength: config.expectedBoardLength,
        addedLines: "",
        removedLines: "",
      })
    );

    const loadConfig = (value: unknown) => {
      const configValue = value as ConfigValue;
      config.startingPot = configValue.startingPot;
      config.effectiveStack = configValue.effectiveStack;
      config.rakePercent = configValue.rakePercent;
      config.rakeCap = configValue.rakeCap;
      config.donkOption = Boolean(configValue.donkOption);
      config.oopFlopBet = configValue.oopFlopBet;
      config.oopFlopRaise = configValue.oopFlopRaise;
      config.oopTurnBet = configValue.oopTurnBet;
      config.oopTurnRaise = configValue.oopTurnRaise;
      config.oopTurnDonk = configValue.oopTurnDonk;
      config.oopRiverBet = configValue.oopRiverBet;
      config.oopRiverRaise = configValue.oopRiverRaise;
      config.oopRiverDonk = configValue.oopRiverDonk;
      config.ipFlopBet = configValue.ipFlopBet;
      config.ipFlopRaise = configValue.ipFlopRaise;
      config.ipTurnBet = configValue.ipTurnBet;
      config.ipTurnRaise = configValue.ipTurnRaise;
      config.ipRiverBet = configValue.ipRiverBet;
      config.ipRiverRaise = configValue.ipRiverRaise;
      config.addAllInThreshold = configValue.addAllInThreshold;
      config.forceAllInThreshold = configValue.forceAllInThreshold;
      config.mergingThreshold = configValue.mergingThreshold;
      config.expectedBoardLength = configValue.expectedBoardLength;
      config.addedLines = configValue.addedLines;
      config.removedLines = configValue.removedLines;
    };

    const startEdit = () => {
      isEditMode.value = true;
      if (config.expectedBoardLength === 0) {
        config.expectedBoardLength = Math.max(config.board.length, 3);
      }
    };

    const clearEdit = () => {
      config.expectedBoardLength = 0;
      config.addedLines = "";
      config.removedLines = "";
    };

    const saveEdit = (addedLines: string, removedLines: string) => {
      isEditMode.value = false;
      config.addedLines = addedLines;
      config.removedLines = removedLines;
      if (config.addedLines === "" && config.removedLines === "") {
        config.expectedBoardLength = 0;
      }
    };

    const cancelEdit = () => {
      isEditMode.value = false;
      if (config.addedLines === "" && config.removedLines === "") {
        config.expectedBoardLength = 0;
      }
    };

    return {
      config,
      isEditMode,
      addedLinesArray,
      removedLinesArray,
      hasEdit,
      isOopError,
      isIpError,
      isInputValid,
      clearConfig,
      oopToIp,
      ipToOop,
      dbValue,
      loadConfig,
      startEdit,
      clearEdit,
      saveEdit,
      cancelEdit,
    };
  },
});
</script>

<style scoped>
input {
  @apply disabled:cursor-not-allowed;
}

.button-arrow {
  @apply px-2 py-1 text-lg;
}
</style>
