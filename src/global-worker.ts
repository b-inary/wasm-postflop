import * as Comlink from "comlink";
import { WorkerApi } from "./worker";

let worker: Worker | null = null;

export async function init(numThreds: number) {
  if (worker) {
    worker.terminate();
  }

  worker = new Worker(new URL("./worker.ts", import.meta.url), {
    type: "module",
  });

  await Comlink.wrap<WorkerApi>(worker).init(numThreds);
}

export async function getHandler() {
  if (!worker) {
    throw new Error("Worker not initialized");
  }

  return await Comlink.wrap<WorkerApi>(worker).getHandler();
}
