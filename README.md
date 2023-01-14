# SWC Plugin for Vue JSX

[![npm package](https://img.shields.io/npm/v/@westhide/swc-plugin-vue-jsx.svg)](https://www.npmjs.com/package/@westhide/swc-plugin-vue-jsx)

`Effective`,`Flexible`,`Intelligent` Vue JSX

## Install

pnpm

```bash
pnpm install @westhide/swc-plugin-vue-jsx -D
```

## Usage

### SWC

```ts
import { transform } from "@swc/core";

export type PluginOptions = {
  // staticVNodes above threshold will compile to html
  staticThreshold?: number /* default=5 */;
  // regexs match custom element tag
  customElementPatterns?: string[];
};

transform(src, {
  isModule: true,
  jsc: {
    target: "es2022",
    parser: {
      syntax: "typescript",
      tsx: true,
    },
    experimental: {
      plugins: [["@westhide/swc-plugin-vue-jsx", {} as PluginOptions]],
    },
  },
});
```

### Vite

> pnpm install [@westhide/vite-plugin-vue-jsx-swc](https://github.com/westhide/vite-plugin-vue-jsx-swc)
> -D

<details>
    <summary>e.g.</summary>

```ts
import { type Plugin, createFilter } from "vite";
import { transform } from "@swc/core";

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
        return transform(src, {
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
```

## </details>

_Now you can use JSX in Vue Component !_

```jsx
import { defineComponent } from "vue";

export const App = defineComponent({
  setup() {
    return () => <h1>SWC Plugin Vue JSX</h1>;
  },
});
```

<details>
<summary>explore</summary>

```js
import { defineComponent } from "vue";
import { createTextVNode, createElementVNode } from "vue";

const _hoisted_ = createTextVNode("SWC Plugin Vue JSX"),
  _hoisted_1 = createElementVNode("h1", null, [_hoisted_], -1);

export const App = defineComponent({
  setup() {
    return () => _hoisted_1;
  },
});
```

</details>

## Syntax

### Tag

#### native

```jsx
const vnode = <div></div>;
```

<details>
<summary>explore</summary>

```js
import { createElementVNode } from "vue";

const _hoisted_ = createElementVNode("div", null, null, -1);

const vnode = _hoisted_;
```

</details>

#### componet

```jsx
import { A } from "./componets";

const vnode = <A></A>;
```

<details>
<summary>explore</summary>

```js
import { A } from "./componets";
import { createVNode } from "vue";

const vnode = createVNode(A, null, null);
```

</details>

#### resolveComponet

```jsx
const vnode = <A></A>;
```

<details>
<summary>explore</summary>

```js
import { resolveComponent, createVNode } from "vue";

const vnode = (() => {
  const _v = resolveComponent("A");
  return createVNode(_v, null, null);
})();
```

</details>

#### member

```jsx
const vnode = <A.b></A.b>;
```

<details>
<summary>explore</summary>

```js
import { createVNode } from "vue";

const vnode = createVNode(A.b, null, null);
```

</details>

#### custom

```jsx
// customElementPatterns: ["custom-tag"]

const vnode = <custom-tag></custom-tag>;
```

<details>
<summary>explore</summary>

```js
import { createVNode } from "vue";

const vnode = createVNode("custom-tag", null, null);
```

</details>

### Attribute / Prop

#### literal

```jsx
const vnode = <div class="box"></div>;
```

<details>
<summary>explore</summary>

```js
import { createElementVNode } from "vue";

const _hoisted_ = createElementVNode("div", { class: "box" }, null, -1);

const vnode = _hoisted_;
```

</details>

#### boolean attribute

```jsx
const vnode = <input autofocus />;
```

<details>
<summary>explore</summary>

```js
import { createElementVNode } from "vue";

const _hoisted_ = createElementVNode("input", { autofocus: "" }, null, -1);

const vnode = _hoisted_;
```

</details>

#### binding

```jsx
const vnode = <div class={a}></div>;
```

<details>
<summary>explore</summary>

```js
import { createVNode } from "vue";

const vnode = createVNode("div", { class: a }, null, 2);
```

</details>

#### spread / mergeProps

```jsx
const vnode = <div {...a} class="box"></div>;
```

<details>
<summary>explore</summary>

```js
import { mergeProps, createVNode } from "vue";

const vnode = createVNode("div", mergeProps(a, { class: "box" }), null, 16);
```

</details>

### Directive

#### v-text

```jsx
const vnode = <div v-text="msg"></div>;
```

<details>
<summary>explore</summary>

```js
import { createVNode } from "vue";

const vnode = createVNode("div", { textContent: "msg" }, null);
```

</details>

#### v-html

```jsx
const vnode = <div v-html="<span>hello</span>"></div>;
```

<details>
<summary>explore</summary>

```js
import { createVNode } from "vue";

const vnode = createVNode("div", { innerHTML: "<span>hello</span>" }, null);
```

</details>

#### v-show

```jsx
const vnode = <div v-show={isShow}></div>;
```

<details>
<summary>explore</summary>

```js
import { createVNode, vShow, withDirectives } from "vue";

const vnode = withDirectives(createVNode("div", null, null, 512), [
  [vShow, isShow],
]);
```

</details>

#### v-model<sup>`wip`</sup>

```jsx
const vnode = <input v-model={val} />;
```

<details>
<summary>explore</summary>

```js
import { createVNode, vModelText, withDirectives } from "vue";

const vnode = withDirectives(
  createVNode(
    "input",
    { "onUpdate:modelValue": ($v) => (val = $v) },
    null,
    512
  ),
  [[vModelText, val]]
);
```

</details>

_v-model with arguments_

```jsx
const vnode = <A v-model:title={val}></A>;
```

<details>
<summary>explore</summary>

```js
import { resolveComponent, createVNode } from "vue";

const vnode = (() => {
  const _v = resolveComponent("A");
  return createVNode(
    _v,
    { title: val, "onUpdate:title": ($v) => (val = $v) },
    null,
    8,
    ["title"]
  );
})();
```

</details>

#### custom

```jsx
const vnode = <div v-custom={val}></div>;
```

<details>
<summary>explore</summary>

```js
import { createVNode, resolveDirective, withDirectives } from "vue";

const vnode = (() => {
  const _v = resolveDirective("custom");
  return withDirectives(createVNode("div", null, null, 512), [[_v, val]]);
})();
```

</details>

### Slot<sup>`wip`</sup>

#### v-slots

```jsx
// const A = (_props, { slots }) => (
//   <>
//     <h1></h1>
//     <h2>{slots.bar?.()}</h2>
//   </>
// );

const vnode = <A v-slots={slots}></A>;
```

<details>
<summary>explore</summary>

```js
import { resolveComponent, createVNode } from "vue";

const vnode = (() => {
  const _v = resolveComponent("A");
  return createVNode(_v, null, slots, 1024);
})();
```

</details>

## Features

### [Patch Flags](https://vuejs.org/guide/extras/rendering-mechanism.html#patch-flags)

> mark dynamic VNode information at compile time

### [Static Hoisting](https://vuejs.org/guide/extras/rendering-mechanism.html#static-hoisting)

- hoist static VNode
- turn consecutive static VNode to html template

```jsx
// staticThreshold = 5
const tmpl_vnode = (
  <>
    <div>1</div>
    <div>2</div>
    <div>3</div>
    <div>4</div>
    <div>5</div>
  </>
);
```

<details>
<summary>explore</summary>

```js
import { Fragment, createStaticVNode, createVNode } from "vue";

const _hoisted_ = createStaticVNode(
  "<div >1</div><div >2</div><div >3</div><div >4</div><div >5</div>",
  5
);

const tmpl_vnode = createVNode(Fragment, null, [_hoisted_]);
```

</details>

### [Tree Flattening](https://vuejs.org/guide/extras/rendering-mechanism.html#tree-flattening)

> ElementBlock <sup>`TODO`</sup>

## Motive

- Refactor [@vue/babel-plugin-jsx](https://github.com/vuejs/babel-plugin-jsx) by SWC
- Build Compiler-Informed Virtual DOM with JSX
