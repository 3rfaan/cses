# TWO SETS

| var | desc                 |
| --- | -------------------- |
| $n$ | Range $1, 2, ..., n$ |

## Sum of range

Formula for sum of all integers in range $1, 2, ..., n$:

$$sum = \frac {n \times (n + 1)}{2}$$

## Sum of subsets

For two subsets we need to divide the sum by 2:

$$\frac {sum}{2}$$

For example for $n = 7$ the sum of all integers would be:

$$\frac {7 \times (7 + 1)}{2} = 28$$

The sum of each subset should then be

$$\frac {28}{2} = 14$$

## Distribute integers in range

Then we check for each integer $i$ in range $n, .., 2, 1$ if the integer $i$ is smaller than or equal to the sum of the subset, e. g. 14.

If the integer is smaller or equal then we push it to the first array and subtract from the subset sum the integer, e. g. $14 - i$.

If $i$ is not smaller or equal to subset sum, we push $i$ to the second array.

In the end we have two arrays containing integers which add up to the same sum.

## Code

In Rust 🦀 code:

```rust
``fn main() {
    let n: u64 = std::io::read_to_string(std::io::stdin())
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    let (mut a, mut b) = (Vec::new(), Vec::new());
    let mut sum = n * (n + 1) / 2;

    if sum % 2 != 0 { println!("NO"); return; }
    sum /= 2;

    for i in (1..=n).rev() {
        if i <= sum { sum -= i; a.push(i); }
        else { b.push(i) }
    }
    println!("YES\n{}", a.len());
    a.iter().for_each(|e| print!("{e} "));
    println!("\n{}", b.len());
    b.iter().for_each(|e| print!("{e} "));
    println!();
}`
```
