import { mergeProps, resolveComponent, createVNode, createTextVNode, createStaticVNode, withDirectives, vModelText } from "vue";
const _hoisted_ = createTextVNode("123"), _hoisted_1 = [
    "frg"
], _hoisted_2 = createStaticVNode("<span ></span><span ></span><span ></span><span ></span><div ></div><div ></div>", 6), _hoisted_3 = [
    "clalang"
], _hoisted_4 = createVNode("div", null, null, -1);
const tmpl = (()=>{
    const _v = resolveComponent("A");
    return withDirectives(createVNode("div", mergeProps(c, {
        clalang: a,
        "onUpdate:modelValue": ($v)=>b = $v
    }), [
        createVNode(_v, null, null),
        _hoisted_,
        d,
        ...e,
        createVNode("div", {
            frg: _hoisted_4
        }, null, 8, _hoisted_1),
        _hoisted_2
    ], 536, _hoisted_3), [
        [
            vModelText,
            b
        ]
    ]);
})();
