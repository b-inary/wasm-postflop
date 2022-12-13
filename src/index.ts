import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./components/App.vue";
import "./index.css";
import "tippy.js/dist/tippy.css";

createApp(App).use(createPinia()).mount("#app");
