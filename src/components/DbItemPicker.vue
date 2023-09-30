<template>
  <div
    class="w-full h-[22.5rem] px-1 border border-gray-500 rounded-md shadow text-sm overflow-x-auto overflow-y-scroll select-none"
    @click="unselect"
    @keydown.f2="renameItem()"
    @keydown.delete="deleteItem(true)"
  >
    <!-- No saved items -->
    <div v-if="data.length === 0" class="item-toplevel">
      <span class="inline-block px-1">(No saved {{ storeName }})</span>
    </div>

    <!-- Depth 0 -->
    <div
      v-for="item0 in data"
      :key="item0.pathStr"
      :class="{ 'item-toplevel': !item0.isGroup }"
    >
      <template v-if="item0.isGroup">
        <!-- Open/Close button -->
        <button
          :class="item0.opened ? 'bottom-arrow' : 'right-arrow'"
          @click="toggleGroup(item0)"
        >
          &nbsp;
        </button>

        <!-- Name edit -->
        <input
          v-if="item0.isEditing"
          ref="nameInput"
          v-model="editingName"
          type="text"
          :class="'px-1 py-0 text-sm ' + (!isNameValid ? 'input-error' : '')"
          style="width: calc(100% - 1.25rem)"
          @blur="tryRename(item0)"
          @keydown.enter="tryRename(item0)"
          @keydown.escape="cancelRename(item0)"
          @keydown.delete.stop
        />

        <!-- Group name -->
        <label
          v-else
          class="inline-block"
          @dblclick="toggleGroup(item0)"
          @keydown.enter="toggleGroup(item0)"
        >
          <input
            v-model="selectedValue"
            type="radio"
            :name="`item-picker-${storeName}-${index}`"
            class="sr-only peer"
            :value="item0.pathStr"
          />
          <span
            class="inline-block px-1 rounded-sm whitespace-nowrap peer-checked:bg-blue-600 peer-checked:text-white"
          >
            {{ item0.path[0] }}
          </span>
        </label>

        <!-- Group items -->
        <div
          v-if="item0.opened"
          class="group"
          :style="{ '--guide-decrease': guideDecrease(item0) }"
        >
          <!-- Empty group -->
          <div v-if="item0.items.length === 0" class="item-inside">
            <span class="inline-block px-1">(Empty group)</span>
          </div>

          <!-- Depth 1 -->
          <div
            v-for="item1 in item0.items"
            :key="item1.pathStr"
            :class="item1.isGroup ? 'group-inside' : 'item-inside'"
          >
            <template v-if="item1.isGroup">
              <!-- Open/Close button -->
              <button
                :class="item1.opened ? 'bottom-arrow' : 'right-arrow'"
                @click="toggleGroup(item1)"
              >
                &nbsp;
              </button>

              <!-- Name edit -->
              <input
                v-if="item1.isEditing"
                ref="nameInput"
                v-model="editingName"
                type="text"
                :class="
                  'px-1 py-0 text-sm ' + (!isNameValid ? 'input-error' : '')
                "
                style="width: calc(100% - 1.25rem)"
                @blur="tryRename(item1)"
                @keydown.enter="tryRename(item1)"
                @keydown.escape="cancelRename(item1)"
                @keydown.delete.stop
              />

              <!-- Group name -->
              <label
                v-else
                class="inline-block"
                @dblclick="toggleGroup(item1)"
                @keydown.enter="toggleGroup(item1)"
              >
                <input
                  v-model="selectedValue"
                  type="radio"
                  :name="`item-picker-${storeName}-${index}`"
                  class="sr-only peer"
                  :value="item1.pathStr"
                />
                <span
                  class="inline-block px-1 rounded-sm whitespace-nowrap peer-checked:bg-blue-600 peer-checked:text-white"
                >
                  {{ item1.path[1] }}
                </span>
              </label>

              <!-- Group items -->
              <div
                v-if="item1.opened"
                class="group"
                :style="{
                  '--guide-decrease': guideDecrease(item1),
                }"
              >
                <!-- Empty group -->
                <div v-if="item1.items.length === 0" class="item-inside">
                  <span class="inline-block px-1">(Empty group)</span>
                </div>

                <!-- Depth 2 -->
                <div
                  v-for="item2 in item1.items"
                  :key="item2.pathStr"
                  :class="item2.isGroup ? 'group-inside' : 'item-inside'"
                >
                  <template v-if="item2.isGroup">
                    <!-- Open/Close button -->
                    <button
                      :class="item2.opened ? 'bottom-arrow' : 'right-arrow'"
                      @click="toggleGroup(item2)"
                    >
                      &nbsp;
                    </button>

                    <!-- Name edit -->
                    <input
                      v-if="item2.isEditing"
                      ref="nameInput"
                      v-model="editingName"
                      type="text"
                      :class="
                        'px-1 py-0 text-sm ' +
                        (!isNameValid ? 'input-error' : '')
                      "
                      style="width: calc(100% - 1.25rem)"
                      @blur="tryRename(item2)"
                      @keydown.enter="tryRename(item2)"
                      @keydown.escape="cancelRename(item2)"
                      @keydown.delete.stop
                    />

                    <!-- Group name -->
                    <label
                      v-else
                      class="inline-block"
                      @dblclick="toggleGroup(item2)"
                      @keydown.enter="toggleGroup(item2)"
                    >
                      <input
                        v-model="selectedValue"
                        type="radio"
                        :name="`item-picker-${storeName}-${index}`"
                        class="sr-only peer"
                        :value="item2.pathStr"
                      />
                      <span
                        class="inline-block px-1 rounded-sm whitespace-nowrap peer-checked:bg-blue-600 peer-checked:text-white"
                      >
                        {{ item2.path[2] }}
                      </span>
                    </label>

                    <!-- Group items -->
                    <div
                      v-if="item2.opened"
                      class="group"
                      :style="{
                        '--guide-decrease': guideDecrease(item2),
                      }"
                    >
                      <!-- Empty group -->
                      <div v-if="item2.items.length === 0" class="item-inside">
                        <span class="inline-block px-1">(Empty group)</span>
                      </div>

                      <!-- Depth 3 -->
                      <div
                        v-for="item3 in item2.items"
                        :key="item3.pathStr"
                        class="item-inside"
                      >
                        <!-- Name edit -->
                        <input
                          v-if="item3.isEditing"
                          ref="nameInput"
                          v-model="editingName"
                          type="text"
                          :class="
                            'px-1 py-0 w-full text-sm ' +
                            (!isNameValid ? 'input-error' : '')
                          "
                          @blur="tryRename(item3)"
                          @keydown.enter="tryRename(item3)"
                          @keydown.escape="cancelRename(item3)"
                          @keydown.delete.stop
                        />

                        <!-- Item name -->
                        <label
                          v-else
                          class="inline-block"
                          @dblclick="loadItem()"
                          @keydown.enter="loadItem()"
                        >
                          <input
                            v-model="selectedValue"
                            type="radio"
                            :name="`item-picker-${storeName}-${index}`"
                            class="sr-only peer"
                            :value="item3.pathStr"
                          />
                          <span
                            class="inline-block px-1 rounded-sm whitespace-nowrap peer-checked:bg-blue-600 peer-checked:text-white"
                          >
                            {{ item3.path[3] }}
                          </span>
                        </label>
                      </div>
                    </div>
                  </template>

                  <template v-else>
                    <!-- Name edit -->
                    <input
                      v-if="item2.isEditing"
                      ref="nameInput"
                      v-model="editingName"
                      type="text"
                      :class="
                        'px-1 py-0 w-full text-sm ' +
                        (!isNameValid ? 'input-error' : '')
                      "
                      @blur="tryRename(item2)"
                      @keydown.enter="tryRename(item2)"
                      @keydown.escape="cancelRename(item2)"
                      @keydown.delete.stop
                    />

                    <!-- Item name -->
                    <label
                      v-else
                      class="inline-block"
                      @dblclick="loadItem()"
                      @keydown.enter="loadItem()"
                    >
                      <input
                        v-model="selectedValue"
                        type="radio"
                        :name="`item-picker-${storeName}-${index}`"
                        class="sr-only peer"
                        :value="item2.pathStr"
                      />
                      <span
                        class="inline-block px-1 rounded-sm whitespace-nowrap peer-checked:bg-blue-600 peer-checked:text-white"
                      >
                        {{ item2.path[2] }}
                      </span>
                    </label>
                  </template>
                </div>
              </div>
            </template>

            <template v-else>
              <!-- Name edit -->
              <input
                v-if="item1.isEditing"
                ref="nameInput"
                v-model="editingName"
                type="text"
                :class="
                  'px-1 py-0 w-full text-sm ' +
                  (!isNameValid ? 'input-error' : '')
                "
                @blur="tryRename(item1)"
                @keydown.enter="tryRename(item1)"
                @keydown.escape="cancelRename(item1)"
                @keydown.delete.stop
              />

              <!-- Item name -->
              <label
                v-else
                class="inline-block"
                @dblclick="loadItem()"
                @keydown.enter="loadItem()"
              >
                <input
                  v-model="selectedValue"
                  type="radio"
                  :name="`item-picker-${storeName}-${index}`"
                  class="sr-only peer"
                  :value="item1.pathStr"
                />
                <span
                  class="inline-block px-1 rounded-sm whitespace-nowrap peer-checked:bg-blue-600 peer-checked:text-white"
                >
                  {{ item1.path[1] }}
                </span>
              </label>
            </template>
          </div>
        </div>
      </template>

      <template v-else>
        <!-- Name edit -->
        <input
          v-if="item0.isEditing"
          ref="nameInput"
          v-model="editingName"
          type="text"
          :class="
            'px-1 py-0 w-full text-sm ' + (!isNameValid ? 'input-error' : '')
          "
          @blur="tryRename(item0)"
          @keydown.enter="tryRename(item0)"
          @keydown.escape="cancelRename(item0)"
          @keydown.delete.stop
        />

        <!-- Item name -->
        <label
          v-else
          class="inline-block"
          @dblclick="loadItem()"
          @keydown.enter="loadItem()"
        >
          <input
            v-model="selectedValue"
            type="radio"
            :name="`item-picker-${storeName}-${index}`"
            class="sr-only peer"
            :value="item0.pathStr"
          />
          <span
            class="inline-block px-1 rounded-sm whitespace-nowrap peer-checked:bg-blue-600 peer-checked:text-white"
          >
            {{ item0.path[0] }}
          </span>
        </label>
      </template>
    </div>
  </div>

  <div v-if="errorOccured" class="mt-3 text-red-500 font-bold">
    Something went wrong. Please reload the page.
  </div>

  <div class="flex flex-col mt-4 gap-3">
    <div class="grid grid-cols-3 w-full gap-3">
      <button
        class="button-base button-blue button-overrides"
        :disabled="selectedItem?.item?.isGroup ?? true"
        @click="loadItem()"
      >
        Load
      </button>

      <button
        class="button-base button-blue button-overrides"
        :disabled="errorOccured || !allowSave || isEditing"
        @click="addOrOverwriteItem()"
      >
        {{ selectedItem?.item?.isGroup === false ? "Overwrite" : "Save" }}
      </button>

      <button
        class="button-base button-blue button-overrides"
        :disabled="errorOccured || selectedValue === false"
        @click="renameItem()"
      >
        Rename
      </button>
    </div>

    <div class="grid grid-cols-2 w-full gap-3">
      <button
        class="button-base button-blue button-overrides"
        :disabled="
          errorOccured ||
          (selectedItem !== null &&
            selectedItem.item.path.length +
              (selectedItem.item.isGroup ? 1 : 0) >=
              4)
        "
        @click="addGroup()"
      >
        Add Group
      </button>

      <button
        class="button-base button-red button-overrides"
        :disabled="errorOccured || selectedValue === false"
        @click="deleteItem(true)"
      >
        Delete
      </button>

      <input
        ref="importJsonInput"
        type="file"
        class="hidden"
        accept=".json"
        @change="importJson"
      />
      <button
        class="button-base button-green button-overrides"
        :disabled="errorOccured || isEditing"
        @click="importJsonInput?.click()"
      >
        Import JSON
      </button>

      <a
        ref="exportJsonButton"
        :class="
          'button-base button-green button-overrides text-center select-none ' +
          (errorOccured || isEditing
            ? 'cursor-default pointer-events-none opacity-40'
            : 'cursor-pointer')
        "
        :download="storeName + '.json'"
        @click="exportJson"
      >
        Export JSON
      </a>
    </div>
  </div>

  <div
    v-if="importError !== ''"
    class="flex flex-col mt-4 px-2 py-1 text-red-500 bg-red-50 border-2 border-red-600 rounded-md font-semibold"
  >
    <div class="flex">
      Error: Import failed.
      <button
        class="w-6 h-6 ml-auto text-gray-700 opacity-70 hover:opacity-100"
        @click="importError = ''"
      >
        <XMarkIcon class="w-full h-full" />
      </button>
    </div>
    <div>- {{ importError }}</div>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, nextTick, ref } from "vue";
