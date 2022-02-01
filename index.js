import * as Comlink from "comlink";

/**
 * @type {HTMLParagraphElement}
 */
const pInit = document.getElementById("init-status");

/**
 * @type {HTMLParagraphElement}
 */
const pIter = document.getElementById("iter-status");

/**
 * @type {HTMLParagraphElement}
 */
const pExploit = document.getElementById("exploit-status");

/**
 * @type {HTMLParagraphElement}
 */
const pEv = document.getElementById("ev-status");

/**
 * @type {HTMLParagraphElement}
 */
const pTime = document.getElementById("time-status");

/**
 * @type {HTMLButtonElement}
 */
const stopButton = document.getElementById("stop-button");

let terminate = false;

stopButton.addEventListener("click", () => {
  terminate = true;
});

(async function () {
  const worker = await Comlink.wrap(
    new Worker(new URL("./wasm-worker.js", import.meta.url)),
    { type: "module" }
  ).handler;

  const start = performance.now();

  pInit.innerText = "Initializing...";
  await worker.init();
  pInit.innerText = "Initialization successful!";

  const err_string = await worker.get_err_string();
  if (err_string != null) {
    pIter.innerText = err_string;
    return;
  }

  const initialPot = 60;
  const maxIter = 1000;
  const target = initialPot * 0.005;

  for (let i = 0; i < maxIter; ++i) {
    pIter.innerText = `Iteration ${i}`;
    await worker.iterate(i);
    if ((i + 1) % 10 == 0) {
      const exploit = await worker.exploitability();
      pExploit.innerText = `Exploitability: ${exploit.toFixed(3)}`;
      if (exploit <= target) {
        terminate = true;
      }
    }
    if (terminate) {
      pIter.innerText = `Iteration ${i + 1}`;
      break;
    }
  }

  await worker.normalize();
  const ev0 = (await worker.ev(0)) + initialPot / 2;
  const ev1 = (await worker.ev(1)) + initialPot / 2;
  pEv.innerText = `EV: ${ev0.toFixed(3)} vs ${ev1.toFixed(3)}`;

  const end = performance.now();
  pTime.innerText = `Time: ${((end - start) / 1000).toFixed(2)}s`;
})();
