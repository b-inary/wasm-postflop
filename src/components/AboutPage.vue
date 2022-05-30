<template>
  <div class="text-xl font-bold">Welcome to WASM Postflop!</div>

  <div class="mt-4">
    <span class="font-bold">WASM Postflop</span> is a free and open-source
    <span class="font-bold">GTO solver</span> that works on web browsers.<br />
    <span class="font-bold">WASM</span> stands for
    <a href="https://webassembly.org/" class="font-bold link">WebAssembly</a>.
    It is relatively new technology that allows us to distribute a fast,
    portable, and secure program across the web.
  </div>

  <ul class="mt-4 ml-8 list-disc">
    <li>free to use</li>
    <li>open-source: you can inspect the whole program</li>
    <li>no need to install anything</li>
    <li>works inside your browser</li>
    <li>sufficiently fast (supports multithreading)</li>
  </ul>

  <div class="mt-4 underline">Limitations:</div>

  <ul class="mt-4 ml-8 list-disc">
    <li>
      <span class="font-bold">available memory is limited to 4GB</span> due to
      technical limitation of WebAssembly
    </li>
    <li>mobile use is not considered</li>
    <li>
      this is a hobby project in the alpha stage; there are no guarantees, and
      only minimal functionality is provided
    </li>
  </ul>
</template>

<style scoped>
.link {
  @apply text-blue-500 hover:underline;
}
</style>