import * as Db from "../db";

import { XMarkIcon } from "@heroicons/vue/20/solid";

type Item = {
  isGroup: false;
  path: string[];
  pathStr: string;
  value: unknown;
  isEditing: boolean;
};

type Group = {
  isGroup: true;
  path: string[];
  pathStr: string;
  opened: boolean;
  isEditing: boolean;
  items: (Item | Group)[];
};

const isEqual = (lhs: unknown, rhs: unknown) => {
  if (lhs === null || typeof lhs !== "object") return lhs === rhs;
  if (Array.isArray(lhs)) {
    if (!Array.isArray(rhs) || lhs.length !== rhs.length) return false;
    for (let i = 0; i < lhs.length; ++i) {
      if (!isEqual(lhs[i], rhs[i])) return false;
    }
  } else {
    if (rhs === null || typeof rhs !== "object") return false;
    const lhsEntries = Object.entries(lhs);
    const rhsEntries = Object.entries(rhs);
    if (lhsEntries.length !== rhsEntries.length) return false;
    lhsEntries.sort((a, b) => a[0].localeCompare(b[0]));
    rhsEntries.sort((a, b) => a[0].localeCompare(b[0]));
    for (let i = 0; i < lhsEntries.length; ++i) {
      if (lhsEntries[i][0] !== rhsEntries[i][0]) return false;
      if (!isEqual(lhsEntries[i][1], rhsEntries[i][1])) return false;
    }
  }
  return true;
};

