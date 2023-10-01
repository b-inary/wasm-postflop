<template>
  <div v-if="!isEditMode">
    <div class="flex">
      <div class="shrink-0">
        <div class="flex my-1 gap-8">
          <div>
            <div class="my-1">
              <span class="inline-block w-[7.5rem]">Starting pot:</span>
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
              <span class="inline-block w-[7.5rem]">Effective stack:</span>
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

        <div
          v-if="errorBasics.length > 0"
          class="flex font-semibold text-red-500"
        >
          <div class="underline">Error:</div>
          <div class="ml-2">
            <div v-for="error in errorBasics" :key="error">
              {{ error }}
            </div>
          </div>
        </div>

        <div class="mt-6">
          <div class="flex">
            <span class="font-semibold">OOP bet sizes</span>
            <label class="inline-block ml-6">
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
            <div class="flex flex-grow justify-center">
              <Tippy
                class="inline-block cursor-help text-gray-600"
                max-width="500px"
                trigger="mouseenter click"
                placement="bottom"
                :delay="[200, 0]"
                :interactive="true"
              >
                <QuestionMarkCircleIcon class="inline w-5 h-5" />
                <div class="inline-block ml-0.5 text-sm underline">
                  How to input
                </div>
                <template #content>
                  <div class="px-1 py-0.5 text-justify">
                    Multiple bet sizes can be input, separated by commas or
                    spaces, in any of the following sizes. If left blank, no bet
                    or raise will be made.
                    <ul class="pl-6 list-disc">
                      <li class="mt-1">
                        A number representing a percentage of the pot (e.g.,
                        "50"). For raises, the size is calculated by first
                        calling and then combining a bet of the specified
                        percentage. For example, if the pot is 100 before the
                        bet, and the opponent bets 75, a raise of 50% would
                        result in a bet of 75 + (100 + 75 + 75) * 50% = 200.
                      </li>
                      <li class="mt-1">
                        A multiple of the previous bet size (e.g., "2.5x"). Only
                        valid for raises.
                      </li>
                      <li class="mt-1">All-in (e.g., "a").</li>
                      <li class="mt-1">
                        Constant addition (e.g., "100c"). For raises, you can
                        also specify a cap on the number of raises (e.g.,
                        "20c3r").
                      </li>
                      <li class="mt-1">
                        Geometric size, i.e., dividing the remaining stack into
                        a specified number of equal percentage bets (e.g.,
                        "3e"). For example, if the current pot is 100 and the
                        effective stack is 400, "2e" would result in a bet of
                        100. If the number before the "e" is omitted, the number
                        of remaining streets is used (flop=3, turn=2, river=1).
                        For raises, the number of previous raises is subtracted
                        from the specified number. For example, "3e" will change
                        to "2e" on re-raises. You can also specify a maximum
                        percentage limit by adding a number after the "e" (e.g.,
                        "2e200").
                      </li>
                    </ul>
                  </div>
                </template>
              </Tippy>
            </div>
          </div>

          <div class="flex gap-5">
            <div>
              <div class="my-1 underline">Flop</div>
              <div class="my-1">
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
              </div>
              <div class="my-1">
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
              </div>
            </div>

            <div>
              <div class="my-1 underline">Turn</div>
              <div class="my-1">
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
              </div>
              <div class="my-1">
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
              </div>
              <div v-if="config.donkOption" class="my-1">
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
              </div>
            </div>

            <div>
              <div class="my-1 underline">River</div>
              <div class="my-1">
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
              </div>
              <div class="my-1">
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
              </div>
              <div v-if="config.donkOption" class="my-1">
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
              </div>
            </div>
          </div>
        </div>

        <div>
          <div class="flex">
            <div class="mt-6 font-semibold">IP bet sizes</div>
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
              <div class="my-1 underline">Flop</div>
              <div class="my-1">
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
              </div>
              <div class="my-1">
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
              </div>
            </div>

            <div>
              <div class="my-1 underline">Turn</div>
              <div class="my-1">
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
              </div>
              <div class="my-1">
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
              </div>
            </div>

            <div>
              <div class="my-1 underline">River</div>
              <div class="my-1">
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
              </div>
              <div class="my-1">
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
              </div>
            </div>
          </div>
        </div>

        <div
          v-if="errorOop.length > 0 || errorIp.length > 0"
          class="flex mt-1 font-semibold text-red-500"
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
            <div class="my-1">
              <div class="inline-block w-48">
                Add all-in threshold:
                <Tippy
                  class="inline-block cursor-help"
                  max-width="420px"
                  trigger="mouseenter click"
                  :delay="[200, 0]"
                  :interactive="true"
                >
                  <QuestionMarkCircleIcon class="w-5 h-5 text-gray-600" />
                  <template #content>
                    <div class="px-1 py-0.5 text-justify">
                      Add all-in action to all spots where the ratio of the
                      maximum possible betting size to the pot is less than this
                      threshold.
                    </div>
                  </template>
                </Tippy>
              </div>
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
            </div>

            <div class="my-1">
              <div class="inline-block w-48">
                Force all-in threshold:
                <Tippy
                  class="inline-block cursor-help"
                  max-width="500px"
                  trigger="mouseenter click"
                  :delay="[200, 0]"
                  :interactive="true"
                >
                  <QuestionMarkCircleIcon class="w-5 h-5 text-gray-600" />
                  <template #content>
                    <div class="px-1 py-0.5 text-justify">
                      <div>
                        Replace the betting action with an all-in action if the
                        SPR (Stack/Pot Ratio) is below this threshold after the
                        betting action is called by the opponent. The
                        recommended value is around 15% - 20%.
                      </div>
                      <div class="mt-3">
                        This option is similar to the "all-in threshold" in
                        PioSOLVER. PioSOLVER will replace the betting action
                        with an all-in if the percentage of one's pot commitment
                        to the initial stack exceeds the threshold.
                      </div>
                      <div class="mt-3">
                        The conversion formula, when ignoring decimal rounding,
                        is as follows (s = initial SPR, r = PioSOLVER's
                        threshold):
                      </div>
                      <div class="mt-1 text-center">
                        Threshold = s * (1 - r) / (1 + 2 * s * r).
                      </div>
                    </div>
                  </template>
                </Tippy>
              </div>
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
            </div>

            <div class="my-1">
              <div class="inline-block w-48">
                Merging threshold:
                <Tippy
                  class="inline-block cursor-help"
                  max-width="500px"
                  trigger="mouseenter click"
                  :delay="[200, 0]"
                  :interactive="true"
                >
                  <QuestionMarkCircleIcon class="w-5 h-5 text-gray-600" />
                  <template #content>
                    <div class="px-1 py-0.5 text-justify">
                      <div>
                        Merge betting actions if there are betting actions of
                        similar size.
                      </div>
                      <div class="mt-3">
                        The algorithm is the same as in PioSOLVER. That is,
                        select the largest bet size (= X% of the pot) and remove
                        all other betting actions with sizes (= Y% of the pot)
                        that satisfy the following inequality:
                      </div>
                      <div class="my-1 text-center">
                        (100 + X) / (100 + Y) &lt; 1.0 + Threshold.
                      </div>
                      <div>
                        Repeat this process with the next largest bet size among
                        the remaining bets.
                      </div>
                    </div>
                  </template>
                </Tippy>
              </div>
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
            </div>
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
          class="flex mt-1 font-semibold text-red-500"
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
          class="flex mt-1 font-semibold text-orange-500"
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

      <div class="flex-grow max-w-[18rem] mt-1 ml-6">
        <DbItemPicker
          store-name="configurations"
          :value="dbValue"
          :allow-save="isInputValid"
          @load-item="loadConfig"
        />
      </div>
    </div>

    <div
      v-if="addedLinesArray.length > 0 || removedLinesArray.length > 0"
      class="mt-5"
    >
      <div v-if="addedLinesArray.length > 0" class="flex">
        <div class="font-semibold underline w-[7.75rem]">
          Added line{{ addedLinesArray.length > 1 ? "s" : "" }}:
        </div>
        <div class="flex flex-col">
          <div v-for="addedLine in addedLinesArray" :key="addedLine">
            {{ addedLine }}
          </div>
        </div>
      </div>

      <div v-if="removedLinesArray.length > 0" class="flex mt-2">
        <div class="font-semibold underline w-[7.75rem]">
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
        class="flex mt-2 font-semibold text-orange-500"
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

    <div
      v-if="errorLines.length > 0"
      class="flex mt-2 font-semibold text-red-500"
    >
      <div class="underline">Error:</div>
      <div class="ml-2">
        <div v-for="error in errorLines" :key="error">
          {{ error }}
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
import { useStore, useConfigStore } from "../store";
import {
  MAX_AMOUNT,
  sanitizeBetString,
  ROOT_LINE_STRING,
  INVALID_LINE_STRING,
  readableLineString,
} from "../utils";

