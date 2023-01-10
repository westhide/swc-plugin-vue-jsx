import { describe, bench } from "vitest";

import {
  transform as babelTransform,
  transformSync as babelTransformSync,
} from "@babel/core";

import {
  type Options as SwcOptions,
  transform as swcTransform,
  transformSync as swcTransformSync,
} from "@swc/core";

const babelOptions = {
  plugins: ["@vue/babel-plugin-jsx"],
};

const swcOptions = {
  isModule: true,
  jsc: {
    target: "es2022",
    parser: {
      syntax: "ecmascript",
      jsx: true,
    },
    experimental: {
      plugins: [["@westhide/swc-plugin-vue-jsx", {}]],
    },
  },
} as SwcOptions;

const code = `const App = () => <h1>Hello World</h1>;`;

describe("transform", () => {
  bench("babel", () => {
    babelTransform(code, babelOptions);
  });
  bench("swc", () => {
    swcTransform(code, swcOptions);
  });
});

describe("transformSync", () => {
  bench("babel", () => {
    babelTransformSync(code, babelOptions);
  });
  bench("swc", () => {
    swcTransformSync(code, swcOptions);
  });
});
