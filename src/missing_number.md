# MISSING NUMBER

| var | desc                 |
| --- | -------------------- |
| $n$ | Range $1, 2, ..., n$ |

As an input we get $n$ which is the last digit in a range from $1, 2, ..., n$. Then as second input we get the digits of this range except one of the digits. Our goal is to find out which digit is missing from the range.

The formula to get the sum of a range of increasing natural numbers is:

$$\frac {n \times (n + 1)}{2}$$

So to find the missing number all we have to do is to find out the sum of the range with above formula, and then subtracting the sum of the given integers (second input).

## Code

In Rust 🦀 code:

```rust
fn main() {
    let mut l = std::io::stdin().lines().take(2);
    let (f, s) = (l.next().unwrap(), l.next().unwrap());

    let n: u64 = f.unwrap().parse().unwrap();

    let sum: u64 = s
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .sum();

    println!("{}", n * (n + 1) / 2 - sum);
}
```
