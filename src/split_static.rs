use swc_helper_jsx_transform::vnode::VNode;

pub enum Item<'a> {
    VNode(&'a VNode<'a>),
    Static(&'a [VNode<'a>]),
}

pub struct Split<'a> {
    rest: &'a [VNode<'a>],
}

impl<'a> Iterator for Split<'a> {
    type Item = Item<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let Self { rest } = self;

        if rest.is_empty() {
            return None;
        }

        let mut idx = 0usize;

        for vnode in rest.iter() {
            if vnode.is_static() {
                idx += 1
            } else {
                break;
            }
        }

        let block = if idx == 0 {
            idx = 1;
            Item::VNode(&rest[0])
        } else {
            Item::Static(&rest[..idx])
        };

        *rest = &rest[idx..];

        Some(block)
    }
}

pub trait SplitStatic<'a> {
    fn split_static(&'a self) -> Split;
}

impl<'a> SplitStatic<'a> for [VNode<'a>] {
    fn split_static(&'a self) -> Split {
        Split { rest: self }
    }
}
