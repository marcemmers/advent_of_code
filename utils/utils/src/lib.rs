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

pub fn f64_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.0001f64
}

pub fn f64_is_integer(val: f64) -> bool {
    f64_equal(val, val.round())
}

/// Used to solve linear equations and finds the factors to solve it
///
/// # Example
///
/// ```
/// use utils::gaussian_elimination;
///
/// let input = vec![vec![ 2f64,  1f64, -1f64,   8f64],
///                  vec![-3f64, -1f64,  2f64, -11f64],
///                  vec![-2f64,  1f64,  2f64,  -3f64]];
/// let result = gaussian_elimination(&input);
/// assert_eq!(result.iter().map(|f| f.round()).collect::<Vec<_>>(), [2f64, 3f64, -1f64]);
/// ```
pub fn gaussian_elimination(matrix: &[Vec<f64>]) -> Vec<f64> {
    let mut matrix: Vec<Vec<_>> = matrix.iter().map(|row| row.to_vec()).collect();

    assert_ne!(matrix.len(), 0);
    assert_ne!(matrix[0].len(), 0);
    let m = matrix.len();
    let n = matrix[0].len();
    assert_eq!(m, n - 1);

    let mut h = 0;
    let mut k = 0;

    while h < m && k < n {
        let i_max = (h..m)
            .map(|i| (i, matrix[i][k].abs()))
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(i, _)| i)
            .unwrap();

        if matrix[i_max][k] == 0f64 {
            k += 1;
        } else {
            let copy = matrix[i_max].clone();
            matrix[i_max] = matrix[h].clone();
            matrix[h] = copy;

            for i in (h + 1)..m {
                let f = matrix[i][k] / matrix[h][k];
                matrix[i][k] = 0f64;
                for j in (k + 1)..n {
                    matrix[i][j] -= matrix[h][j] * f;
                }
            }
            h += 1;
            k += 1;
        }
    }

    for i in (1..m).rev() {
        if matrix[i][i] != 0f64 {
            for j in (0..i).rev() {
                let f = matrix[j][i] / matrix[i][i];
                for k in (0..n).rev() {
                    matrix[j][k] -= f * matrix[i][k];
                }
            }
        }
    }

    (0..m).map(|i| matrix[i][m] / matrix[i][i]).collect()
}

/// Used to solve linear equations and finds the factors to solve it, converts to integers and return None if the conversion failed
///
/// # Example
///
/// ```
/// use utils::gaussian_elimination_int;
///
/// let input = vec![vec![ 2, 4, 8],
///                  vec![ 4, 2, 10]];
/// let result = gaussian_elimination_int(&input);
/// assert_eq!(result, Some(vec![2, 1]));
///
/// let input = vec![vec![ 2, 3, 8],
///                  vec![ 3, 3, 8]];
/// let result = gaussian_elimination_int(&input);
/// assert_eq!(result, None);
/// ```
pub fn gaussian_elimination_int(matrix: &[Vec<i64>]) -> Option<Vec<i64>> {
    let matrix: Vec<Vec<_>> = matrix
        .iter()
        .map(|row| row.iter().map(|v| *v as f64).collect())
        .collect();
    let result = gaussian_elimination(&matrix);

    if result.iter().all(|f| f64_is_integer(*f)) {
        Some(result.iter().map(|f| f.round() as i64).collect())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_factors() {
        assert_eq!(
            prime_factors(214154151512),
            [(2, 3), (7, 2), (2333, 1), (234167, 1)]
                .iter()
                .cloned()
                .collect()
        );
    }
}
