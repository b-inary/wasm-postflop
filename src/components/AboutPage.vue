<template>
  <div class="flex">
    <div
      class="flex pl-2.5 pr-3 py-1 text-cyan-600 bg-cyan-50 border-2 border-cyan-600 rounded-md font-semibold"
    >
      <InformationCircleIcon class="inline w-5 h-5 mt-[0.1875rem] mr-1.5" />
      <div>
        This website will no longer be updated. Please see
        <a
          href="https://github.com/b-inary/postflop-solver/issues/46"
          class="link"
          target="_blank"
          >this GitHub issue</a
        >
        for more information.<br />
        If you try to find similar tools, please beware of malicious fake sites.
      </div>
    </div>
  </div>

  <p class="mt-4">
    <span class="font-bold">WASM Postflop</span> is a free, open-source
    <span class="font-bold">GTO solver</span> that works on web browsers.<br />
    <span class="font-bold">WASM</span> stands for
    <a href="https://webassembly.org/" class="font-bold link" target="_blank"
      >WebAssembly</a
    >. It is a modern technology that allows us to distribute a fast, portable,
    and secure program across the web.
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
    <a
      href="https://github.com/b-inary/wasm-postflop"
      class="link"
      target="_blank"
      >GitHub</a
    >.
  </p>

  <p class="mt-4 underline">See also:</p>

  <ul class="mt-4 ml-8 list-disc">
    <li>
      <a
        href="https://github.com/b-inary/desktop-postflop"
        class="font-bold link"
        target="_blank"
        >Desktop Postflop</a
      >: A native desktop application for Windows and Linux. Faster and no
      memory limitations.
    </li>
  </ul>

  <p class="mt-4 underline">Limitations:</p>

  <ul class="mt-4 ml-8 list-disc">
    <li>
      <span class="font-bold underline"
        >Multithreading is not supported on iOS and Safari</span
      >
      because nested workers are
      <a
        href="https://bugs.webkit.org/show_bug.cgi?id=22723"
        class="link"
        target="_blank"
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
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { InformationCircleIcon } from "@heroicons/vue/20/solid";

export default defineComponent({
  components: { InformationCircleIcon },
});
</script>

<style scoped>
.link {
  @apply text-blue-500 hover:underline;
}
</style>
