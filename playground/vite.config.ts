import {createFilter, defineConfig} from "vite";
import {resolve} from "path";

import {transformSync} from "@swc/core";

import Inspect from "vite-plugin-inspect";

const filter = createFilter(/\.[jt]sx$/);


export default defineConfig({
    resolve: {
        alias: {
            "@": `${resolve(__dirname, "src")}/`,
        },
    },
    plugins: [Inspect(), {

        name: "swc-plugin-vue-jsx",
        enforce: "pre",

        async transform(source, id) {
            if (filter(id)) {
                return transformSync(source, {
                    isModule: true,
                    jsc: {
                        target: "es2022",
                        parser: {
                            syntax: "typescript",
                            tsx: true,
                            decorators: true,
                            dynamicImport: true,
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


    }],
});
