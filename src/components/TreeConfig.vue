<template>
  <div class="flex">
    <div class="shrink-0">
      <div>
        <p class="my-1">
          <span class="inline-block w-[7.5rem]">Starting pot:</span>
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
            min="0"
            max="100000"
          />
        </p>

        <p class="my-1">
          <span class="inline-block w-[7.5rem]">Effective stack:</span>
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
            min="0"
            max="100000"
          />
        </p>
      </div>

      <div class="mt-6">
        <p class="font-bold">OOP bet sizes</p>
        <div class="flex flex-row">
          <div>
            <p class="my-1 underline">Flop</p>
            <p class="my-1">
              <span class="inline-block w-14">Bet:</span>
              <input
                v-model="config.oopFlopBetStr"
                type="text"
                :class="
                  'w-[5.25rem] px-2 py-1 rounded-lg text-sm ' +
                  (config.oopFlopBet === null ? 'input-error' : '')
                "
                @change="
                  config.oopFlopBet &&
                    (config.oopFlopBetStr = sanitize(config.oopFlopBetStr))
                "
              />
              %
            </p>
            <p class="my-1">
              <span class="inline-block w-14">Raise:</span>
              <input
                v-model="config.oopFlopRaiseStr"
                type="text"
                :class="
                  'w-[5.25rem] px-2 py-1 rounded-lg text-sm ' +
                  (config.oopFlopRaise === null ? 'input-error' : '')
                "
                @change="
                  config.oopFlopRaise &&
                    (config.oopFlopRaiseStr = sanitize(config.oopFlopRaiseStr))
                "
              />
              %
            </p>
          </div>

          <div class="ml-6">
            <p class="my-1 underline">Turn</p>
            <p class="my-1">
              <span class="inline-block w-14">Bet:</span>
              <input
                v-model="config.oopTurnBetStr"
                type="text"
                :class="
                  'w-[5.25rem] px-2 py-1 rounded-lg text-sm ' +
                  (config.oopTurnBet === null ? 'input-error' : '')
                "
                @change="
                  config.oopTurnBet &&
                    (config.oopTurnBetStr = sanitize(config.oopTurnBetStr))
                "
              />
              %
            </p>
            <p class="my-1">
              <span class="inline-block w-14">Raise:</span>
              <input
                v-model="config.oopTurnRaiseStr"
                type="text"
                :class="
                  'w-[5.25rem] px-2 py-1 rounded-lg text-sm ' +
                  (config.oopTurnRaise === null ? 'input-error' : '')
                "
                @change="
                  config.oopTurnRaise &&
                    (config.oopTurnRaiseStr = sanitize(config.oopTurnRaiseStr))
                "
              />
              %
            </p>
          </div>

          <div class="ml-6">
            <p class="my-1 underline">River</p>
            <p class="my-1">
              <span class="inline-block w-14">Bet:</span>
              <input
                v-model="config.oopRiverBetStr"
                type="text"
                :class="
                  'w-[5.25rem] px-2 py-1 rounded-lg text-sm ' +
                  (config.oopRiverBet === null ? 'input-error' : '')
                "
                @change="
                  config.oopRiverBet &&
                    (config.oopRiverBetStr = sanitize(config.oopRiverBetStr))
                "
              />
              %
            </p>
            <p class="my-1">
              <span class="inline-block w-14">Raise:</span>
              <input
                v-model="config.oopRiverRaiseStr"
                type="text"
                :class="
                  'w-[5.25rem] px-2 py-1 rounded-lg text-sm ' +
                  (config.oopRiverRaise === null ? 'input-error' : '')
                "
                @change="
                  config.oopRiverRaise &&
                    (config.oopRiverRaiseStr = sanitize(
                      config.oopRiverRaiseStr
                    ))
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
              class="mt-3 button-blue"
              :disabled="isIpError"
              @click="ipToOop"
            >
              ↑
            </button>
            <button
              class="mt-3 button-blue"
              :disabled="isOopError"
              @click="oopToIp"
            >
              ↓
            </button>
          </div>
        </div>

        <div class="flex flex-row">
          <div>
            <p class="my-1 underline">Flop</p>
            <p class="my-1">
              <span class="inline-block w-14">Bet:</span>
              <input
                v-model="config.ipFlopBetStr"
                type="text"
                :class="
                  'w-[5.25rem] px-2 py-1 rounded-lg text-sm ' +
                  (config.ipFlopBet === null ? 'input-error' : '')
                "
                @change="
                  config.ipFlopBet &&
                    (config.ipFlopBetStr = sanitize(config.ipFlopBetStr))
                "
              />
              %
            </p>
            <p class="my-1">
              <span class="inline-block w-14">Raise:</span>
              <input
                v-model="config.ipFlopRaiseStr"
                type="text"
                :class="
                  'w-[5.25rem] px-2 py-1 rounded-lg text-sm ' +
                  (config.ipFlopRaise === null ? 'input-error' : '')
                "
                @change="
                  config.ipFlopRaise &&
                    (config.ipFlopRaiseStr = sanitize(config.ipFlopRaiseStr))
                "
              />
              %
            </p>
          </div>

          <div class="ml-6">
            <p class="my-1 underline">Turn</p>
            <p class="my-1">
              <span class="inline-block w-14">Bet:</span>
              <input
                v-model="config.ipTurnBetStr"
                type="text"
                :class="
                  'w-[5.25rem] px-2 py-1 rounded-lg text-sm ' +
                  (config.ipTurnBet === null ? 'input-error' : '')
                "
                @change="
                  config.ipTurnBet &&
                    (config.ipTurnBetStr = sanitize(config.ipTurnBetStr))
                "
              />
              %
            </p>
            <p class="my-1">
              <span class="inline-block w-14">Raise:</span>
              <input
                v-model="config.ipTurnRaiseStr"
                type="text"
                :class="
                  'w-[5.25rem] px-2 py-1 rounded-lg text-sm ' +
                  (config.ipTurnRaise === null ? 'input-error' : '')
                "
                @change="
                  config.ipTurnRaise &&
                    (config.ipTurnRaiseStr = sanitize(config.ipTurnRaiseStr))
                "
              />
              %
            </p>
          </div>

          <div class="ml-6">
            <p class="my-1 underline">River</p>
            <p class="my-1">
              <span class="inline-block w-14">Bet:</span>
              <input
                v-model="config.ipRiverBetStr"
                type="text"
                :class="
                  'w-[5.25rem] px-2 py-1 rounded-lg text-sm ' +
                  (config.ipRiverBet === null ? 'input-error' : '')
                "
                @change="
                  config.ipRiverBet &&
                    (config.ipRiverBetStr = sanitize(config.ipRiverBetStr))
                "
              />
              %
            </p>
            <p class="my-1">
              <span class="inline-block w-14">Raise:</span>
              <input
                v-model="config.ipRiverRaiseStr"
                type="text"
                :class="
                  'w-[5.25rem] px-2 py-1 rounded-lg text-sm ' +
                  (config.ipRiverRaise === null ? 'input-error' : '')
                "
                @change="
                  config.ipRiverRaise &&
                    (config.ipRiverRaiseStr = sanitize(config.ipRiverRaiseStr))
                "
              />
              %
            </p>
          </div>
        </div>
      </div>

      <div class="mt-6">
        <p class="my-1">
          Add all-in if effective stack is less than:
          <input
            v-model="config.addAllInThreshold"
            type="number"
            :class="
              'w-20 ml-2 px-2 py-1 rounded-lg text-sm text-center ' +
              (config.addAllInThreshold < 0 || config.addAllInThreshold > 10000
                ? 'input-error'
                : '')
            "
            min="0"
            max="1000"
          />
          % of pot
        </p>

        <p class="my-1">
          Force all-in if next opponent's raise size is less than:
          <input
            v-model="config.forceAllInThreshold"
            type="number"
            :class="
              'w-20 ml-2 px-2 py-1 rounded-lg text-sm text-center ' +
              (config.forceAllInThreshold < 0 ||
              config.forceAllInThreshold > 10000
                ? 'input-error'
                : '')
            "
            min="0"
            max="100"
          />
          % of pot
        </p>

        <p class="my-1">
          <label class="cursor-pointer">
            <input
              v-model="config.adjustLastTwoBetSizes"
              type="checkbox"
              class="mr-1 align-middle rounded cursor-pointer"
            />
            Adjust last two bet sizes
          </label>
        </p>
      </div>
    </div>

    <db-item-picker
      class="shrink mt-2 ml-7"
      store-name="configurations"
      :value="dbValue"
      :allow-save="allowSave"
      @load-item="loadConfig"
    />
  </div>
