# ternary-tuple

A `#![forbid(unsafe_code)]` library for **ternary tuples** — vectors over the alphabet **{-1, 0, +1}** — with distance metrics, correlation, orthogonality, complement, rotation, and subtuple extraction.

## Why It Matters

Ternary tuples are the atomic data structure of the ternary ecosystem. They represent compact signed signals: each element is a *trit* carrying negative, neutral, or positive sentiment. The metrics implemented here — Hamming distance, normalized correlation, Manhattan distance — are the building blocks for comparing ternary-encoded genomes in genetic algorithms, measuring agreement between ternary agents, and defining the geometry of the ternary state space **{-1, 0, +1}ⁿ**.

The space has **3ⁿ** elements for dimension *n*, offering richer structure than binary Hamming space **{0,1}ⁿ** (2ⁿ elements) while remaining finitely enumerable.

## How It Works

### Vector Space Structure

Ternary tuples live in **ℤ₃ⁿ** (the *n*-dimensional module over the ring ℤ₃). Each tuple is a vector:

```
t = (t₀, t₁, ..., t_{n-1}),  t_i ∈ {-1, 0, +1}
```

### Distance Metrics

**Hamming distance** — count of differing positions:

```
d_H(a, b) = |{ i : a_i ≠ b_i }|
```

- **Complexity:** O(n)
- **Range:** [0, n]

**Manhattan distance** — L₁ norm of the difference:

```
d_M(a, b) = Σᵢ |a_i − b_i|
```

- **Complexity:** O(n)
- **Range:** [0, 2n] (max when one tuple is all +1 and the other all −1)

**Correlation** — normalized dot product (Pearson-like):

```
ρ(a, b) = (a · b) / n = (1/n) Σᵢ a_i · b_i
```

- **Complexity:** O(n)
- **Range:** [−1, +1]
- ρ = +1: identical tuples
- ρ = 0: orthogonal (uncorrelated)
- ρ = −1: anti-correlated

### Orthogonality

Two tuples are orthogonal when their dot product vanishes:

```
a ⊥ b  ⟺  Σᵢ a_i · b_i = 0
```

In ℤ₃ⁿ, the maximum number of pairwise orthogonal vectors of length *n* is *n* (matching the linear-algebraic dimension).

### Complement

The complement negates every element:

```
¬t = (−t₀, −t₁, ..., −t_{n−1})
```

Note: `complement(complement(t)) = t`. The complement corresponds to the additive inverse in ℤ₃ⁿ.

### Rotation

Cyclic rotation by *n* positions, with proper modular wrapping for negative shifts:

```
rotate(t, n)[i] = t[(i − n) mod len]
```

- **Complexity:** O(n)
- Handles negative rotation values correctly.

### Subtuple

Contiguous slice extraction with bounds checking:

```
subtuple(t, start, len) = (t_start, ..., t_{start+len−1})
```

## Quick Start

```rust
use ternary_tuple::*;

let a = TernaryTuple::new(vec![1, 0, -1, 1, -1]);
let b = TernaryTuple::new(vec![-1, 0, 1, 1, -1]);

assert_eq!(hamming(&a, &b), 2);           // positions 0 and 2 differ
assert_eq!(manhattan(&a, &b), 4);         // |1−(−1)| + |−1−1| = 4
assert!((correlation(&a, &b) - 0.2).abs() < 1e-9);  // dot=1, /5=0.2

let c = complement(&a);
assert_eq!(c.values, vec![-1, 0, 1, -1, 1]);

let r = rotate(&a, 1);
assert_eq!(r.values, vec![-1, 1, 0, -1, 1]);

assert!(are_orthogonal(
    &TernaryTuple::new(vec![1, -1, 0]),
    &TernaryTuple::new(vec![1, 1, 0]),
));
```

## API

| Function | Signature | Returns |
|---|---|---|
| `TernaryTuple::new` | `Vec<i8> → TernaryTuple` | Validated tuple (panics on invalid) |
| `hamming` | `(&TernaryTuple, &TernaryTuple) → usize` | Count of differing positions |
| `correlation` | `(&TernaryTuple, &TernaryTuple) → f64` | Normalized dot product ∈ [−1, +1] |
| `manhattan` | `(&TernaryTuple, &TernaryTuple) → usize` | L₁ distance |
| `are_orthogonal` | `(&TernaryTuple, &TernaryTuple) → bool` | True if dot product = 0 |
| `complement` | `&TernaryTuple → TernaryTuple` | Negated tuple |
| `rotate` | `(&TernaryTuple, isize) → TernaryTuple` | Cyclic shift |
| `subtuple` | `(&TernaryTuple, usize, usize) → TernaryTuple` | Contiguous slice |

## Architecture Notes

The correlation metric connects this crate to the **γ + η = C** conservation law of the ternary ecosystem. When ternary tuples encode agent votes (γ = +1 votes, η = −1 votes), the correlation between two tuples equals the agreement rate scaled by active participation. Two tuples with correlation ρ = 0 represent populations whose γ + η distributions are independent — the neutral state absorbs the difference.

The ℤ₃ vector space is also the natural setting for the ternary ballot operations in `warp-ternary-vote` and the tape alphabet of `ternary-turing`.

## References

- Conway, J. H. & Sloane, N. J. A. (1988). *Sphere Packings, Lattices and Groups.* Springer. — §7: Ternary and Lee-type codes.
- Hamming, R. W. (1950). *"Error Detecting and Error Correcting Codes."* Bell System Technical Journal.
- Forney, G. D. (1988). *"Coset Codes—Part II: Binary Lattices and Related Codes."* IEEE Trans. Inf. Theory.

## License

MIT
