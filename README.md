# Ternary Tuple — Operations on Fixed-Length Ternary Vectors

**Ternary Tuple** provides algebraic operations on tuples of ternary values {-1, 0, +1}: Hamming distance, correlation, Manhattan distance, orthogonality testing, complement, rotation, and subtuple extraction. These are the fundamental operations for comparing and manipulating ternary vectors.

## Why It Matters

Ternary vectors are the native data type for ternary neural networks, agent decision histories, and ternary fingerprints. Computing distances between them is essential for: nearest-neighbor search in ternary databases, similarity detection in agent populations, and error correction in ternary codes. The ternary alphabet {-1, 0, +1} gives richer distance metrics than binary: Hamming distance treats all differences equally, but Manhattan distance distinguishes "1 vs 0" (distance 1) from "1 vs -1" (distance 2). Correlation — the dot product normalized by length — directly measures agreement, making it ideal for agent similarity scoring.

## How It Works

### Hamming Distance

Number of positions where tuples differ:

```
d_H(a, b) = Σ [aᵢ ≠ bᵢ]
```

O(n) for n elements. Range: [0, n].

### Correlation

Normalized dot product:

```
corr(a, b) = (Σ aᵢ × bᵢ) / n
```

Range: [-1, +1]. Correlation = 1 means identical; -1 means perfectly anti-correlated; 0 means orthogonal. O(n).

### Manhattan Distance

Sum of absolute differences:

```
d_M(a, b) = Σ |aᵢ - bᵢ|
```

Range: [0, 2n]. Values: |1-(-1)| = 2, |1-0| = 1, |0-(-1)| = 1. O(n).

### Orthogonality

Two tuples are orthogonal if their dot product is zero:

```
orthogonal(a, b) ⟺ Σ aᵢ × bᵢ = 0
```

O(n). Orthogonal ternary vectors form the basis for ternary error-correcting codes.

### Complement

Negate each element: `complement(a)ᵢ = -aᵢ`. O(n). The complement of (+1, 0, -1) is (-1, 0, +1).

### Rotation

Circular rotation by n positions. O(n). A right rotation by 1 of (1, -1, 0) gives (0, 1, -1).

### Subtuple

Extract a contiguous slice. O(len) for the extracted length. Bounds-checked.

## Quick Start

```rust
use ternary_tuple::{TernaryTuple, hamming, correlation, manhattan, are_orthogonal};

let a = TernaryTuple::new(vec![1, 0, -1, 1]);
let b = TernaryTuple::new(vec![1, 1, -1, 0]);

println!("Hamming: {}", hamming(&a, &b));         // 2
println!("Correlation: {:.2}", correlation(&a, &b)); // 0.25
println!("Manhattan: {}", manhattan(&a, &b));      // 2
println!("Orthogonal: {}", are_orthogonal(&a, &b)); // false
```

```bash
cargo add ternary-tuple
```

## API

| Type / Function | Description |
|---|---|
| `TernaryTuple` | Vec<i8> with values in {-1, 0, +1} |
| `hamming(a, b)` | Count of differing positions (O(n)) |
| `correlation(a, b)` | Normalized dot product [-1, +1] (O(n)) |
| `manhattan(a, b)` | Sum of absolute differences (O(n)) |
| `are_orthogonal(a, b)` | True if dot product = 0 |
| `complement(t)` | Negate all elements |
| `rotate(t, n)` | Circular rotation |
| `subtuple(t, start, len)` | Contiguous slice |

## Architecture Notes

Tuple operations are the linear algebra primitives of **SuperInstance**. Agent similarity scoring uses correlation; agent diversity uses Hamming distance. The γ + η = C conservation manifests in orthogonality: orthogonal ternary vectors represent independent γ and η contributions that sum to the total capacity. See [Architecture](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## References

| MacWilliams, Florence & Sloane, Neil. *The Theory of Error-Correcting Codes*, North-Holland, 1977.
| Deza, Michel & Deza, Elena. *Encyclopedia of Distances*, 4th ed., Springer, 2016.
| Stanley, Richard. *Enumerative Combinatorics*, Cambridge UP, 2011.



## Complexity Summary

| Operation | Time | Space |
|---|---|---|
| Hamming(a, b) | O(n) | O(1) |
| Correlation(a, b) | O(n) | O(1) |
| Manhattan(a, b) | O(n) | O(1) |
| Orthogonality check | O(n) | O(1) |
| Complement | O(n) | O(n) |
| Rotation | O(n) | O(n) |
| Subtuple extraction | O(len) | O(len) |

All distance/similarity metrics are single-pass O(n) — optimal for comparing ternary vectors in high-dimensional spaces.

## License

MIT