</template>

<script lang="ts">
import { computed, defineComponent } from "vue";
import { useConfigStore } from "../store";

import DbItemPicker from "./DbItemPicker.vue";

const whitespacesBeforeCommaPat = /\s+,/;
const whitespacesPat = /\s+/;

type ConfigValue = {
  startingPot: number;
  effectiveStack: number;
  oopFlopBetStr: string;
  oopFlopRaiseStr: string;
  oopTurnBetStr: string;
  oopTurnRaiseStr: string;
  oopRiverBetStr: string;
  oopRiverRaiseStr: string;
  ipFlopBetStr: string;
  ipFlopRaiseStr: string;
  ipTurnBetStr: string;
  ipTurnRaiseStr: string;
  ipRiverBetStr: string;
  ipRiverRaiseStr: string;
  addAllInThreshold: number;
  forceAllInThreshold: number;
  adjustLastTwoBetSizes: number;
};

export default defineComponent({
  components: {
    DbItemPicker,
  },

  setup() {
    const config = useConfigStore();

    const sanitize = (str: string) => {
      return str
        .trim()
        .replace(whitespacesBeforeCommaPat, ",")
        .replace(whitespacesPat, " ");
    };

    const isOopError = computed(() => {
      return (
        config.oopFlopBet === null ||
        config.oopFlopRaise === null ||
        config.oopTurnBet === null ||
        config.oopTurnRaise === null ||
        config.oopRiverBet === null ||
        config.oopRiverRaise === null
      );
    });

    const isIpError = computed(() => {
      return (
        config.ipFlopBet === null ||
        config.ipFlopRaise === null ||
        config.ipTurnBet === null ||
        config.ipTurnRaise === null ||
        config.ipRiverBet === null ||
        config.ipRiverRaise === null
      );
    });

    const oopToIp = () => {
      config.ipFlopBetStr = config.oopFlopBetStr;
      config.ipFlopRaiseStr = config.oopFlopRaiseStr;
      config.ipTurnBetStr = config.oopTurnBetStr;
      config.ipTurnRaiseStr = config.oopTurnRaiseStr;
      config.ipRiverBetStr = config.oopRiverBetStr;
      config.ipRiverRaiseStr = config.oopRiverRaiseStr;
    };

    const ipToOop = () => {
      config.oopFlopBetStr = config.ipFlopBetStr;
      config.oopFlopRaiseStr = config.ipFlopRaiseStr;
      config.oopTurnBetStr = config.ipTurnBetStr;
      config.oopTurnRaiseStr = config.ipTurnRaiseStr;
      config.oopRiverBetStr = config.ipRiverBetStr;
      config.oopRiverRaiseStr = config.ipRiverRaiseStr;
    };

    const dbValue = computed(
      () =>
        ({
          startingPot: config.startingPot,
          effectiveStack: config.effectiveStack,
          oopFlopBetStr: config.oopFlopBetStr,
          oopFlopRaiseStr: config.oopFlopRaiseStr,
          oopTurnBetStr: config.oopTurnBetStr,
          oopTurnRaiseStr: config.oopTurnRaiseStr,
          oopRiverBetStr: config.oopRiverBetStr,
          oopRiverRaiseStr: config.oopRiverRaiseStr,
          ipFlopBetStr: config.ipFlopBetStr,
          ipFlopRaiseStr: config.ipFlopRaiseStr,
          ipTurnBetStr: config.ipTurnBetStr,
          ipTurnRaiseStr: config.ipTurnRaiseStr,
          ipRiverBetStr: config.ipRiverBetStr,
          ipRiverRaiseStr: config.ipRiverRaiseStr,
          addAllInThreshold: config.addAllInThreshold,
          forceAllInThreshold: config.forceAllInThreshold,
          adjustLastTwoBetSizes: Number(config.adjustLastTwoBetSizes),
        } as ConfigValue)
    );

    const allowSave = computed(
      () =>
        config.startingPot > 0 &&
        config.startingPot <= 100000 &&
        config.startingPot % 1 === 0 &&
        config.effectiveStack > 0 &&
        config.effectiveStack <= 100000 &&
        config.effectiveStack % 1 === 0 &&
        !isOopError.value &&
        !isIpError.value &&
        config.addAllInThreshold >= 0 &&
        config.addAllInThreshold <= 10000 &&
        config.forceAllInThreshold >= 0 &&
        config.forceAllInThreshold <= 10000
    );

    const loadConfig = (value: ConfigValue) => {
      config.startingPot = value.startingPot;
      config.effectiveStack = value.effectiveStack;
      config.oopFlopBetStr = value.oopFlopBetStr;
      config.oopFlopRaiseStr = value.oopFlopRaiseStr;
      config.oopTurnBetStr = value.oopTurnBetStr;
      config.oopTurnRaiseStr = value.oopTurnRaiseStr;
      config.oopRiverBetStr = value.oopRiverBetStr;
      config.oopRiverRaiseStr = value.oopRiverRaiseStr;
      config.ipFlopBetStr = value.ipFlopBetStr;
      config.ipFlopRaiseStr = value.ipFlopRaiseStr;
      config.ipTurnBetStr = value.ipTurnBetStr;
      config.ipTurnRaiseStr = value.ipTurnRaiseStr;
      config.ipRiverBetStr = value.ipRiverBetStr;
      config.ipRiverRaiseStr = value.ipRiverRaiseStr;
      config.addAllInThreshold = value.addAllInThreshold;
      config.forceAllInThreshold = value.forceAllInThreshold;
      config.adjustLastTwoBetSizes = Boolean(value.adjustLastTwoBetSizes);
    };

    return {
      config,
      sanitize,
      isOopError,
      isIpError,
      oopToIp,
      ipToOop,
      dbValue,
      allowSave,
      loadConfig,
    };
  },
});
</script>

<style scoped>
input.input-error {
  @apply ring-1 ring-red-600 border-red-600 bg-red-50;
}

.button-blue {
  @apply rounded-lg shadow-sm px-2 py-1 text-white text-lg;
  @apply bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-4 focus:ring-blue-300;
  @apply disabled:opacity-40 disabled:bg-blue-600;
}
</style>
