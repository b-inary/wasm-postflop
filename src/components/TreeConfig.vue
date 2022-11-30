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
                v-model="config.oopFlopBet"
                type="text"
                :class="
                  'w-[5.5rem] px-2 py-1 rounded-lg text-sm ' +
                  (!config.oopFlopBetSanitized.valid ? 'input-error' : '')
                "
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
                  'w-[5.5rem] px-2 py-1 rounded-lg text-sm ' +
                  (!config.oopFlopRaiseSanitized.valid ? 'input-error' : '')
                "
                @change="
                  config.oopFlopRaiseSanitized.valid &&
                    (config.oopFlopRaise = config.oopFlopRaiseSanitized.s)
                "
              />
              %
            </p>
          </div>

          <div class="ml-5">
            <p class="my-1 underline">Turn</p>
            <p class="my-1">
              <span class="inline-block w-14">Bet:</span>
              <input
                v-model="config.oopTurnBet"
                type="text"
                :class="
                  'w-[5.5rem] px-2 py-1 rounded-lg text-sm ' +
                  (!config.oopTurnBetSanitized.valid ? 'input-error' : '')
                "
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
                  'w-[5.5rem] px-2 py-1 rounded-lg text-sm ' +
                  (!config.oopTurnRaiseSanitized.valid ? 'input-error' : '')
                "
                @change="
                  config.oopTurnRaiseSanitized.valid &&
                    (config.oopTurnRaise = config.oopTurnRaiseSanitized.s)
                "
              />
              %
            </p>
          </div>

          <div class="ml-5">
            <p class="my-1 underline">River</p>
            <p class="my-1">
              <span class="inline-block w-14">Bet:</span>
              <input
                v-model="config.oopRiverBet"
                type="text"
                :class="
                  'w-[5.5rem] px-2 py-1 rounded-lg text-sm ' +
                  (!config.oopRiverBetSanitized.valid ? 'input-error' : '')
                "
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
                  'w-[5.5rem] px-2 py-1 rounded-lg text-sm ' +
                  (!config.oopRiverRaiseSanitized.valid ? 'input-error' : '')
                "
                @change="
                  config.oopRiverRaiseSanitized.valid &&
                    (config.oopRiverRaise = config.oopRiverRaiseSanitized.s)
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
                v-model="config.ipFlopBet"
                type="text"
                :class="
                  'w-[5.5rem] px-2 py-1 rounded-lg text-sm ' +
                  (!config.ipFlopBetSanitized.valid ? 'input-error' : '')
                "
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
                  'w-[5.5rem] px-2 py-1 rounded-lg text-sm ' +
                  (!config.ipFlopRaiseSanitized.valid ? 'input-error' : '')
                "
                @change="
                  config.ipFlopRaiseSanitized.valid &&
                    (config.ipFlopRaise = config.ipFlopRaiseSanitized.s)
                "
              />
              %
            </p>
          </div>

          <div class="ml-5">
            <p class="my-1 underline">Turn</p>
            <p class="my-1">
              <span class="inline-block w-14">Bet:</span>
              <input
                v-model="config.ipTurnBet"
                type="text"
                :class="
                  'w-[5.5rem] px-2 py-1 rounded-lg text-sm ' +
                  (!config.ipTurnBetSanitized.valid ? 'input-error' : '')
                "
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
                  'w-[5.5rem] px-2 py-1 rounded-lg text-sm ' +
                  (!config.ipTurnRaiseSanitized.valid ? 'input-error' : '')
                "
                @change="
                  config.ipTurnRaiseSanitized.valid &&
                    (config.ipTurnRaise = config.ipTurnRaiseSanitized.s)
                "
              />
              %
            </p>
          </div>

          <div class="ml-5">
            <p class="my-1 underline">River</p>
            <p class="my-1">
              <span class="inline-block w-14">Bet:</span>
              <input
                v-model="config.ipRiverBet"
                type="text"
                :class="
                  'w-[5.5rem] px-2 py-1 rounded-lg text-sm ' +
                  (!config.ipRiverBetSanitized.valid ? 'input-error' : '')
                "
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
                  'w-[5.5rem] px-2 py-1 rounded-lg text-sm ' +
                  (!config.ipRiverRaiseSanitized.valid ? 'input-error' : '')
                "
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

      <div class="mt-6">
        <p class="my-1">
          <span class="inline-block w-40">Add all-in threshold:</span>
          <input
            v-model="config.addAllInThreshold"
            type="number"
            :class="
              'w-24 ml-2 px-2 py-1 rounded-lg text-sm text-center ' +
              (config.addAllInThreshold < 0 ? 'input-error' : '')
            "
            min="0"
            max="10000000"
          />
          %
        </p>

        <p class="my-1">
          <span class="inline-block w-40">Force all-in threshold:</span>
          <input
            v-model="config.forceAllInThreshold"
            type="number"
            :class="
              'w-24 ml-2 px-2 py-1 rounded-lg text-sm text-center ' +
              (config.forceAllInThreshold < 0 ? 'input-error' : '')
            "
            min="0"
            max="10000000"
          />
          %
        </p>

        <p class="my-1">
          <span class="inline-block w-40">Merging threshold:</span>
          <input
            v-model="config.mergingThreshold"
            type="number"
            :class="
              'w-24 ml-2 px-2 py-1 rounded-lg text-sm text-center ' +
              (config.mergingThreshold < 0 ? 'input-error' : '')
            "
            min="0"
            max="10000000"
          />
          %
        </p>
      </div>
    </div>

    <db-item-picker
      class="shrink mt-2 ml-6"
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

