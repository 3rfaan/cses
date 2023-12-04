# TWO KNIGHTS

| var                | desc                     |
| ------------------ | ------------------------ |
| $n$                | Size of $k$              |
| $k$                | No. of rows / cols       |
| $k \times k = k^2$ | Dimensions of chessboard |

## Knights

Knight 1: $k^2$

Knight 2: $k^2 - 1$

First knight can be on any position of chessboard.
Second knight can be on any position except the position of the first knight, so we subtract 1.

$$k^2(k^2 - 1)$$

Then we remove duplicate cases where knights position can be swapped, because we don't distinguish between them and don't want to double-count them.

$$\frac {k^2(k^2 - 1)}{2}$$

## Attack Fields

We found out all the possible positions where knights can be placed. Now we have to find out where the knights can't be placed.

Knights can't be placed when they are in an attack position consisting of a 2/3 block or a 3/2 block.

In a chessboard of size $k \times k$ there will fit $(k - 1) \times (k - 2)$ blocks of size 2/3 and $(k - 2) \times (k - 1)$ of size 3/2. So for example in a chessboard with dimensions of $3 \times 3$ there fits $2 \times 1$ 2/3 blocks and $1 \times 2$ 3/2 blocks.

$$(k - 1) \times (k - 2) \times (k - 2) \times (k - 1)$$
$$= 2 \times (k - 1) \times (k - 2)$$

In each block the knight could be in 2 positions where it would attack the other knight, so we have to multiply the above formula by 2:

$$4 \times (k - 2) \times (k - 1)$$

So in the end the complete formula to solve this problem is:

$$f(k) = \frac{k^2 \times (k^2 - 1)}{2} - 4 \times (k - 2) \times (k - 1)$$

## Code

In Rust 🦀 code:

```rust
fn main() {
    let n = std::io::read_to_string(std::io::stdin())
        .unwrap()
        .trim()
        .parse::<i64>()
        .unwrap();

    (1..=n).for_each(|k| {
        println!(
            "{}",
            (k.pow(2) * (k.pow(2) - 1) / 2) - (4 * (k - 2) * (k - 1))
        );
    });
}
```
