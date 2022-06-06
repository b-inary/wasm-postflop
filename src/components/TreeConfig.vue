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
            v-model="config.oopFlopBetSizesStr"
            type="text"
            :class="
              'w-24 -ml-1 px-2 py-1 rounded-lg text-sm ' +
              (config.oopFlopBetSizes === null ? 'input-error' : '')
            "
          />
          %
        </p>
        <p class="my-1">
          <span class="inline-block w-24">Raise sizes:</span>
          <input
            v-model="config.oopFlopRaiseSizesStr"
            type="text"
            :class="
              'w-24 -ml-1 px-2 py-1 rounded-lg text-sm ' +
              (config.oopFlopRaiseSizes === null ? 'input-error' : '')
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
            v-model="config.oopTurnBetSizesStr"
            type="text"
            :class="
              'w-24 -ml-1 px-2 py-1 rounded-lg text-sm ' +
              (config.oopTurnBetSizes === null ? 'input-error' : '')
            "
          />
          %
        </p>
        <p class="my-1">
          <span class="inline-block w-24">Raise sizes:</span>
          <input
            v-model="config.oopTurnRaiseSizesStr"
            type="text"
            :class="
              'w-24 -ml-1 px-2 py-1 rounded-lg text-sm ' +
              (config.oopTurnRaiseSizes === null ? 'input-error' : '')
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
            v-model="config.oopRiverBetSizesStr"
            type="text"
            :class="
              'w-24 -ml-1 px-2 py-1 rounded-lg text-sm ' +
              (config.oopRiverBetSizes === null ? 'input-error' : '')
            "
          />
          %
        </p>
        <p class="my-1">
          <span class="inline-block w-24">Raise sizes:</span>
          <input
            v-model="config.oopRiverRaiseSizesStr"
            type="text"
            :class="
              'w-24 -ml-1 px-2 py-1 rounded-lg text-sm ' +
              (config.oopRiverRaiseSizes === null ? 'input-error' : '')
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
            v-model="config.ipFlopBetSizesStr"
            type="text"
            :class="
              'w-24 -ml-1 px-2 py-1 rounded-lg text-sm ' +
              (config.ipFlopBetSizes === null ? 'input-error' : '')
            "
          />
          %
        </p>
        <p class="my-1">
          <span class="inline-block w-24">Raise sizes:</span>
          <input
            v-model="config.ipFlopRaiseSizesStr"
            type="text"
            :class="
              'w-24 -ml-1 px-2 py-1 rounded-lg text-sm ' +
              (config.ipFlopRaiseSizes === null ? 'input-error' : '')
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
            v-model="config.ipTurnBetSizesStr"
            type="text"
            :class="
              'w-24 -ml-1 px-2 py-1 rounded-lg text-sm ' +
              (config.ipTurnBetSizes === null ? 'input-error' : '')
            "
          />
          %
        </p>
        <p class="my-1">
          <span class="inline-block w-24">Raise sizes:</span>
          <input
            v-model="config.ipTurnRaiseSizesStr"
            type="text"
            :class="
              'w-24 -ml-1 px-2 py-1 rounded-lg text-sm ' +
              (config.ipTurnRaiseSizes === null ? 'input-error' : '')
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
            v-model="config.ipRiverBetSizesStr"
            type="text"
            :class="
              'w-24 -ml-1 px-2 py-1 rounded-lg text-sm ' +
              (config.ipRiverBetSizes === null ? 'input-error' : '')
            "
          />
          %
        </p>
        <p class="my-1">
          <span class="inline-block w-24">Raise sizes:</span>
          <input
            v-model="config.ipRiverRaiseSizesStr"
            type="text"
            :class="
              'w-24 -ml-1 px-2 py-1 rounded-lg text-sm ' +
              (config.ipRiverRaiseSizes === null ? 'input-error' : '')
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
          (config.addAllInThreshold < 0 || config.addAllInThreshold > 1000
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
          (config.forceAllInThreshold < 0 || config.forceAllInThreshold > 100
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