const clone = (value: unknown) => {
  if (value === null || typeof value !== "object") return value;
  if (Array.isArray(value)) return [...value];
  return { ...value };
};

const groupToDbGroup = (group: Group): Db.DbGroup => ({
  name0: group.path[0],
  name1: group.path[1] ?? "",
  name2: group.path[2] ?? "",
  name3: group.path[3] ?? "",
  isGroup: 1,
});

const itemToDbItem = (item: Item): Db.DbItem => ({
  name0: item.path[0],
  name1: item.path[1] ?? "",
  name2: item.path[2] ?? "",
  name3: item.path[3] ?? "",
  isGroup: 0,
  value: clone(item.value),
});

const toDbItemOrGroup = (itemOrGroup: Item | Group): Db.DbItem | Db.DbGroup =>
  itemOrGroup.isGroup ? groupToDbGroup(itemOrGroup) : itemToDbItem(itemOrGroup);

type AddItemMessage = {
  type: "addItem";
  path: string[];
  value: unknown;
};

type OverwriteItemMessage = {
  type: "overwriteItem";
  path: string[];
  value: unknown;
};

type RenameItemMessage = {
  type: "renameItem";
  path: string[];
  newName: string;
};

type AddGroupMessage = {
  type: "addGroup";
  path: string[];
};

