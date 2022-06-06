<template>
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
    <p class="font-bold">OOP settings</p>
    <div class="flex flex-row">
      <div>
        <p class="my-1 underline">Flop</p>
        <p class="my-1">
          <span class="inline-block w-24">Bet sizes:</span>
          <input
            v-model="config.oopFlopBetStr"
            type="text"
            :class="
              'w-24 -ml-1 px-2 py-1 rounded-lg text-sm ' +
              (config.oopFlopBet === null ? 'input-error' : '')
            "
          />
          %
        </p>
        <p class="my-1">
          <span class="inline-block w-24">Raise sizes:</span>
          <input
            v-model="config.oopFlopRaiseStr"
            type="text"
            :class="
              'w-24 -ml-1 px-2 py-1 rounded-lg text-sm ' +
              (config.oopFlopRaise === null ? 'input-error' : '')
            "
          />
          %
        </p>
      </div>

      <div class="ml-6">
        <p class="my-1 underline">Turn</p>
        <p class="my-1">
          <span class="inline-block w-24">Bet sizes:</span>
          <input
            v-model="config.oopTurnBetStr"
            type="text"
            :class="
              'w-24 -ml-1 px-2 py-1 rounded-lg text-sm ' +
              (config.oopTurnBet === null ? 'input-error' : '')
            "
          />
          %
        </p>
        <p class="my-1">
          <span class="inline-block w-24">Raise sizes:</span>
          <input
            v-model="config.oopTurnRaiseStr"
            type="text"
            :class="
              'w-24 -ml-1 px-2 py-1 rounded-lg text-sm ' +
              (config.oopTurnRaise === null ? 'input-error' : '')
            "
          />
          %
        </p>
      </div>

      <div class="ml-6">
        <p class="my-1 underline">River</p>
        <p class="my-1">
          <span class="inline-block w-24">Bet sizes:</span>
          <input
            v-model="config.oopRiverBetStr"
            type="text"
            :class="
              'w-24 -ml-1 px-2 py-1 rounded-lg text-sm ' +
              (config.oopRiverBet === null ? 'input-error' : '')
            "
          />
          %
        </p>
        <p class="my-1">
          <span class="inline-block w-24">Raise sizes:</span>
          <input
            v-model="config.oopRiverRaiseStr"
            type="text"
            :class="
              'w-24 -ml-1 px-2 py-1 rounded-lg text-sm ' +
              (config.oopRiverRaise === null ? 'input-error' : '')
            "
          />
          %
        </p>
      </div>
    </div>
  </div>

  <div class="mt-6">
    <p class="font-bold">IP settings</p>
    <div class="flex flex-row">
      <div>
        <p class="my-1 underline">Flop</p>
        <p class="my-1">
          <span class="inline-block w-24">Bet sizes:</span>
          <input
            v-model="config.ipFlopBetStr"
            type="text"
            :class="
              'w-24 -ml-1 px-2 py-1 rounded-lg text-sm ' +
              (config.ipFlopBet === null ? 'input-error' : '')
            "
          />
          %
        </p>
        <p class="my-1">
          <span class="inline-block w-24">Raise sizes:</span>
          <input
            v-model="config.ipFlopRaiseStr"
            type="text"
            :class="
              'w-24 -ml-1 px-2 py-1 rounded-lg text-sm ' +
              (config.ipFlopRaise === null ? 'input-error' : '')
            "
          />
          %
        </p>
      </div>

      <div class="ml-6">
        <p class="my-1 underline">Turn</p>
        <p class="my-1">
          <span class="inline-block w-24">Bet sizes:</span>
          <input
            v-model="config.ipTurnBetStr"
            type="text"
            :class="
              'w-24 -ml-1 px-2 py-1 rounded-lg text-sm ' +
              (config.ipTurnBet === null ? 'input-error' : '')
            "
          />
          %
        </p>
        <p class="my-1">
          <span class="inline-block w-24">Raise sizes:</span>
          <input
            v-model="config.ipTurnRaiseStr"
            type="text"
            :class="
              'w-24 -ml-1 px-2 py-1 rounded-lg text-sm ' +
              (config.ipTurnRaise === null ? 'input-error' : '')
            "
          />
          %
        </p>
      </div>

      <div class="ml-6">
        <p class="my-1 underline">River</p>
        <p class="my-1">
          <span class="inline-block w-24">Bet sizes:</span>
          <input
            v-model="config.ipRiverBetStr"
            type="text"
            :class="
              'w-24 -ml-1 px-2 py-1 rounded-lg text-sm ' +
              (config.ipRiverBet === null ? 'input-error' : '')
            "
          />
          %
        </p>
        <p class="my-1">
          <span class="inline-block w-24">Raise sizes:</span>
          <input
            v-model="config.ipRiverRaiseStr"
            type="text"
            :class="
              'w-24 -ml-1 px-2 py-1 rounded-lg text-sm ' +
              (config.ipRiverRaise === null ? 'input-error' : '')
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
          (config.forceAllInThreshold < 0 || config.forceAllInThreshold > 10000
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
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { useConfigStore } from "../store";

export default defineComponent({
  setup() {
    return { config: useConfigStore() };
  },
});
</script>

<style>
input.input-error {
  @apply ring-1 ring-red-600 border-red-600 bg-red-50;
}
</style>