type ConfigValue = {
  startingPot: number;
  effectiveStack: number;
  oopFlopBet: string;
  oopFlopRaise: string;
  oopTurnBet: string;
  oopTurnRaise: string;
  oopRiverBet: string;
  oopRiverRaise: string;
  ipFlopBet: string;
  ipFlopRaise: string;
  ipTurnBet: string;
  ipTurnRaise: string;
  ipRiverBet: string;
  ipRiverRaise: string;
  addAllInThreshold: number;
  forceAllInThreshold: number;
  mergingThreshold: number;
  addedLines: string[];
  removedLines: string[];
};

export default defineComponent({
  components: {
    DbItemPicker,
  },

  setup() {
    const config = useConfigStore();

    const isOopError = computed(() => {
      return (
        !config.oopFlopBetSanitized.valid ||
        !config.oopFlopRaiseSanitized.valid ||
        !config.oopTurnBetSanitized.valid ||
        !config.oopTurnRaiseSanitized.valid ||
        !config.oopRiverBetSanitized.valid ||
        !config.oopRiverRaiseSanitized.valid
      );
    });

    const isIpError = computed(() => {
      return (
        !config.ipFlopBetSanitized.valid ||
        !config.ipFlopRaiseSanitized.valid ||
        !config.ipTurnBetSanitized.valid ||
        !config.ipTurnRaiseSanitized.valid ||
        !config.ipRiverBetSanitized.valid ||
        !config.ipRiverRaiseSanitized.valid
      );
    });

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
        oopFlopBet: config.oopFlopBet,
        oopFlopRaise: config.oopFlopRaise,
        oopTurnBet: config.oopTurnBet,
        oopTurnRaise: config.oopTurnRaise,
        oopRiverBet: config.oopRiverBet,
        oopRiverRaise: config.oopRiverRaise,
        ipFlopBet: config.ipFlopBet,
        ipFlopRaise: config.ipFlopRaise,
        ipTurnBet: config.ipTurnBet,
        ipTurnRaise: config.ipTurnRaise,
        ipRiverBet: config.ipRiverBet,
        ipRiverRaise: config.ipRiverRaise,
        addAllInThreshold: config.addAllInThreshold,
        forceAllInThreshold: config.forceAllInThreshold,
        mergingThreshold: config.mergingThreshold,
        addedLines: [],
        removedLines: [],
      })
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
        config.forceAllInThreshold >= 0 &&
        config.mergingThreshold >= 0
    );

    const loadConfig = (value: ConfigValue) => {
      config.startingPot = value.startingPot;
      config.effectiveStack = value.effectiveStack;
      config.oopFlopBet = value.oopFlopBet;
      config.oopFlopRaise = value.oopFlopRaise;
      config.oopTurnBet = value.oopTurnBet;
      config.oopTurnRaise = value.oopTurnRaise;
      config.oopRiverBet = value.oopRiverBet;
      config.oopRiverRaise = value.oopRiverRaise;
      config.ipFlopBet = value.ipFlopBet;
      config.ipFlopRaise = value.ipFlopRaise;
      config.ipTurnBet = value.ipTurnBet;
      config.ipTurnRaise = value.ipTurnRaise;
      config.ipRiverBet = value.ipRiverBet;
      config.ipRiverRaise = value.ipRiverRaise;
      config.addAllInThreshold = value.addAllInThreshold;
      config.forceAllInThreshold = value.forceAllInThreshold;
      config.mergingThreshold = value.mergingThreshold;
    };

    return {
      config,
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