type DeleteItemMessage = {
  type: "deleteItem";
  path: string[];
};

type ImportJsonMessage = {
  type: "importJson";
};

type Message =
  | AddItemMessage
  | OverwriteItemMessage
  | RenameItemMessage
  | AddGroupMessage
  | DeleteItemMessage
  | ImportJsonMessage;

export default defineComponent({
  components: {
    XMarkIcon,
  },

  props: {
    storeName: {
      type: String,
      required: true,
    },
    index: {
      type: Number,
      default: 0,
    },
    value: {
      type: null,
      required: true,
    },
    allowSave: {
      type: Boolean,
      required: true,
    },
  },

  emits: {
    "load-item": (_value: unknown) => true,
  },

  setup(props, context) {
    const data = ref<(Item | Group)[]>([]);
    const selectedValue = ref(false as false | string);

    const selectedItem = computed(() => {
      if (selectedValue.value === false) return null;
      const path = JSON.parse(selectedValue.value);
      let item: Item | Group | undefined;
      let items = data.value;
      for (let i = 0; i < path.length; ++i) {
        item = items.find((item) => item.path[i] === path[i]);
        if (item === undefined) return null;
        if (i < path.length - 1) {
          if (!item.isGroup) return null;
          items = item.items;
        }
      }
      return item ? { item, parentItems: items } : null;
    });

    const errorOccured = ref(false);

    // load data from DB
    const loadData = async () => {
      const masterData = await Db.getArray(props.storeName);

      masterData.sort(
        (a, b) =>
          Number(b.name1 === "") - Number(a.name1 === "") ||
          Number(b.name2 === "") - Number(a.name2 === "") ||
          Number(b.name3 === "") - Number(a.name3 === "") ||
          b.isGroup - a.isGroup ||
          a.name0.localeCompare(b.name0, undefined, { numeric: true }) ||
          a.name1.localeCompare(b.name1, undefined, { numeric: true }) ||
          a.name2.localeCompare(b.name2, undefined, { numeric: true }) ||
          a.name3.localeCompare(b.name3, undefined, { numeric: true })
      );

      data.value = [];
      selectedValue.value = false;

      const dataMap = new Map<string, Item | Group>();
      const dataItems = new Map<string, (Item | Group)[]>();
      dataItems.set(JSON.stringify([]), data.value);

      for (const masterItem of masterData) {
        if (masterItem.isGroup) {
          const path = [
            masterItem.name0,
            masterItem.name1,
            masterItem.name2,
            "",
          ];
          const depth = path.findIndex((name) => name === "");
          path.length = depth;
          const pathStr = JSON.stringify(path);
          const parent = path.slice(0, -1);
          const parentStr = JSON.stringify(parent);
          const parentItems = dataItems.get(parentStr);
          if (parentItems) {
            const items: (Item | Group)[] = [];
            const group: Group = {
              isGroup: true,
              path,
              pathStr,
              opened: depth === 1,
              isEditing: false,
              items,
            };
            parentItems.push(group);
            dataMap.set(pathStr, group);
            dataItems.set(pathStr, items);
          } else {
            errorOccured.value = true;
            return;
          }
        } else {
          const path = [
            masterItem.name0,
            masterItem.name1,
            masterItem.name2,
            masterItem.name3,
            "",
          ];
          const depth = path.findIndex((name) => name === "");
          path.splice(depth);
          const pathStr = JSON.stringify(path);
          const parent = path.slice(0, -1);
          const parentStr = JSON.stringify(parent);
          const parentItems = dataItems.get(parentStr);
          if (parentItems) {
            const item: Item = {
              isGroup: false,
              path,
              pathStr,
              value: masterItem.value,
              isEditing: false,
            };
            parentItems.push(item);
            dataMap.set(pathStr, item);
          } else {
            errorOccured.value = true;
            return;
          }
        }
      }
    };

    loadData();

    // Broadcast channel
    const channel = new BroadcastChannel(`item-picker-${props.storeName}`);

    channel.onmessage = async (event: MessageEvent<Message>) => {
      if (errorOccured.value) return;

      const message = event.data;
      const pathStr =
        message.type === "importJson" ? "" : JSON.stringify(message.path);

      if (message.type === "addItem") {
        // add item
        if (message.path.length > 1) {
          selectedValue.value = JSON.stringify(message.path.slice(0, -1));
          if (selectedItem.value === null) {
            errorOccured.value = true;
            selectedValue.value = false;
            return;
          }
        } else {
          selectedValue.value = false;
        }
        await addOrOverwriteItem(
          message.value,
          message.path[message.path.length - 1]
        );
      } else if (message.type === "overwriteItem") {
        // overwrite item
        selectedValue.value = pathStr;
        if (selectedItem.value === null) {
          errorOccured.value = true;
          selectedValue.value = false;
          return;
        }
        await addOrOverwriteItem(message.value);
      } else if (message.type === "renameItem") {
        // rename item
        selectedValue.value = pathStr;
        if (selectedItem.value === null) {
          errorOccured.value = true;
          selectedValue.value = false;
          return;
        }
        await tryRename(selectedItem.value.item, message.newName);
      } else if (message.type === "addGroup") {
        // add group
        if (message.path.length > 1) {
          selectedValue.value = JSON.stringify(message.path.slice(0, -1));
          if (selectedItem.value === null) {
            errorOccured.value = true;
            selectedValue.value = false;
            return;
          }
        } else {
          selectedValue.value = false;
        }
        await addGroup(message.path[message.path.length - 1]);
      } else if (message.type === "deleteItem") {
        // delete item
        selectedValue.value = pathStr;
        if (selectedItem.value === null) {
          errorOccured.value = true;
          selectedValue.value = false;
          return;
        }
        await deleteItem(false);
      } else if (message.type === "importJson") {
        // import JSON
        await loadData();
      }
    };

    const isEditing = ref(false);
    const editingName = ref("");
    const duplicateName = ref([] as string[]);
    const nameInput = ref(null as HTMLInputElement[] | null);

    const unselect = (ev: MouseEvent) => {
      const target = ev.target as HTMLElement;
      if (target.tagName === "DIV") {
        selectedValue.value = false;
      }
    };

    const closeGroup = (group: Group) => {
      group.opened = false;
      for (const child of group.items) {
        if (child.isGroup) {
          closeGroup(child);
        }
      }
    };

    const toggleGroup = (group: Group) => {
      selectedValue.value = false;
      if (group.opened) {
        closeGroup(group);
      } else {
        group.opened = true;
      }
    };

    const numDisplayedRows = (group: Group) => {
      if (!group.opened) return 0;
      let ret = group.items.length || 1;
      for (const child of group.items) {
        if (child.isGroup) {
          ret += numDisplayedRows(child);
        }
      }
      return ret;
    };

    const guideDecrease = (group: Group) => {
      if (group.items.length > 0) {
        const lastChild = group.items[group.items.length - 1];
        if (lastChild.isGroup) {
          return numDisplayedRows(lastChild);
        }
      }
      return 0;
    };

    const loadItem = () => {
      if (!selectedItem.value) return;
      if (selectedItem.value.item.isGroup === false) {
        context.emit("load-item", selectedItem.value.item.value);
      }
      selectedValue.value = false;
    };

    const addOrOverwriteItem = async (value?: unknown, name?: string) => {
      if (errorOccured.value) return;

      if (selectedItem.value?.item?.isGroup === false) {
        // overwrite
        selectedItem.value.item.value = value ?? props.value;

        if (value === undefined) {
          // save and broadcast
          errorOccured.value ||= !(await Db.overwriteItem(
            props.storeName,
            itemToDbItem(selectedItem.value.item)
          ));
          channel.postMessage({
            type: "overwriteItem",
            path: [...selectedItem.value.item.path],
            value: clone(props.value),
          } as OverwriteItemMessage);
        }

        selectedValue.value = false;
      } else {
        // add
        const path = selectedItem.value?.item
          ? [...selectedItem.value.item.path, name ?? ""]
          : [name ?? ""];
        const pathStr = JSON.stringify(path);
        const parentItems = selectedItem.value?.item?.items ?? data.value;
        parentItems.push({
          isGroup: false,
          path,
          pathStr,
          value: value ?? props.value,
          isEditing: false,
        });

        if (name === undefined) {
          // user must enter a name
          if (selectedItem.value) {
            selectedItem.value.item.opened = true;
          }
          selectedValue.value = pathStr;
          await renameItem();
        } else {
          // name is provided
          selectedValue.value = false;
          const depth = path.length - 1;
          parentItems.sort(
            (a, b) =>
              Number(b.isGroup) - Number(a.isGroup) ||
              a.path[depth].localeCompare(b.path[depth], undefined, {
                numeric: true,
              })
          );
        }
      }
    };

    const isNameValid = computed(() => {
      const name = editingName.value.trim();
      return name !== "" && !duplicateName.value.includes(name);
    });

    const renameItem = async () => {
      if (!selectedItem.value || errorOccured.value) return;
      const item = selectedItem.value.item;
      item.isEditing = true;
      isEditing.value = true;
      editingName.value = item.path[item.path.length - 1];
      duplicateName.value = selectedItem.value.parentItems
        .map((item) => item.path[item.path.length - 1])
        .filter((name) => name !== editingName.value);
      const defaultName = item.isGroup
        ? "New group"
        : `New ${props.storeName.slice(0, -1)}`; // remove "s" hack
      if (editingName.value === "") {
        let i = 2;
        let newName = defaultName;
        while (duplicateName.value.includes(newName)) {
          newName = `${defaultName} (${i++})`;
        }
        editingName.value = newName;
      }
      selectedValue.value = false;
      await nextTick();
      if (nameInput.value) {
        nameInput.value[0].select();
        nameInput.value[0].scrollIntoView({ block: "nearest" });
      }
    };

    const tryRename = async (item: Item | Group, newName?: string) => {
      // existence check (`tryRename()` might be called when an item is deleted)
      selectedValue.value = item.pathStr;
      if (selectedItem.value === null) {
        selectedValue.value = false;
        return;
      }

      const isUserAction = newName === undefined;

      if (newName === undefined) {
        // is user action
        item.isEditing = false;
        isEditing.value = false;

        if (!isNameValid.value) {
          editingName.value = "";
          if (item.path[item.path.length - 1] === "") {
            await deleteItem(false);
          }
          return;
        }

        newName = editingName.value.trim();
        editingName.value = "";
      }

      if (errorOccured.value) return;

      const depth = item.path.length - 1;
      const isNewItem = item.path[depth] === "";
      const prevPath = [...item.path];

      if (item.path[depth] === newName) {
        if (isUserAction) {
          // give focus
          await nextTick();
          const name = `item-picker-${props.storeName}-${props.index}`;
          const checked = document.querySelector(
            `input[name="${name}"]:checked`
          );
          if (checked) {
            (checked as HTMLInputElement).focus();
          }
        }

        return;
      }

      item.path[depth] = newName;
      item.pathStr = JSON.stringify(item.path);
      if (item.isGroup) {
        renameRecursive(item, newName, depth);
      }

      selectedValue.value = item.pathStr;
      const parentItems = selectedItem.value?.parentItems;
      if (parentItems) {
        parentItems.sort(
          (a, b) =>
            Number(b.isGroup) - Number(a.isGroup) ||
            a.path[depth].localeCompare(b.path[depth], undefined, {
              numeric: true,
            })
        );
      }

      if (isUserAction) {
        // save and broadcast
        if (isNewItem) {
          // add
          if (!item.isGroup) {
            // item
            errorOccured.value ||= !(await Db.addItem(
              props.storeName,
              itemToDbItem(item)
            ));
            channel.postMessage({
              type: "addItem",
              path: [...item.path],
              value: clone(item.value),
            } as AddItemMessage);
          } else {
            // group
            errorOccured.value ||= !(await Db.addGroup(
              props.storeName,
              groupToDbGroup(item)
            ));
            channel.postMessage({
              type: "addGroup",
              path: [...item.path],
            } as AddGroupMessage);
          }
        } else {
          // rename
          errorOccured.value ||= !(await Db.renameItem(
            props.storeName,
            toDbItemOrGroup({ ...item, path: prevPath }),
            newName
          ));
          channel.postMessage({
            type: "renameItem",
            path: prevPath,
            newName: newName,
          } as RenameItemMessage);
        }

        // give focus
        await nextTick();
        const name = `item-picker-${props.storeName}-${props.index}`;
        const checked = document.querySelector(`input[name="${name}"]:checked`);
        if (checked) {
          (checked as HTMLInputElement).focus();
        }
      } else {
        selectedValue.value = false;
      }
    };

    const renameRecursive = (group: Group, newName: string, depth: number) => {
      for (const item of group.items) {
        item.path[depth] = newName;
        item.pathStr = JSON.stringify(item.path);
        if (item.isGroup) {
          renameRecursive(item, newName, depth);
        }
      }
    };

    const cancelRename = async (item: Item | Group) => {
      item.isEditing = false;
      isEditing.value = false;
      editingName.value = "";
      if (item.path[item.path.length - 1] === "") {
        selectedValue.value = item.pathStr;
        await deleteItem(false);
      }
    };

    const addGroup = async (name?: string) => {
      if (errorOccured.value) return;

      const path = selectedItem.value
        ? selectedItem.value.item.isGroup
          ? [...selectedItem.value.item.path, name ?? ""]
          : [...selectedItem.value.item.path.slice(0, -1), name ?? ""]
        : [name ?? ""];
      if (path.length > 3) return;
      const pathStr = JSON.stringify(path);
      const parentItems = selectedItem.value
        ? selectedItem.value.item.isGroup
          ? selectedItem.value.item.items
          : selectedItem.value.parentItems
        : data.value;
      parentItems.push({
        isGroup: true,
        path,
        pathStr,
        opened: false,
        isEditing: false,
        items: [],
      });

      if (name === undefined) {
        // user must enter a name
        if (selectedItem.value?.item?.isGroup) {
          selectedItem.value.item.opened = true;
        }
        selectedValue.value = pathStr;
        renameItem();
      } else {
        // name is provided
        selectedValue.value = false;
        const depth = path.length - 1;
        parentItems.sort(
          (a, b) =>
            Number(b.isGroup) - Number(a.isGroup) ||
            a.path[depth].localeCompare(b.path[depth], undefined, {
              numeric: true,
            })
        );
      }
    };

    const deleteItem = async (isUserAction: boolean) => {
      if (!selectedItem.value || errorOccured.value) return;

      const path = [...selectedItem.value.item.path];
      const dbItem = toDbItemOrGroup(selectedItem.value.item);

      const parentItems = selectedItem.value.parentItems;
      parentItems.splice(parentItems.indexOf(selectedItem.value.item), 1);
      selectedValue.value = false;

      if (isUserAction) {
        // save and broadcast
        errorOccured.value ||= !(await Db.deleteItem(props.storeName, dbItem));
        channel.postMessage({
          type: "deleteItem",
          path,
        } as DeleteItemMessage);
      }
    };

    const importError = ref("");
    const importJsonInput = ref<HTMLInputElement | null>(null);
    const exportJsonButton = ref<HTMLAnchorElement | null>(null);

    type JsonItem = {
      path: string[];
      isGroup: boolean;
      value?: unknown;
    };

    const checkJson = (array: JsonItem[]) => {
      if (!Array.isArray(array)) return false;

      let valueType = "";
      if (props.storeName === "ranges") valueType = "string";
      if (props.storeName === "configurations") valueType = "object";
      if (valueType === "") return false;

      const map = new Map<string, boolean>();
      map.set(JSON.stringify([]), true);

      for (const item of array) {
        if (
          typeof item.isGroup !== "boolean" ||
          !Array.isArray(item.path) ||
          item.path.some((x) => typeof x !== "string") ||
          item.path.length > (item.isGroup ? 3 : 4) ||
          (!item.isGroup && typeof item.value !== valueType)
        ) {
          return false;
        }

        item.path = item.path.map((x) => x.trim());

        if (
          item.path.some((x) => x === "") ||
          !map.get(JSON.stringify(item.path.slice(0, -1))) ||
          map.has(JSON.stringify(item.path))
        ) {
          return false;
        }

        map.set(JSON.stringify(item.path), item.isGroup);
      }

      return true;
    };

    const getItemsToAdd = async (array: JsonItem[]) => {
      const masterData = await Db.getArray(props.storeName);
      const masterMap = new Map<string, Db.DbItem | Db.DbGroup>();
      for (const item of masterData) {
        const path = [item.name0, item.name1, item.name2, item.name3, ""];
        path.length = path.findIndex((x) => x === "");
        masterMap.set(JSON.stringify(path), item);
      }

      const ret: (Db.DbItem | Db.DbGroup)[] = [];

      for (const item of array) {
        const depth = item.path.length;
        const name = item.path[depth - 1];
        let pathStr = JSON.stringify(item.path);

        if (item.isGroup) {
          const master = masterMap.get(pathStr);
          if (master && master.isGroup === 0) return name;
          if (!master) {
            ret.push({
              name0: item.path[0],
              name1: item.path[1] ?? "",
              name2: item.path[2] ?? "",
              name3: item.path[3] ?? "",
              isGroup: 1,
            });
          }
        } else {
          let i = 2;
          let master = masterMap.get(pathStr);
          while (
            master &&
            (master.isGroup === 1 || !isEqual(master.value, item.value))
          ) {
            item.path[depth - 1] = `${name} (${i++})`;
            pathStr = JSON.stringify(item.path);
            master = masterMap.get(pathStr);
          }
          if (!master) {
            ret.push({
              name0: item.path[0],
              name1: item.path[1] ?? "",
              name2: item.path[2] ?? "",
              name3: item.path[3] ?? "",
              isGroup: 0,
              value: item.value,
            });
          }
        }
      }

      return ret;
    };

    const importJson = async () => {
      if (errorOccured.value || isEditing.value || !importJsonInput.value) {
        return;
      }

      const file = importJsonInput.value.files?.[0];
      if (!file) return;

      importError.value = "";
      importJsonInput.value.value = "";

      const text = await file.text();
      let obj: { version: number; name: string; data: JsonItem[] };
      try {
        obj = JSON.parse(text);
      } catch (e) {
        importError.value = "Parse error (invalid JSON)";
        return;
      }

      if (obj.name !== props.storeName) {
        importError.value = "Data type mismatch";
        return;
      }

      if (obj.version !== 2) {
        importError.value = "Version mismatch";
        return;
      }

      if (!checkJson(obj.data)) {
        importError.value = "Invalid data";
        return;
      }

      const itemsToAdd = await getItemsToAdd(obj.data);
      if (typeof itemsToAdd === "string") {
        importError.value = `Cannot create group "${itemsToAdd}" because the item already exists`;
        return;
      }

      errorOccured.value ||= !(await Db.bulkAdd(props.storeName, itemsToAdd));
      channel.postMessage({ type: "importJson" });
      await loadData();
    };

    const appendRecursive = (item: Item | Group, array: JsonItem[]) => {
      array.push({
        path: item.path,
        isGroup: item.isGroup,
        value: item.isGroup ? undefined : item.value,
      });
      if (item.isGroup) {
        for (const child of item.items) {
          appendRecursive(child, array);
        }
      }
    };

    const exportJson = () => {
      if (errorOccured.value || isEditing.value || !exportJsonButton.value) {
        return;
      }

      const array: JsonItem[] = [];
      for (const item of data.value) {
        appendRecursive(item, array);
      }

      const obj = { version: 2, name: props.storeName, data: array };
      const jsonStr = JSON.stringify(obj, undefined, 2);

      const blob = new Blob([jsonStr], { type: "application/json" });
      const url = URL.createObjectURL(blob);
      exportJsonButton.value.href = url;
    };

    return {
      data,
      selectedValue,
      selectedItem,
      errorOccured,
      isEditing,
      editingName,
      nameInput,
      unselect,
      toggleGroup,
      guideDecrease,
      loadItem,
      addOrOverwriteItem,
      isNameValid,
      renameItem,
      tryRename,
      cancelRename,
      addGroup,
      deleteItem,
      importError,
      importJsonInput,
      exportJsonButton,
      importJson,
      exportJson,
    };
  },
});
</script>

