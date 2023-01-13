use swc_helper_jsx_transform::vnode::VNode;

pub enum Block<'a> {
    VNode(&'a VNode<'a>),
    Static(&'a [VNode<'a>]),
}

pub struct Split<'a> {
    rest: &'a [VNode<'a>],
}

impl<'a> Iterator for Split<'a> {
    type Item = Block<'a>;

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

        let item = if idx == 0 {
            idx = 1;
            Block::VNode(&rest[0])
        } else {
            Block::Static(&rest[..idx])
        };

        *rest = &rest[idx..];

        Some(item)
    }
}

pub trait SplitStatic {
    fn split_static(&self) -> Split;
}

impl<'a> SplitStatic for [VNode<'a>] {
    fn split_static(&self) -> Split {
        Split { rest: self }
    }
}

pub trait StaticContent {
    fn static_content(&self) -> String;
}

impl<'a> StaticContent for [VNode<'a>] {
    fn static_content(&self) -> String {
        self.iter().map(VNode::static_content).collect()
    }
}
