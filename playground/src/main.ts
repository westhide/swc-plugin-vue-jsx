import { createApp } from "vue";
import { App } from "./app";
import { init } from "./explorer";

import "./style/index.css";

createApp(App).mount("#app");

init();
