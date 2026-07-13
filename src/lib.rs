#![forbid(unsafe_code)]

#[derive(Clone, Debug, PartialEq)]
pub struct TernaryTuple {
    pub values: Vec<i8>,
}

impl TernaryTuple {
    pub fn new(values: Vec<i8>) -> Self {
        for &v in &values {
            assert!((-1..=1).contains(&v), "values must be in {{-1, 0, 1}}");
        }
        Self { values }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

pub fn hamming(a: &TernaryTuple, b: &TernaryTuple) -> usize {
    assert_eq!(a.len(), b.len(), "tuples must be same length");
    a.values
        .iter()
        .zip(&b.values)
        .filter(|(x, y)| x != y)
        .count()
}

pub fn correlation(a: &TernaryTuple, b: &TernaryTuple) -> f64 {
    assert_eq!(a.len(), b.len(), "tuples must be same length");
    if a.is_empty() {
        return 0.0;
    }
    let dot: i64 = a
        .values
        .iter()
        .zip(&b.values)
        .map(|(x, y)| (x * y) as i64)
        .sum();
    dot as f64 / a.len() as f64
}

pub fn manhattan(a: &TernaryTuple, b: &TernaryTuple) -> usize {
    assert_eq!(a.len(), b.len(), "tuples must be same length");
    a.values
        .iter()
        .zip(&b.values)
        .map(|(x, y)| (x - y).unsigned_abs() as usize)
        .sum()
}

pub fn are_orthogonal(a: &TernaryTuple, b: &TernaryTuple) -> bool {
    assert_eq!(a.len(), b.len(), "tuples must be same length");
    let dot: i64 = a
        .values
        .iter()
        .zip(&b.values)
        .map(|(x, y)| (x * y) as i64)
        .sum();
    dot == 0
}

pub fn complement(t: &TernaryTuple) -> TernaryTuple {
    let values = t.values.iter().map(|&v| -v).collect();
    TernaryTuple { values }
}

pub fn rotate(t: &TernaryTuple, n: isize) -> TernaryTuple {
    if t.is_empty() {
        return t.clone();
    }
    let len = t.len();
    let shift = ((n % len as isize) + len as isize) as usize % len;
    let mut values = Vec::with_capacity(len);
    values.extend_from_slice(&t.values[len - shift..]);
    values.extend_from_slice(&t.values[..len - shift]);
    TernaryTuple { values }
}

pub fn subtuple(t: &TernaryTuple, start: usize, len: usize) -> TernaryTuple {
    assert!(start + len <= t.len(), "subtuple out of bounds");
    TernaryTuple {
        values: t.values[start..start + len].to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid() {
        let t = TernaryTuple::new(vec![-1, 0, 1]);
        assert_eq!(t.values, vec![-1, 0, 1]);
    }

    #[test]
    #[should_panic]
    fn test_new_invalid() {
        TernaryTuple::new(vec![2]);
    }

    #[test]
    fn test_hamming_same() {
        let a = TernaryTuple::new(vec![1, 0, -1]);
        assert_eq!(hamming(&a, &a), 0);
    }

    #[test]
    fn test_hamming_diff() {
        let a = TernaryTuple::new(vec![1, 0, -1]);
        let b = TernaryTuple::new(vec![-1, 0, 1]);
        assert_eq!(hamming(&a, &b), 2);
    }

    #[test]
    fn test_correlation_identical() {
        let a = TernaryTuple::new(vec![1, 1, 1]);
        assert!((correlation(&a, &a) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_correlation_orthogonal() {
        let a = TernaryTuple::new(vec![1, -1]);
        let b = TernaryTuple::new(vec![1, 1]);
        assert!((correlation(&a, &b) - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_manhattan() {
        let a = TernaryTuple::new(vec![1, 0, -1]);
        let b = TernaryTuple::new(vec![-1, 0, 1]);
        assert_eq!(manhattan(&a, &b), 4);
    }

    #[test]
    fn test_are_orthogonal_true() {
        let a = TernaryTuple::new(vec![1, -1, 0]);
        let b = TernaryTuple::new(vec![1, 1, 0]);
        assert!(are_orthogonal(&a, &b));
    }

    #[test]
    fn test_are_orthogonal_false() {
        let a = TernaryTuple::new(vec![1, 0, 0]);
        let b = TernaryTuple::new(vec![1, 0, 0]);
        assert!(!are_orthogonal(&a, &b));
    }

    #[test]
    fn test_complement() {
        let t = TernaryTuple::new(vec![1, 0, -1]);
        let c = complement(&t);
        assert_eq!(c.values, vec![-1, 0, 1]);
    }

    #[test]
    fn test_rotate_one() {
        let t = TernaryTuple::new(vec![1, 0, -1]);
        let r = rotate(&t, 1);
        assert_eq!(r.values, vec![-1, 1, 0]);
    }

    #[test]
    fn test_rotate_zero() {
        let t = TernaryTuple::new(vec![1, 0, -1]);
        assert_eq!(rotate(&t, 0), t);
    }

    #[test]
    fn test_subtuple() {
        let t = TernaryTuple::new(vec![1, 0, -1, 1]);
        let s = subtuple(&t, 1, 2);
        assert_eq!(s.values, vec![0, -1]);
    }

    #[test]
    fn test_empty_tuple() {
        let t = TernaryTuple::new(vec![]);
        assert!(t.is_empty());
        assert_eq!(hamming(&t, &t), 0);
    }

    #[test]
    fn test_rotate_negative() {
        let t = TernaryTuple::new(vec![1, 0, -1]);
        let r = rotate(&t, -1);
        assert_eq!(r.values, vec![0, -1, 1]);
    }

    // ---- additional coverage for real, previously-untested branches ----

    #[test]
    fn test_new_invalid_negative() {
        // the lower bound of the {-1, 0, 1} alphabet is enforced too
        let result = std::panic::catch_unwind(|| TernaryTuple::new(vec![-2]));
        assert!(result.is_err(), "value -2 must be rejected");
    }

    #[test]
    #[should_panic(expected = "subtuple out of bounds")]
    fn test_subtuple_out_of_bounds() {
        let t = TernaryTuple::new(vec![1, 0, -1]); // len 3
        subtuple(&t, 1, 3); // start(1) + len(3) = 4 > 3
    }

    #[test]
    fn test_correlation_empty() {
        // empty is a special-cased branch returning 0.0 (avoids div-by-zero)
        let t = TernaryTuple::new(vec![]);
        assert_eq!(correlation(&t, &t), 0.0);
    }

    #[test]
    fn test_correlation_anticorrelated() {
        // README documents rho = -1 for anti-correlated tuples
        let a = TernaryTuple::new(vec![1, 1, -1]);
        let b = TernaryTuple::new(vec![-1, -1, 1]);
        assert!((correlation(&a, &b) - (-1.0)).abs() < 1e-9);
    }

    #[test]
    #[should_panic(expected = "tuples must be same length")]
    fn test_hamming_mismatched_length() {
        hamming(&TernaryTuple::new(vec![1, 0]), &TernaryTuple::new(vec![1]));
    }

    #[test]
    #[should_panic(expected = "tuples must be same length")]
    fn test_manhattan_mismatched_length() {
        manhattan(&TernaryTuple::new(vec![1]), &TernaryTuple::new(vec![1, 0]));
    }

    #[test]
    #[should_panic(expected = "tuples must be same length")]
    fn test_correlation_mismatched_length() {
        correlation(&TernaryTuple::new(vec![1, 0]), &TernaryTuple::new(vec![1]));
    }

    #[test]
    #[should_panic(expected = "tuples must be same length")]
    fn test_are_orthogonal_mismatched_length() {
        are_orthogonal(&TernaryTuple::new(vec![1]), &TernaryTuple::new(vec![1, 0]));
    }

    #[test]
    fn test_complement_involution() {
        // README: complement(complement(t)) = t
        let t = TernaryTuple::new(vec![1, 0, -1, 1]);
        assert_eq!(complement(&complement(&t)).values, t.values);
    }

    #[test]
    fn test_rotate_identity_full_length() {
        // rotating by exactly len is the identity
        let t = TernaryTuple::new(vec![1, 0, -1, 1]);
        assert_eq!(rotate(&t, t.len() as isize).values, t.values);
    }

    #[test]
    fn test_rotate_wraparound_positive() {
        // rotate(t, len + k) == rotate(t, k)
        let t = TernaryTuple::new(vec![1, 0, -1, 1]);
        let len = t.len() as isize;
        assert_eq!(rotate(&t, len + 1).values, rotate(&t, 1).values);
    }

    #[test]
    fn test_rotate_wraparound_negative() {
        // rotate(t, -(len + 1)) == rotate(t, -1).
        // This exercises the inner `% len` in the Euclidean reduction: dropping
        // it makes `n + len` negative, which wraps on the `as usize` cast and
        // yields the wrong shift. An *odd* length is used deliberately: when len
        // is a power of two, `2^64 ≡ 0 (mod len)`, so `(-1isize as usize) % len`
        // happens to equal `len - 1` (the correct shift) and masks the bug.
        // len = 5 is coprime-ish (odd), so MAX % 5 == 0 != 4 == (-1) mod 5.
        let t = TernaryTuple::new(vec![1, 0, -1, 1, 0]);
        let len = t.len() as isize;
        assert_eq!(rotate(&t, -(len + 1)).values, rotate(&t, -1).values);
    }

    #[test]
    fn test_rotate_composition() {
        // group law: rotate(rotate(t, n), m) == rotate(t, n + m)
        let t = TernaryTuple::new(vec![1, 0, -1, 1, 0]);
        for n in -7..=7isize {
            for m in -7..=7isize {
                assert_eq!(
                    rotate(&rotate(&t, n), m).values,
                    rotate(&t, n + m).values,
                    "rotate composition failed for n={n}, m={m}"
                );
            }
        }
    }
}
