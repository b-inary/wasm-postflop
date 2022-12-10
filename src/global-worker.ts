import * as Comlink from "comlink";
import { WorkerApi, Handler } from "./worker";
export { ReadonlyBuffer } from "./worker";

let worker: Worker | null = null;
export let handler: Comlink.Remote<Handler> | null = null;
export let memory: Comlink.Remote<WebAssembly.Memory> | null = null;

export async function init(numThreads: number) {
  if (worker) {
    worker.terminate();
  }

  worker = new Worker(new URL("./worker.ts", import.meta.url), {
    type: "module",
  });

  await Comlink.wrap<WorkerApi>(worker).init(numThreads);

  handler = await Comlink.wrap<WorkerApi>(worker).getHandler();
  memory = await Comlink.wrap<WorkerApi>(worker).getMemory();
}
