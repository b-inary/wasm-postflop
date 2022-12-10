import { createApp } from "vue";
import { createPinia } from "pinia";
import FloatingVue from "floating-vue";
import App from "./components/App.vue";

import "./index.css";
import "floating-vue/dist/style.css";

FloatingVue.options.disposeTimeout = 200;

createApp(App).use(createPinia()).use(FloatingVue).mount("#app");