<style scoped>
input.input-error {
  @apply ring-1 ring-red-600 border-red-600 bg-red-50;
}

.group {
  @apply relative pl-5;
}

.group::before {
  content: "";
  height: calc(100% - 1.4rem * var(--guide-decrease) - 0.825rem);
  @apply absolute top-0.5 left-2.5 border-l border-dotted border-gray-500;
}

.group-inside {
  @apply relative;
}

.group-inside::before {
  content: "";
  @apply absolute top-[0.7rem] -left-[0.5625rem] w-2.5 border-t border-dotted border-gray-500;
}

.item-toplevel {
  @apply relative pl-5;
}

.item-toplevel::before {
  content: "";
  @apply absolute top-1/2 left-[0.0625rem] w-2.5 border-t border-dotted border-gray-500;
}

.item-inside {
  @apply relative pl-5;
}

.item-inside::before {
  content: "";
  @apply absolute top-1/2 -left-[0.5625rem] w-5 border-t border-dotted border-gray-500;
}

.right-arrow,
.bottom-arrow {
  @apply relative inline-block w-5 min-h-full;
}

.right-arrow::before {
  content: "";
  @apply absolute w-1.5 h-1.5 -mt-0.5 top-1/2 left-1.5 border-t-2 border-r-2 border-gray-500 rotate-45;
}

.bottom-arrow::before {
  content: "";
  @apply absolute w-1.5 h-1.5 -mt-[0.1875rem] top-1/2 left-[0.4375rem] border-r-2 border-b-2 border-gray-500 rotate-45;
}

.button-overrides {
  @apply px-0;
}
</style>
