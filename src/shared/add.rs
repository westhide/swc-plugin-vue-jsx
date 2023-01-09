use swc_core::ecma::{
    ast::{Expr, ExprOrSpread},
    utils::ExprFactory,
};

pub trait Add<T> {
    fn add(&mut self, target: T);
}

impl<T: Into<Expr>> Add<T> for Vec<ExprOrSpread> {
    fn add(&mut self, val: T) {
        self.push(val.as_arg())
    }
}

impl Add<ExprOrSpread> for Vec<Option<ExprOrSpread>> {
    fn add(&mut self, val: ExprOrSpread) {
        self.push(Some(val))
    }
}
