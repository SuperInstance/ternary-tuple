# ternary-tuple

**Fixed-length ternary vectors. Hamming distance, correlation, and algebraic operations.**

A ternary tuple is a fixed-length vector of values from {-1, 0, +1}. It's the simplest compound data structure in the ternary ecosystem — a row in a table, a word in a ternary vocabulary, a chromosome in a ternary genome.

This crate provides distance metrics, similarity measures, and algebraic operations on ternary tuples: Hamming distance, correlation, elementwise addition (mod 3), scalar multiplication, dot product, and set operations.

## What's Inside

- **`TernaryTuple`** — validated vector of {-1, 0, +1} values
- **`hamming(a, b)`** — number of positions that differ
- **`correlation(a, b)`** — Pearson correlation between two tuples
- **`elementwise_add(a, b)`** — balanced ternary addition per element (wraps mod 3)
- **`scalar_mul(tuple, n)`** — multiply each element by an integer, wrap to ternary
- **`dot(a, b)`** — inner product (sum of elementwise products)
- **`weight(tuple)`** — number of non-zero elements (the "information content")
- **`majority(tuples)`** — elementwise majority vote across multiple tuples
- **`permute(tuple, order)`** — rearrange elements by a permutation

## Quick Example

```rust
use ternary_tuple::*;

let a = TernaryTuple::new(vec![1, 0, -1, 1]);
let b = TernaryTuple::new(vec![1, 1, -1, 0]);

let h = hamming(&a, &b);
assert_eq!(h, 2); // positions 1 and 3 differ

let c = correlation(&a, &b);
// Pearson correlation between the two tuples

let sum = elementwise_add(&a, &b);
// [1+1=-1, 0+1=1, -1+-1=1, 1+0=1] (mod 3 wrapping)

let d = dot(&a, &b);
// 1*1 + 0*1 + (-1)*(-1) + 1*0 = 2

let w = weight(&a);
assert_eq!(w, 3); // 3 non-zero elements

// Majority vote across three tuples
let c = TernaryTuple::new(vec![-1, 0, 1, -1]);
let maj = majority(&[&a, &b, &c]);
```

## The Deeper Truth

**Ternary tuples are the natural data structure for Z₃ vector spaces.** Elementwise addition (mod 3) makes the set of all n-length ternary tuples into a vector space over Z₃. This isn't an analogy — it's literally true. The dimension is n, there are 3^n possible tuples, and all the linear algebra operations (addition, scalar multiplication, dot product) are well-defined and exact (no floating-point).

The Hamming distance is the L₀ norm — how many positions changed. The weight is the L₀ "norm" of a single tuple — how many positions carry information. Together, they measure the *structural difference* between tuples, which is more informative than Euclidean distance for discrete data.

**Use cases:**
- **Error-correcting codes** — ternary codes with Hamming distance guarantees
- **Genetic algorithms** — ternary chromosomes with crossover and mutation
- **Database records** — compact ternary-valued rows
- **Feature vectors** — ternary features for ML (negative/neutral/positive sentiment per dimension)
- **Cryptography** — Z₃ linear algebra for ternary ciphers

## See Also

- **ternary-matrix** — matrix operations over Z₃ (tuples are rows)
- **ternary-mutual-info** — information-theoretic distance between tuple-valued signals
- **ternary-diff** — diff operations on tuples (and sequences of tuples)
- **ternary-permutation** — permutation operations on tuple positions

## Install

```bash
cargo add ternary-tuple
```

## License

MIT
