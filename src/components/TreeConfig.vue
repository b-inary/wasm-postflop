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
                  config.startingPot > MAX_AMOUNT ||
                  config.startingPot % 1 !== 0
                    ? 'input-error'
                    : '')
                "
                :disabled="hasEdit"
                min="0"
                :max="MAX_AMOUNT"
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
                  config.effectiveStack > MAX_AMOUNT ||
                  config.effectiveStack % 1 !== 0
                    ? 'input-error'
                    : '')
                "
                :disabled="hasEdit"
                min="0"
                :max="MAX_AMOUNT"
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
                  (config.rakeCap < 0 || config.rakeCap > 3 * MAX_AMOUNT
                    ? 'input-error'
                    : '')
                "
                :disabled="hasEdit"
                min="0"
                :max="3 * MAX_AMOUNT"
              />
            </div>
          </div>

          <div class="ml-auto p-1">
            <button class="button-base button-blue" @click="clearConfig">
              Clear
            </button>
          </div>
        </div>

        <div v-if="errorBasics.length > 0" class="flex font-bold text-red-500">
          <div class="underline">Error:</div>
          <div class="ml-2">
            <div v-for="error in errorBasics" :key="error">
              {{ error }}
            </div>
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
                :disabled="errorIp.length > 0 || hasEdit"
                @click="ipToOop"
              >
                ↑
              </button>
              <button
                class="mt-3 button-base button-blue button-arrow"
                :disabled="errorOop.length > 0 || hasEdit"
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

        <div
          v-if="errorOop.length > 0 || errorIp.length > 0"
          class="flex mt-1 font-bold text-red-500"
        >
          <div class="underline">Error:</div>
          <div class="ml-2">
            <div v-for="error in errorOop" :key="error">
              {{ error }}
            </div>
            <div v-for="error in errorIp" :key="error">
              {{ error }}
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
                :max="MAX_AMOUNT * 100"
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
                :max="MAX_AMOUNT * 100"
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
                :max="MAX_AMOUNT * 100"
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

        <div
          v-if="errorMisc.length > 0"
          class="flex mt-1 font-bold text-red-500"
        >
          <div class="underline">Error:</div>
          <div class="ml-2">
            <div v-for="error in errorMisc" :key="error">
              {{ error }}
            </div>
          </div>
        </div>

        <div
          v-if="warningMisc.length > 0"
          class="flex mt-1 font-bold text-orange-500"
        >
          <div class="underline">Warning:</div>
          <div class="ml-2">
            <div
              v-for="warning in warningMisc"
              :key="warning"
              class="whitespace-pre-line"
            >
              {{ warning }}
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
      <div v-if="addedLinesArray.length > 0" class="flex">
        <div class="font-bold underline w-[7.75rem]">
          Added line{{ addedLinesArray.length > 1 ? "s" : "" }}:
        </div>
        <div class="flex flex-col">
          <div v-for="addedLine in addedLinesArray" :key="addedLine">
            {{ addedLine }}
          </div>
        </div>
      </div>

      <div v-if="removedLinesArray.length > 0" class="flex mt-2">
        <div class="font-bold underline w-[7.75rem]">
          Removed line{{ removedLinesArray.length > 1 ? "s" : "" }}:
        </div>
        <div class="flex flex-col">
          <div v-for="removedLine in removedLinesArray" :key="removedLine">
            {{ removedLine }}
          </div>
        </div>
      </div>

      <div
        v-if="
          config.board.length >= 3 &&
          config.expectedBoardLength > 0 &&
          config.board.length !== config.expectedBoardLength
        "
        class="flex mt-2 font-bold text-orange-500"
      >
        <div class="underline">Warning:</div>
        <div class="ml-2">
          The edited tree assumes a {{ config.expectedBoardLength }}-card board,
          but the current board consists of {{ config.board.length }} cards.
          <br />
          To reset the edited tree, click the "Clear Edit & Unlock" button.
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
import { MAX_AMOUNT, readableLineString } from "../utils";

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

    const errorBasics = computed(() => {
      const errors: string[] = [];
      if (config.startingPot <= 0) {
        errors.push("Starting pot must be positive");
      }
      if (config.startingPot > MAX_AMOUNT) {
        errors.push(`Starting pot must not exceed ${MAX_AMOUNT}`);
      }
      if (config.startingPot % 1 !== 0) {
        errors.push("Starting pot must be an integer");
      }
      if (config.effectiveStack <= 0) {
        errors.push("Effective stack must be positive");
      }
      if (config.effectiveStack > MAX_AMOUNT) {
        errors.push(`Effective stack must not exceed ${MAX_AMOUNT}`);
      }
      if (config.effectiveStack % 1 !== 0) {
        errors.push("Effective stack must be an integer");
      }
      if (config.rakePercent < 0) {
        errors.push("Rake must be non-negative");
      }
      if (config.rakePercent > 100) {
        errors.push("Rake must not exceed 100%");
      }
      if (config.rakeCap < 0) {
        errors.push("Rake cap must be non-negative");
      }
      if (config.rakeCap > 3 * MAX_AMOUNT) {
        errors.push(`Rake cap must not exceed ${3 * MAX_AMOUNT}`);
      }
      return errors;
    });

    const errorOop = computed(() => {
      const errors: string[] = [];
      const betConfig = [
        { name: "OOP flop bet", value: config.oopFlopBetSanitized },
        { name: "OOP flop raise", value: config.oopFlopRaiseSanitized },
        { name: "OOP turn bet", value: config.oopTurnBetSanitized },
        { name: "OOP turn raise", value: config.oopTurnRaiseSanitized },
        { name: "OOP turn donk", value: config.oopTurnDonkSanitized },
        { name: "OOP river bet", value: config.oopRiverBetSanitized },
        { name: "OOP river raise", value: config.oopRiverRaiseSanitized },
        { name: "OOP river donk", value: config.oopRiverDonkSanitized },
      ];
      for (const { name, value } of betConfig) {
        if (!value.valid) {
          errors.push(`${name}: ${value.s}`);
        }
      }
      return errors;
    });

    const errorIp = computed(() => {
      const errors: string[] = [];
      const betConfig = [
        { name: "IP flop bet", value: config.ipFlopBetSanitized },
        { name: "IP flop raise", value: config.ipFlopRaiseSanitized },
        { name: "IP turn bet", value: config.ipTurnBetSanitized },
        { name: "IP turn raise", value: config.ipTurnRaiseSanitized },
        { name: "IP river bet", value: config.ipRiverBetSanitized },
        { name: "IP river raise", value: config.ipRiverRaiseSanitized },
      ];
      for (const { name, value } of betConfig) {
        if (!value.valid) {
          errors.push(`${name}: ${value.s}`);
        }
      }
      return errors;
    });

    const errorMisc = computed(() => {
      const errors: string[] = [];
      if (config.addAllInThreshold < 0) {
        errors.push("Add all-in threshold must be non-negative");
      }
      if (config.forceAllInThreshold < 0) {
        errors.push("Force all-in threshold must be non-negative");
      }
      if (config.mergingThreshold < 0) {
        errors.push("Merging threshold must be non-negative");
      }
      return errors;
    });

    const warningMisc = computed(() => {
      const warnings: string[] = [];
      if (config.forceAllInThreshold > 30) {
        warnings.push(
          "Force all-in threshold higher than 30% is not recommended.\nPlease see help to confirm the meaning."
        );
      }
      return warnings;
    });

    const isInputValid = computed(
      () =>
        errorBasics.value.length == 0 &&
        errorOop.value.length == 0 &&
        errorIp.value.length == 0 &&
        errorMisc.value.length == 0
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
      MAX_AMOUNT,
      config,
      isEditMode,
      addedLinesArray,
      removedLinesArray,
      hasEdit,
      errorBasics,
      errorOop,
      errorIp,
      errorMisc,
      warningMisc,
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
