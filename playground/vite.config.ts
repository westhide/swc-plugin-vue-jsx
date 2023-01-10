import { defineConfig } from "vite";
import { resolve } from "path";

import Inspect from "vite-plugin-inspect";
import VueJSX from "./plugin_vue_jsx";

export default defineConfig({
  resolve: {
    alias: {
      "@": `${resolve(__dirname, "src")}/`,
    },
  },
  plugins: [Inspect(), VueJSX()],
});
