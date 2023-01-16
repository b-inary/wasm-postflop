import * as Comlink from "comlink";
import { WorkerApi, Handler } from "./worker";

let worker: Worker | null = null;
let proxy: Comlink.Remote<WorkerApi> | null = null;
export let handler: Comlink.Remote<Handler> | null = null;

export const init = async (numThreads: number) => {
  if (worker && proxy) {
    await proxy.beforeTerminate();
    const oldWorker = worker;
    setTimeout(() => oldWorker.terminate(), 1000);
  }

  worker = new Worker(new URL("./worker.ts", import.meta.url), {
    type: "module",
  });

  proxy = Comlink.wrap<WorkerApi>(worker);
  handler = await proxy.initHandler(numThreads);
};
