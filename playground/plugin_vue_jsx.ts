import { type Plugin, createFilter } from "vite";
import { transformSync } from "@swc/core";

export default function () {
  const filter = createFilter(/\.[jt]sx$/);

  return {
    name: "vite-plugin-vue-jsx",

    config() {
      return {
        esbuild: {
          include: /\.ts$/,
        },
        define: {
          __VUE_OPTIONS_API__: true,
          __VUE_PROD_DEVTOOLS__: false,
        },
      };
    },

    async transform(src, id) {
      if (filter(id)) {
        return transformSync(src, {
          isModule: true,
          jsc: {
            target: "es2022",
            parser: {
              syntax: "typescript",
              tsx: true,
            },
            experimental: {
              plugins: [["@westhide/swc-plugin-vue-jsx", {}]],
            },
          },
        });
      } else {
        return null;
      }
    },
  } as Plugin;
}
