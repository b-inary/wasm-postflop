import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./components/App.vue";
import "./index.css";

createApp(App).use(createPinia()).mount("#app");
