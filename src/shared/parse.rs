pub trait Parse<T> {
    fn parse(target: T) -> Self;
}

impl<'a, T, P: Parse<&'a T>> Parse<&'a Vec<T>> for Vec<P> {
    fn parse(targets: &'a Vec<T>) -> Self {
        targets.iter().map(P::parse).collect()
    }
}
