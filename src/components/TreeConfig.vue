<template>
  <div>
    <p class="my-1">
      <span class="inline-block w-28">Starting pot:</span>
      <input
        v-model="store.startingPot"
        type="number"
        :class="
          'w-24 ml-2 px-2 py-1 rounded-lg text-sm text-center ' +
          (store.startingPot <= 0 ||
          store.startingPot > 100000 ||
          store.startingPot % 1 !== 0
            ? 'input-error'
            : '')
        "
        min="0"
        max="100000"
      />
    </p>

    <p class="my-1">
      <span class="inline-block w-28">Effective stack:</span>
      <input
        v-model="store.effectiveStack"
        type="number"
        :class="
          'w-24 ml-2 px-2 py-1 rounded-lg text-sm text-center ' +
          (store.effectiveStack <= 0 ||
          store.effectiveStack > 100000 ||
          store.effectiveStack % 1 !== 0
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
          <span class="inline-block w-20">Bet sizes:</span>
          <input
            v-model="store.oopFlopBetSizesStr"
            type="text"
            :class="
              'w-24 ml-2 px-2 py-1 rounded-lg text-sm ' +
              (store.oopFlopBetSizes === null ? 'input-error' : '')
            "
          />
          %
        </p>
        <p class="my-1">
          <span class="inline-block w-20">Raise sizes:</span>
          <input
            v-model="store.oopFlopRaiseSizesStr"
            type="text"
            :class="
              'w-24 ml-2 px-2 py-1 rounded-lg text-sm ' +
              (store.oopFlopRaiseSizes === null ? 'input-error' : '')
            "
          />
          %
        </p>
      </div>

      <div class="ml-6">
        <p class="my-1 underline">Turn</p>
        <p class="my-1">
          <span class="inline-block w-20">Bet sizes:</span>
          <input
            v-model="store.oopTurnBetSizesStr"
            type="text"
            :class="
              'w-24 ml-2 px-2 py-1 rounded-lg text-sm ' +
              (store.oopTurnBetSizes === null ? 'input-error' : '')
            "
          />
          %
        </p>
        <p class="my-1">
          <span class="inline-block w-20">Raise sizes:</span>
          <input
            v-model="store.oopTurnRaiseSizesStr"
            type="text"
            :class="
              'w-24 ml-2 px-2 py-1 rounded-lg text-sm ' +
              (store.oopTurnRaiseSizes === null ? 'input-error' : '')
            "
          />
          %
        </p>
      </div>

      <div class="ml-6">
        <p class="my-1 underline">River</p>
        <p class="my-1">
          <span class="inline-block w-20">Bet sizes:</span>
          <input
            v-model="store.oopRiverBetSizesStr"
            type="text"
            :class="
              'w-24 ml-2 px-2 py-1 rounded-lg text-sm ' +
              (store.oopRiverBetSizes === null ? 'input-error' : '')
            "
          />
          %
        </p>
        <p class="my-1">
          <span class="inline-block w-20">Raise sizes:</span>
          <input
            v-model="store.oopRiverRaiseSizesStr"
            type="text"
            :class="
              'w-24 ml-2 px-2 py-1 rounded-lg text-sm ' +
              (store.oopRiverRaiseSizes === null ? 'input-error' : '')
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
          <span class="inline-block w-20">Bet sizes:</span>
          <input
            v-model="store.ipFlopBetSizesStr"
            type="text"
            :class="
              'w-24 ml-2 px-2 py-1 rounded-lg text-sm ' +
              (store.ipFlopBetSizes === null ? 'input-error' : '')
            "
          />
          %
        </p>
        <p class="my-1">
          <span class="inline-block w-20">Raise sizes:</span>
          <input
            v-model="store.ipFlopRaiseSizesStr"
            type="text"
            :class="
              'w-24 ml-2 px-2 py-1 rounded-lg text-sm ' +
              (store.ipFlopRaiseSizes === null ? 'input-error' : '')
            "
          />
          %
        </p>
      </div>

      <div class="ml-6">
        <p class="my-1 underline">Turn</p>
        <p class="my-1">
          <span class="inline-block w-20">Bet sizes:</span>
          <input
            v-model="store.ipTurnBetSizesStr"
            type="text"
            :class="
              'w-24 ml-2 px-2 py-1 rounded-lg text-sm ' +
              (store.ipTurnBetSizes === null ? 'input-error' : '')
            "
          />
          %
        </p>
        <p class="my-1">
          <span class="inline-block w-20">Raise sizes:</span>
          <input
            v-model="store.ipTurnRaiseSizesStr"
            type="text"
            :class="
              'w-24 ml-2 px-2 py-1 rounded-lg text-sm ' +
              (store.ipTurnRaiseSizes === null ? 'input-error' : '')
            "
          />
          %
        </p>
      </div>

      <div class="ml-6">
        <p class="my-1 underline">River</p>
        <p class="my-1">
          <span class="inline-block w-20">Bet sizes:</span>
          <input
            v-model="store.ipRiverBetSizesStr"
            type="text"
            :class="
              'w-24 ml-2 px-2 py-1 rounded-lg text-sm ' +
              (store.ipRiverBetSizes === null ? 'input-error' : '')
            "
          />
          %
        </p>
        <p class="my-1">
          <span class="inline-block w-20">Raise sizes:</span>
          <input
            v-model="store.ipRiverRaiseSizesStr"
            type="text"
            :class="
              'w-24 ml-2 px-2 py-1 rounded-lg text-sm ' +
              (store.ipRiverRaiseSizes === null ? 'input-error' : '')
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
        v-model="store.addAllInThreshold"
        type="number"
        :class="
          'w-20 ml-2 px-2 py-1 rounded-lg text-sm text-center ' +
          (store.addAllInThreshold < 0 || store.addAllInThreshold > 1000
            ? 'input-error'
            : '')
        "
        min="0"
        max="1000"
      />
      % of pot
    </p>

    <p class="my-1">
      Force all-in if next opponent's bet size is less than:
      <input
        v-model="store.forceAllInThreshold"
        type="number"
        :class="
          'w-20 ml-2 px-2 py-1 rounded-lg text-sm text-center ' +
          (store.forceAllInThreshold < 0 || store.forceAllInThreshold > 100
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
          v-model="store.adjustLastTwoBetSizes"
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
import { useStore } from "../store";

export default defineComponent({
  setup() {
    const store = useStore();
    return {
      store,
    };
  },
});
</script>

<style>
input.input-error {
  @apply ring-1 ring-red-600 border-red-600 bg-red-50;
}
</style>
