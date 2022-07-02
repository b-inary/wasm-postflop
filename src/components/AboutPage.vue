<template>
  <p>
    <span class="font-bold">WASM Postflop</span> is a free, open-source
    <span class="font-bold">GTO solver</span> that works on web browsers.<br />
    <span class="font-bold">WASM</span> stands for
    <a href="https://webassembly.org/" class="font-bold link">WebAssembly</a>.
    It is a relatively new technology that allows us to distribute a fast,
    portable, and secure program across the web.
  </p>

  <ul class="mt-4 ml-8 list-disc">
    <li><span class="font-bold">Free to use</span></li>
    <li>
      <span class="font-bold">Open source</span>: anyone can examine the whole
      program
    </li>
    <li>
      <span class="font-bold">Works on web browsers</span>: no need to install
      anything
    </li>
    <li>
      <span class="font-bold">Sufficiently fast</span>: supports multithreading
    </li>
  </ul>

  <p class="mt-4">
    For more detailed information about this project, please see
    <a href="https://github.com/b-inary/wasm-postflop" class="link">GitHub</a>.
  </p>

  <p class="mt-4 underline">Limitations:</p>

  <ul class="mt-4 ml-8 list-disc">
    <li>
      <span class="font-bold underline"
        >Multithreading is not supported on iOS and Safari</span
      >
      because nested workers are
      <a href="https://bugs.webkit.org/show_bug.cgi?id=22723" class="link"
        >not implemented</a
      >
      <ul class="ml-8" style="list-style-type: circle">
        <li>
          If you are using macOS, please use other browsers such as Google
          Chrome instead
        </li>
      </ul>
    </li>
    <li>
      <span class="font-bold underline"
        >Available memory is limited to 4GB</span
      >
      due to the technical limitation of WebAssembly
    </li>
    <li>Mobile use is not considered</li>
  </ul>

  <p class="mt-4 underline">Change log:</p>

  <ul class="mt-4 ml-8 list-disc">
    <li>
      <span class="font-bold">2022-07-01</span>:
      <a
        href="https://github.com/b-inary/desktop-postflop"
        class="font-bold link"
        >Desktop Postflop</a
      >
      v0.1.1 was released
    </li>
    <li>
      <span class="font-bold">2022-06-11</span>: Save/Load ranges and
      configurations
    </li>
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