import DbItemPicker from "./DbItemPicker.vue";
import TreeEditor from "./TreeEditor.vue";
import { Tippy } from "vue-tippy";
import { QuestionMarkCircleIcon } from "@heroicons/vue/20/solid";

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
    Tippy,
    QuestionMarkCircleIcon,
  },

  setup() {
    const store = useStore();
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
      const isDonk = true;
      const betConfig = [
        { name: "OOP flop bet", res: config.oopFlopBetSanitized },
        { name: "OOP flop raise", res: config.oopFlopRaiseSanitized },
        { name: "OOP turn bet", res: config.oopTurnBetSanitized },
        { name: "OOP turn raise", res: config.oopTurnRaiseSanitized },
        { name: "OOP turn donk", res: config.oopTurnDonkSanitized, isDonk },
        { name: "OOP river bet", res: config.oopRiverBetSanitized },
        { name: "OOP river raise", res: config.oopRiverRaiseSanitized },
        { name: "OOP river donk", res: config.oopRiverDonkSanitized, isDonk },
      ];
      for (const { name, res, isDonk } of betConfig) {
        if (!res.valid && (!isDonk || config.donkOption)) {
          errors.push(`${name}: ${res.s}`);
        }
      }
      return errors;
    });

    const errorIp = computed(() => {
      const errors: string[] = [];
      const betConfig = [
        { name: "IP flop bet", res: config.ipFlopBetSanitized },
        { name: "IP flop raise", res: config.ipFlopRaiseSanitized },
        { name: "IP turn bet", res: config.ipTurnBetSanitized },
        { name: "IP turn raise", res: config.ipTurnRaiseSanitized },
        { name: "IP river bet", res: config.ipRiverBetSanitized },
        { name: "IP river raise", res: config.ipRiverRaiseSanitized },
      ];
      for (const { name, res } of betConfig) {
        if (!res.valid) {
          errors.push(`${name}: ${res.s}`);
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

    const errorLines = computed(() => {
      const errors: string[] = [];
      if (
        addedLinesArray.value.includes(ROOT_LINE_STRING) ||
        addedLinesArray.value.includes(INVALID_LINE_STRING) ||
        removedLinesArray.value.includes(ROOT_LINE_STRING) ||
        removedLinesArray.value.includes(INVALID_LINE_STRING)
      ) {
        errors.push("Invalid line found (loaded broken configurations?)");
      }
      if (
        ![0, 3, 4, 5].includes(config.expectedBoardLength) ||
        (config.expectedBoardLength === 0 &&
          (addedLinesArray.value.length > 0 ||
            removedLinesArray.value.length > 0)) ||
        (config.expectedBoardLength > 0 &&
          addedLinesArray.value.length === 0 &&
          removedLinesArray.value.length === 0)
      ) {
        errors.push("Invalid configurations (loaded broken configurations?)");
      }
      return errors;
    });

    const isInputValid = computed(
      () =>
        errorBasics.value.length === 0 &&
        errorOop.value.length === 0 &&
        errorIp.value.length === 0 &&
        errorMisc.value.length === 0 &&
        errorLines.value.length === 0
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
        addedLines: config.addedLines,
        removedLines: config.removedLines,
      })
    );

    const loadConfig = (value: unknown) => {
      const configValue = value as ConfigValue;
      config.startingPot = Number(configValue.startingPot);
      config.effectiveStack = Number(configValue.effectiveStack);
      config.rakePercent = Number(configValue.rakePercent);
      config.rakeCap = Number(configValue.rakeCap);
      config.donkOption = Boolean(configValue.donkOption);
      config.addAllInThreshold = Number(configValue.addAllInThreshold);
      config.forceAllInThreshold = Number(configValue.forceAllInThreshold);
      config.mergingThreshold = Number(configValue.mergingThreshold);
      config.expectedBoardLength = Number(configValue.expectedBoardLength);
      config.addedLines = String(configValue.addedLines);
      config.removedLines = String(configValue.removedLines);

      const betMembers = [
        "oopFlopBet",
        "oopFlopRaise",
        "oopTurnBet",
        "oopTurnRaise",
        "oopTurnDonk",
        "oopRiverBet",
        "oopRiverRaise",
        "oopRiverDonk",
        "ipFlopBet",
        "ipFlopRaise",
        "ipTurnBet",
        "ipTurnRaise",
        "ipRiverBet",
        "ipRiverRaise",
      ] as const;

      for (const member of betMembers) {
        const str = String(configValue[member]);
        const sanitized = sanitizeBetString(str, member.endsWith("Raise"));
        config[member] = sanitized.valid ? sanitized.s : str;
      }
    };

    const startEdit = () => {
      isEditMode.value = true;
      if (config.expectedBoardLength === 0) {
        config.expectedBoardLength = Math.max(config.board.length, 3);
      }
      store.headers["tree-config"].push("Tree Preview & Edit");
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
      store.headers["tree-config"].pop();
    };

    const cancelEdit = () => {
      isEditMode.value = false;
      if (config.addedLines === "" && config.removedLines === "") {
        config.expectedBoardLength = 0;
      }
      store.headers["tree-config"].pop();
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
      errorLines,
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
