/// Returns an iterator over all the combinations of the input
///
/// # Example
///
/// ```
/// use utils::permutations;
///
/// let sets: Vec<_> = permutations(&[5, 10, 15]).collect();
/// assert_eq!(sets, vec![(&5, &10), (&5, &15), (&10, &15)]);
/// ```
pub fn permutations<T>(input: &[T]) -> impl Iterator<Item = (&T, &T)> + '_ {
    input
        .iter()
        .enumerate()
        .flat_map(|(i, a)| input[i + 1..].iter().map(move |b| (a, b)))
}
