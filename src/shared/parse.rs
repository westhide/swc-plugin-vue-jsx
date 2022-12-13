pub trait Parse<T> {
    fn parse(target: T) -> Self;
}

impl<'a, T, P: Parse<&'a T>> Parse<&'a [T]> for Vec<P> {
    fn parse(targets: &'a [T]) -> Self {
        targets.iter().map(P::parse).collect()
    }
}
