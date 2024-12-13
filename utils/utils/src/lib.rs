use std::collections::HashMap;

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

/// Returns the least common multiple
///
/// # Example
///
/// ```
/// use utils::least_common_multiple;
///
/// assert_eq!(least_common_multiple(5, 7), 35);
/// ```
pub fn least_common_multiple(first: u64, second: u64) -> u64 {
    first * second / greatest_common_divisor(first, second)
}

/// Returns the greatest common divisor
///
/// # Example
///
/// ```
/// use utils::greatest_common_divisor;
///
/// assert_eq!(greatest_common_divisor(72, 27), 9);
/// ```
pub fn greatest_common_divisor(first: u64, second: u64) -> u64 {
    let mut max = first.max(second);
    let mut min = first.min(second);

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

/// Returns the prime factors of the input and returns a hashmap with the factors and their amounts
///
/// # Example
///
/// ```
/// use utils::prime_factors;
///
/// let factors = prime_factors(315);
/// assert_eq!(factors, [(3, 2), (5, 1), (7, 1)].iter().cloned().collect());
/// ```
pub fn prime_factors(mut n: u64) -> HashMap<u64, u64> {
    let mut map = HashMap::new();

    while n % 2 == 0 {
        *map.entry(2).or_default() += 1;
        n /= 2;
    }

    let mut i = 3;
    while i <= (n as f64).sqrt() as u64 {
        while n % i == 0 {
            *map.entry(i).or_default() += 1;
            n /= i;
        }
        i += 2;
    }

    if n > 2 {
        *map.entry(n).or_default() += 1;
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            prime_factors(214154151512),
            [(2, 3), (7, 2), (2333, 1), (234167, 1)]
                .iter()
                .cloned()
                .collect()
        );
    }
}
