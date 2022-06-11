<template>
  <div>
    <div
      ref="container"
      class="max-w-xs h-96 px-1 border border-gray-500 rounded-md shadow text-sm overflow-x-auto overflow-y-scroll select-none"
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
            v-model="edittingName"
            type="text"
            :class="'px-1 py-0 text-sm ' + (!isNameValid ? 'input-error' : '')"
            style="width: calc(100% - 1.25rem)"
            @blur="tryRename(item0)"
            @keydown.enter="tryRename(item0)"
            @keydown.escape="cancelRename(item0)"
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
                  v-model="edittingName"
                  type="text"
                  :class="
                    'px-1 py-0 text-sm ' + (!isNameValid ? 'input-error' : '')
                  "
                  style="width: calc(100% - 1.25rem)"
                  @blur="tryRename(item1)"
                  @keydown.enter="tryRename(item1)"
                  @keydown.escape="cancelRename(item1)"
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
                        v-model="edittingName"
                        type="text"
                        :class="
                          'px-1 py-0 text-sm ' +
                          (!isNameValid ? 'input-error' : '')
                        "
                        style="width: calc(100% - 1.25rem)"
                        @blur="tryRename(item2)"
                        @keydown.enter="tryRename(item2)"
                        @keydown.escape="cancelRename(item2)"
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
                        <div
                          v-if="item2.items.length === 0"
                          class="item-inside"
                        >
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
                            v-model="edittingName"
                            type="text"
                            :class="
                              'px-1 py-0 w-full text-sm ' +
                              (!isNameValid ? 'input-error' : '')
                            "
                            @blur="tryRename(item3)"
                            @keydown.enter="tryRename(item3)"
                            @keydown.escape="cancelRename(item3)"
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
                        v-model="edittingName"
                        type="text"
                        :class="
                          'px-1 py-0 w-full text-sm ' +
                          (!isNameValid ? 'input-error' : '')
                        "
                        @blur="tryRename(item2)"
                        @keydown.enter="tryRename(item2)"
                        @keydown.escape="cancelRename(item2)"
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
                  v-model="edittingName"
                  type="text"
                  :class="
                    'px-1 py-0 w-full text-sm ' +
                    (!isNameValid ? 'input-error' : '')
                  "
                  @blur="tryRename(item1)"
                  @keydown.enter="tryRename(item1)"
                  @keydown.escape="cancelRename(item1)"
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
            v-model="edittingName"
            type="text"
            :class="
              'px-1 py-0 w-full text-sm ' + (!isNameValid ? 'input-error' : '')
            "
            @blur="tryRename(item0)"
            @keydown.enter="tryRename(item0)"
            @keydown.escape="cancelRename(item0)"
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

    <div v-if="errorOccured" class="mt-3 text-red-600 font-bold">
      Something went wrong. Please reload the page.
    </div>

    <div class="flex flex-wrap justify-center mt-4 gap-3 max-w-xs">
      <button
        class="button button-blue"
        :disabled="selectedItem?.item?.isGroup ?? true"
        @click="loadItem()"
      >
        Load
      </button>

      <button
        class="button button-blue"
        :disabled="errorOccured || !allowSave"
        @click="addOrOverwriteItem()"
      >
        {{ selectedItem?.item?.isGroup === false ? "Overwrite" : "Save" }}
      </button>

      <button
        class="button button-blue"
        :disabled="errorOccured || selectedValue === false"
        @click="renameItem()"
      >
        Rename
      </button>

      <button
        class="button button-blue"
        :disabled="
          errorOccured ||
          (selectedItem !== null &&
            selectedItem.item.path.length +
              (selectedItem.item.isGroup ? 1 : 0) >=
              4)
        "
        @click="addGroup()"
      >
        Add group
      </button>

      <button
        class="button button-red"
        :disabled="errorOccured || selectedValue === false"
        @click="deleteItem(true)"
      >
        Delete
      </button>
    </div>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, nextTick, ref } from "vue";
import * as Db from "../db";

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

type Message =
  | AddItemMessage
  | OverwriteItemMessage
  | RenameItemMessage
  | AddGroupMessage
  | DeleteItemMessage;

export default defineComponent({
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

  emits: ["load-item"],

  setup(props, context) {
    const data = ref([] as (Item | Group)[]);
    const selectedValue = ref(false as false | string);

    const selectedItem = computed(() => {
      if (selectedValue.value === false) return null;
      const path = JSON.parse(selectedValue.value);
      let item: Item | Group | undefined;
      let items = data.value;
      for (let i = 0; i < path.length; ++i) {
        item = items.find((item) => item.path[i] === path[i]);
        if (i < path.length - 1 && item?.isGroup) items = item.items;
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
          path.splice(depth);
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
      const pathStr = JSON.stringify(message.path);

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
      }
    };

    const edittingName = ref("");
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
      const name = edittingName.value.trim();
      return name !== "" && !duplicateName.value.includes(name);
    });

    const renameItem = async () => {
      if (!selectedItem.value || errorOccured.value) return;
      const item = selectedItem.value.item;
      item.isEditing = true;
      edittingName.value = item.path[item.path.length - 1];
      duplicateName.value = selectedItem.value.parentItems
        .map((item) => item.path[item.path.length - 1])
        .filter((name) => name !== edittingName.value);
      const defaultName = item.isGroup
        ? "New group"
        : `New ${props.storeName.slice(0, -1)}`; // remove "s" hack
      if (edittingName.value === "") {
        let i = 2;
        let newName = defaultName;
        while (duplicateName.value.includes(newName)) {
          newName = `${defaultName} (${i++})`;
        }
        edittingName.value = newName;
      }
      selectedValue.value = false;
      await nextTick();
      if (nameInput.value) {
        nameInput.value[0].select();
        nameInput.value[0].scrollIntoView({ block: "nearest" });
      }
    };

    const tryRename = async (item: Item | Group, newName?: string) => {
      const isUserAction = newName === undefined;

      if (newName === undefined) {
        // is user action
        item.isEditing = false;

        if (!isNameValid.value) {
          edittingName.value = "";
          if (item.path[item.path.length - 1] === "") {
            selectedValue.value = item.pathStr;
            await deleteItem(false);
          }
          return;
        }

        newName = edittingName.value.trim();
        edittingName.value = "";
      }

      if (errorOccured.value) return;

      const depth = item.path.length - 1;
      const isNewItem = item.path[depth] === "";
      const prevPath = [...item.path];

      if (item.path[depth] === newName) {
        if (isUserAction) {
          // give focus
          selectedValue.value = item.pathStr;
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
      edittingName.value = "";
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

    return {
      data,
      selectedValue,
      selectedItem,
      errorOccured,
      edittingName,
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

.button {
  @apply rounded-lg shadow-sm py-1.5 w-24 text-white text-sm font-medium;
  @apply focus:outline-none focus:ring-4 disabled:opacity-40;
}

.button-blue {
  @apply bg-blue-600 hover:bg-blue-700 focus:ring-blue-300 disabled:bg-blue-600;
}

.button-red {
  @apply bg-red-600 hover:bg-red-700 focus:ring-red-300 disabled:bg-red-600;
}
</style>
