import { describe, it, expect } from "vitest";

import { transformSync } from "@swc/core";

function complie(src: string) {
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
    expect(complie(`const App = <h1>Hello World</h1>;`)).toMatchSnapshot();
  });
});
