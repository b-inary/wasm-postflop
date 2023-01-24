import Dexie, { Table } from "dexie";
import { ConfigValue1, migrateConfig1to2 } from "./db-migration";

export type DbItem = {
  id?: number;
  name0: string;
  name1: string;
  name2: string;
  name3: string;
  isGroup: 0;
  value: unknown;
};

export type DbGroup = {
  id?: number;
  name0: string;
  name1: string;
  name2: string;
  name3: string;
  isGroup: 1;
};

class WASMPostflopDB extends Dexie {
  public ranges!: Table<DbItem | DbGroup, number>;
  public configurations!: Table<DbItem | DbGroup, number>;

  public constructor() {
    super("WASMPostflopDB");

    this.version(1).stores({
      ranges: "++id, [name0+name1+name2+name3+isGroup]",
      configurations: "++id, [name0+name1+name2+name3+isGroup]",
    });

    this.version(2)
      .stores({
        ranges: "++id, [name0+name1+name2+name3+isGroup]",
        configurations: "++id, [name0+name1+name2+name3+isGroup]",
      })
      .upgrade((tx) => {
        return tx
          .table("configurations")
          .toCollection()
          .modify((item: DbItem | DbGroup) => {
            if (!item.isGroup) {
              item.value = migrateConfig1to2(item.value as ConfigValue1);
            }
          });
      });
  }
}

const db = new WASMPostflopDB();

const makeParent = (item: DbItem | DbGroup) => {
  if (item.name3 !== "") {
    return { ...item, name3: "", isGroup: 1 };
  } else if (item.name2 !== "") {
    return { ...item, name2: "", isGroup: 1 };
  } else if (item.name1 !== "") {
    return { ...item, name1: "", isGroup: 1 };
  } else {
    throw new Error("Cannot make parent of top-level item");
  }
};

const makeRenamed = (item: DbItem | DbGroup, newName: string) => {
  if (item.name3 !== "") {
    return { ...item, name3: newName };
  } else if (item.name2 !== "") {
    return { ...item, name2: newName };
  } else if (item.name1 !== "") {
    return { ...item, name1: newName };
  } else {
    return { ...item, name0: newName };
  }
};

export const getArray = async (store: string) => {
  return (await db.table(store).toArray()) as (DbItem | DbGroup)[];
};

export const addItem = async (store: string, item: DbItem) => {
  try {
    const table = db.table(store);

    return await db.transaction("rw", table, async () => {
      // duplicate check
      const count = await table
        .where("[name0+name1+name2+name3]")
        .equals([item.name0, item.name1, item.name2, item.name3])
        .count();
      if (count > 0) {
        return false;
      }

      // parent check
      if (item.name1 !== "") {
        const parent = makeParent(item);
        const count = await table
          .where("[name0+name1+name2+name3+isGroup]")
          .equals([parent.name0, parent.name1, parent.name2, parent.name3, 1])
          .count();
        if (count !== 1) {
          return false;
        }
      }

      // insert
      await table.add(item);

      return true;
    });
  } catch {
    return false;
  }
};

export const overwriteItem = async (store: string, item: DbItem) => {
  try {
    const table = db.table(store);

    return await db.transaction("rw", table, async () => {
      // get collection
      const collection = table
        .where("[name0+name1+name2+name3+isGroup]")
        .equals([item.name0, item.name1, item.name2, item.name3, 0]);

      // check if exists
      if ((await collection.count()) !== 1) {
        return false;
      }

      // update
      return (await collection.modify({ value: item.value })) === 1;
    });
  } catch {
    return false;
  }
};

export const renameItem = async (
  store: string,
  item: DbItem | DbGroup,
  newName: string
) => {
  try {
    const table = db.table(store);

    return await db.transaction("rw", table, async () => {
      const renamed = makeRenamed(item, newName);

      // duplicate check
      const count = await table
        .where("[name0+name1+name2+name3]")
        .equals([renamed.name0, renamed.name1, renamed.name2, renamed.name3])
        .count();
      if (count > 0) {
        return false;
      }

      const [index, key, modifier] =
        item.name3 !== ""
          ? [
              "[name0+name1+name2+name3]",
              [item.name0, item.name1, item.name2, item.name3],
              { name3: newName },
            ]
          : item.name2 !== ""
          ? [
              "[name0+name1+name2]",
              [item.name0, item.name1, item.name2],
              { name2: newName },
            ]
          : item.name1 !== ""
          ? ["[name0+name1]", [item.name0, item.name1], { name1: newName }]
          : ["name0", item.name0, { name0: newName }];

      // update
      return (await table.where(index).equals(key).modify(modifier)) > 0;
    });
  } catch {
    return false;
  }
};

export const addGroup = async (store: string, group: DbGroup) => {
  if (group.name3 !== "") {
    return false;
  }

  try {
    const table = db.table(store);

    return await db.transaction("rw", table, async () => {
      // duplicate check
      const count = await table
        .where("[name0+name1+name2+name3]")
        .equals([group.name0, group.name1, group.name2, group.name3])
        .count();
      if (count > 0) {
        return false;
      }

      // parent check
      if (group.name1 !== "") {
        const parent = makeParent(group);
        const count = await table
          .where("[name0+name1+name2+name3+isGroup]")
          .equals([parent.name0, parent.name1, parent.name2, parent.name3, 1])
          .count();
        if (count !== 1) {
          return false;
        }
      }

      // insert
      await table.add(group);

      return true;
    });
  } catch {
    return false;
  }
};

export const deleteItem = async (store: string, item: DbItem | DbGroup) => {
  try {
    const table = db.table(store);

    return await db.transaction("rw", table, async () => {
      if (item.isGroup) {
        // check if exists
        const count = await table
          .where("[name0+name1+name2+name3+isGroup]")
          .equals([item.name0, item.name1, item.name2, item.name3, 1])
          .count();
        if (count !== 1) {
          return false;
        }

        const [index, key] =
          item.name2 !== ""
            ? ["[name0+name1+name2]", [item.name0, item.name1, item.name2]]
            : item.name1 !== ""
            ? ["[name0+name1]", [item.name0, item.name1]]
            : ["name0", item.name0];

        // delete
        return (await table.where(index).equals(key).delete()) > 0;
      } else {
        // get collection
        const collection = table
          .where("[name0+name1+name2+name3+isGroup]")
          .equals([item.name0, item.name1, item.name2, item.name3, 0]);

        // check if exists
        if ((await collection.count()) !== 1) {
          return false;
        }

        // delete
        return (await collection.delete()) === 1;
      }
    });
  } catch {
    return false;
  }
};

export const bulkAdd = async (store: string, items: (DbItem | DbGroup)[]) => {
  try {
    const table = db.table(store);

    return await db.transaction("rw", table, async () => {
      // insert
      await table.bulkAdd(items);

      return true;
    });
  } catch {
    return false;
  }
};
