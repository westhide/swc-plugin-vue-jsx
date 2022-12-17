use crate::shared::state::State;

pub trait Convert<'s, T> {
    fn convert<S: State<'s>>(&self, state: &mut S) -> T;
}

impl<'s, T: Convert<'s, U>, U> Convert<'s, Vec<U>> for [T] {
    fn convert<S: State<'s>>(&self, state: &mut S) -> Vec<U> {
        self.iter().map(|item| item.convert(state)).collect()
    }
}

// impl<'s, T: Convert<'s, U>, U> Convert<'s, Option<U>> for Option<T> {
//     fn convert<S: State<'s>>(&self, state: &mut S) -> Option<U> {
//         self.as_ref().map(|item| item.convert(state))
//     }
// }
