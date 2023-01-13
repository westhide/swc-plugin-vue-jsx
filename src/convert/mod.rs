use crate::context::Context;

mod element;
mod fragment;
mod patch_flag;
mod split_static;
mod text;
mod vnode;

pub trait Convert<T> {
    fn convert<C: Context>(&self, ctx: &mut C) -> T;
}
