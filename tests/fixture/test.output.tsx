// comment debug
import { mergeProps, create_vnode } from "vue";

const tmpl = create_vnode("div", mergeProps(b, c, {
    onClickl: a
}));
