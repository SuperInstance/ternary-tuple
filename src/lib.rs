#![forbid(unsafe_code)]

#[derive(Clone, Debug, PartialEq)]
pub struct TernaryTuple {
    pub values: Vec<i8>,
}

impl TernaryTuple {
    pub fn new(values: Vec<i8>) -> Self {
        for &v in &values {
            assert!(v >= -1 && v <= 1, "values must be in {{-1, 0, 1}}");
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
    a.values.iter().zip(&b.values).filter(|(x, y)| x != y).count()
}

pub fn correlation(a: &TernaryTuple, b: &TernaryTuple) -> f64 {
    assert_eq!(a.len(), b.len(), "tuples must be same length");
    if a.is_empty() {
        return 0.0;
    }
    let dot: i64 = a.values.iter().zip(&b.values).map(|(x, y)| (x * y) as i64).sum();
    dot as f64 / a.len() as f64
}

pub fn manhattan(a: &TernaryTuple, b: &TernaryTuple) -> usize {
    assert_eq!(a.len(), b.len(), "tuples must be same length");
    a.values.iter().zip(&b.values).map(|(x, y)| (x - y).unsigned_abs() as usize).sum()
}

pub fn are_orthogonal(a: &TernaryTuple, b: &TernaryTuple) -> bool {
    assert_eq!(a.len(), b.len(), "tuples must be same length");
    let dot: i64 = a.values.iter().zip(&b.values).map(|(x, y)| (x * y) as i64).sum();
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
    TernaryTuple { values: t.values[start..start + len].to_vec() }
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
}
