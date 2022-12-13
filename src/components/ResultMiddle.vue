<template>
  <div class="flex h-12 border-y border-gray-500">
    <button
      v-for="mode in ['Basics', 'Graphs', 'Scatter', 'Compare']"
      :key="mode"
      :class="
        'flex w-[10%] h-full items-center justify-center font-bold text-lg transition ' +
        (statsMode === ''
          ? displayMode === mode.toLowerCase()
            ? 'bg-blue-100 underline'
            : 'hover:bg-blue-100'
          : displayMode === mode.toLowerCase()
          ? 'bg-red-100 underline'
          : 'bg-red-100')
      "
      @click="updateDisplayMode(mode.toLowerCase())"
    >
      {{ mode }}
    </button>
    <button
      :class="
        'flex w-[10%] h-full items-center justify-center font-bold text-lg transition enabled:hover:bg-blue-100 ' +
        (displayMode === 'stats' ? 'bg-blue-100 underline' : '')
      "
      :disabled="statsMode === ''"
      @click="updateDisplayMode('stats')"
    >
      {{ statsMode }}
    </button>

    <div
      class="flex flex-grow shrink-0 h-full pl-2 items-center justify-start gap-2 snug"
    >
      <div class="flex flex-col items-start justify-center h-full">
        <div class="text-sm">Player:</div>
        <select
          v-model="displayOptions.player"
          class="w-28 px-1 py-0.5 border-gray-600 bg-gray-200 rounded-lg shadow cursor-pointer bg-right"
          @change="updateDisplayOptions"
        >
          <option value="auto">Auto ({{ autoPlayer.toUpperCase() }})</option>
          <option value="oop">OOP</option>
          <option value="ip">IP</option>
        </select>
      </div>

      <div class="flex flex-col items-start justify-center h-full">
        <div class="text-sm">Bar Height:</div>
        <select
          v-model="displayOptions.barHeight"
          class="w-28 px-1 py-0.5 border-gray-600 bg-gray-200 rounded-lg shadow cursor-pointer bg-right"
          @change="updateDisplayOptions"
        >
          <option value="normalized">Normalized</option>
          <option value="absolute">Absolute</option>
          <option value="full">Full</option>
        </select>
      </div>

      <div class="flex flex-col items-start justify-center h-full">
        <div class="text-sm">Suit:</div>
        <select
          v-model="displayOptions.suit"
          class="w-[6.25rem] px-1 py-0.5 border-gray-600 bg-gray-200 rounded-lg shadow cursor-pointer bg-right"
          @change="updateDisplayOptions"
        >
          <option value="grouped">Grouped</option>
          <option value="individual">Individual</option>
        </select>
      </div>

      <div class="flex flex-col items-start justify-center h-full">
        <div class="text-sm">Display:</div>
        <select
          v-model="strategyContentPair"
          class="w-[8.75rem] px-1 py-0.5 border-gray-600 bg-gray-200 rounded-lg shadow cursor-pointer bg-right"
          @change="updateDisplayOptions"
        >
          <option value="show,default">Strategy</option>
          <option value="show,eq">Strategy + EQ</option>
          <option value="show,ev">Strategy + EV</option>
          <option value="show,eqr">Strategy + EQR</option>
          <option value="none,default">Weight</option>
          <option value="none,eq">EQ</option>
          <option value="none,ev">EV</option>
          <option value="none,eqr">EQR</option>
        </select>
      </div>

      <div class="flex-grow"></div>

      <div class="flex items-center pr-4 gap-4">
        <Tippy content="Copy range text to clipboard">
          <button
            :class="
              'flex w-8 h-8 items-center justify-center rounded-lg shadow transition-colors border ' +
              (copySuccess === 1
                ? 'border-green-600 bg-green-100'
                : copySuccess === -1
                ? 'border-red-600 bg-red-100'
                : 'border-gray-600 bg-gray-200')
            "
            @click="copyToClipboard"
            @mouseleave="onCopyTooltipLeave"
          >
            <ClipboardDocumentIcon
              v-show="copySuccess !== 1"
              :class="'w-5 h-5 ' + (copySuccess === -1 ? 'text-red-600' : '')"
            />
            <ClipboardDocumentCheckIcon
              v-show="copySuccess === 1"
              class="w-5 h-5 text-green-600"
            />
          </button>
        </Tippy>

        <Tippy content="Open solver integration menu; not yet implemented">
          <button
            class="flex w-8 h-8 items-center justify-center border border-gray-600 bg-gray-200 rounded-lg shadow transition-colors active:bg-gray-300"
          >
            <ComputerDesktopIcon class="w-5 h-5" />
          </button>
        </Tippy>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, reactive, ref } from "vue";
import { Tippy } from "vue-tippy";

import { ComputerDesktopIcon } from "@heroicons/vue/24/solid";
import {
  ClipboardDocumentIcon,
  ClipboardDocumentCheckIcon,
} from "@heroicons/vue/24/outline";

import { DisplayMode, DisplayOptions } from "../result-types";

export default defineComponent({
  components: {
    Tippy,
    ComputerDesktopIcon,
    ClipboardDocumentIcon,
    ClipboardDocumentCheckIcon,
  },

  props: {
    displayMode: {
      type: String,
      required: true,
    },
    statsMode: {
      type: String,
      required: true,
    },
    autoPlayer: {
      type: String as () => "oop" | "ip",
      required: true,
    },
    copySuccess: {
      type: Number,
      required: true,
    },
  },

  emits: {
    "update:display-mode": (_displayMode: DisplayMode) => true,
    "update:display-options": (_displayOptions: DisplayOptions) => true,
    "copy-to-clipboard": () => true,
    "reset-copy-success": () => true,
  },

  setup(_, context) {
    const displayOptions = reactive({
      player: "auto",
      barHeight: "normalized",
      suit: "grouped",
      strategy: "show",
      content: "default",
    } as DisplayOptions);

    const strategyContentPair = ref("show,default");

    const savedDisplayOptions = localStorage.getItem("display-options");
    if (savedDisplayOptions) {
      const parsed = JSON.parse(savedDisplayOptions);
      if (["normalized", "full", "absolute"].includes(parsed?.barHeight)) {
        displayOptions.barHeight = parsed.barHeight;
      }
      if (["grouped", "individual"].includes(parsed?.suit)) {
        displayOptions.suit = parsed.suit;
      }
      if (["show", "none"].includes(parsed?.strategy)) {
        displayOptions.strategy = parsed.strategy;
      }
      if (["default", "eq", "ev", "eqr"].includes(parsed?.content)) {
        displayOptions.content = parsed.content;
      }
      strategyContentPair.value = [
        displayOptions.strategy,
        displayOptions.content,
      ].join(",");
      context.emit("update:display-options", displayOptions);
    }

    const updateDisplayMode = (displayMode: string) => {
      context.emit("update:display-mode", displayMode as DisplayMode);
    };

    const updateDisplayOptions = () => {
      const [strategy, content] = strategyContentPair.value.split(",");
      displayOptions.strategy = strategy as DisplayOptions["strategy"];
      displayOptions.content = content as DisplayOptions["content"];
      localStorage.setItem("display-options", JSON.stringify(displayOptions));
      context.emit("update:display-options", displayOptions);
    };

    const copyToClipboard = () => {
      context.emit("copy-to-clipboard");
    };

    const onCopyTooltipLeave = () => {
      context.emit("reset-copy-success");
    };

    return {
      displayOptions,
      strategyContentPair,
      updateDisplayMode,
      updateDisplayOptions,
      copyToClipboard,
      onCopyTooltipLeave,
    };
  },
});
</script>
