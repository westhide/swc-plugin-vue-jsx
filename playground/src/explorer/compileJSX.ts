import { type Options, transform } from "@swc/core";

const baseTransformOptions = {
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
} satisfies Options;

export async function compileJSX(
  src: string,
  opts: Options = baseTransformOptions
) {
  const { code } = await transform(src, opts);

  return code;
}
