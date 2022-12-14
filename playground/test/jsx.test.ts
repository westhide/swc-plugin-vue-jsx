import {describe, expect, it} from "vitest";

import {transformSync} from "@swc/core";

function compile(src: string) {
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
}

describe("SWC Plugin Vue JSX", () => {
    it("hello world", () => {
        expect(compile(`const App = <h1>Hello World</h1>;`)).toMatchSnapshot();
    });
});
