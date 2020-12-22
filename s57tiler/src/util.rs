
pub fn compare<T: Eq>(lhs: &[T], rhs: &[T]) -> bool {
    (lhs.len() == rhs.len()) &&
        lhs.iter()
            .zip(rhs)
            .all(|(a, b)| a == b)
}
