# COIN PILES

| var   | desc                           |
| ----- | ------------------------------ |
| $t$   | Number of tests                |
| $a,b$ | Number of coins in $A$ and $B$ |

We have two coin piles $A$ and $B$. On each move we have two options:

1. Remove one coin from $A$ and two coins from $B$
1. Remove two coins from $A$ and one coin from $B$

## Divisible by 3

So the total amount of coins removed in one turn is 3 (either $1 + 2$ or $2 + 1$). This means the total amount of coins in $A$ and $B$ must be divisible by 3.

$$A + B\mod 3 = 0$$

## Ratio bigger pile to smaller pile

Also, the bigger of the two piles must not be greater than two times the smaller pile. Because in each turn we take two from one pile and one from the other, so the ratio is 2:1.

$$2 \times min\{A, B\} \ge max\{A, B\}$$

So, let's consider the example where $A = 3$ and $B = 6$.

$$A + B = 3 + 6 = 9 \mod 3 = 0$$

So the total amount of coins in both piles is divisible by three. Then we have to check that the bigger pile $B$ is not greater than $2 \times A$:

$$2 \times A \ge B \equiv 2 \times 3 \ge 6 \equiv 6 \ge 6$$

In this case $2 \times A$ is exactly $B$. That means $2 \times A \ge B$ in this case is $6 \ge 6$ which evaluates to `true`.

So if we have input $A = 3$ and $B = 6$ the program output will be `YES`.

## Code

In Rust 🦀 code:

```rust
fn main() {
    let inp: Vec<(u64, u64)> = std::io::read_to_string(std::io::stdin())
        .unwrap()
        .lines()
        .skip(1)
        .flat_map(|s| s.split_once(' '))
        .map(|(a, b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
        .collect();

    for (a, b) in inp {
        if (a + b) % 3 == 0 && 2 * a.min(b) >= a.max(b) { println!("YES"); }
        else { println!("NO"); }
    }
}
```
