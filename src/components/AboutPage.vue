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
    <li>Free to use</li>
    <li>Open-source: you can inspect the whole program</li>
    <li>No need to install anything</li>
    <li>Works inside your browser</li>
    <li>Sufficiently fast (supports multithreading)</li>
  </ul>

  <div class="mt-4 underline">Limitations:</div>

  <ul class="mt-4 ml-8 list-disc">
    <li>
      <span class="font-bold underline">
        Multithreading is not supported on iOS and Safari</span
      >
      because nested workers are
      <a href="https://bugs.webkit.org/show_bug.cgi?id=22723" class="link">
        not implemented
      </a>
      <ul class="ml-8" style="list-style-type: circle">
        <li>
          If you are using macOS, please use other browsers such as Google
          Chrome instead
        </li>
      </ul>
    </li>
    <li>
      <span class="font-bold underline">
        Available memory is limited to 4GB</span
      >
      due to the technical limitation of WebAssembly
    </li>
    <li>Mobile use is not considered</li>
    <li>
      This is a hobby project in the alpha stage; there are no guarantees, and
      only minimal functionality is provided
    </li>
  </ul>

  <div class="mt-4 underline">Change log:</div>

  <ul class="mt-4 ml-8 list-disc">
    <li>
      <span class="font-bold">2022-06-03</span>: Added equity calculation
      feature
    </li>
    <li><span class="font-bold">2022-06-01</span>: Initial release</li>
  </ul>
</template>

<style scoped>
.link {
  @apply text-blue-500 hover:underline;
}
</style>
