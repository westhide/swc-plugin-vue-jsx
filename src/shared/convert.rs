use crate::context::Context;

pub trait Convert<'a, T> {
    fn convert(&self, ctx: &mut impl Context<'a>) -> T;
}
