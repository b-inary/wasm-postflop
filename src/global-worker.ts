import * as Comlink from "comlink";
import { WorkerApi } from "./worker";

let worker: Worker | null = null;

export async function init(numThreads: number) {
  if (worker) {
    worker.terminate();
  }

  worker = new Worker(new URL("./worker.ts", import.meta.url), {
    type: "module",
  });

  await Comlink.wrap<WorkerApi>(worker).init(numThreads);
}

export async function getMemory() {
  if (!worker) {
    throw new Error("Worker not initialized");
  }

  return Comlink.wrap<WorkerApi>(worker).getMemory();
}

export async function getHandler() {
  if (!worker) {
    throw new Error("Worker not initialized");
  }

  return await Comlink.wrap<WorkerApi>(worker).getHandler();
}
