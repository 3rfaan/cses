# BIT STRINGS

| var | desc                 |
| --- | -------------------- |
| $n$ | Length of bit string |

## n == 2

```
00
01
10
11
```

Possible combinations = $4$

## n == 3

```
000
001
010
011

100
101
110
111
```

Possible combinations = $8$

## n == 4

```
0000
0001
0010
0011

0100
0101
0110
0111

1000
1001
1010
1011

1100
1101
1110
1111
```

Possible combinations = $16$

The formula would be: $2^n$

However this algorithm would cause an overflow with `u64` and even with `u128`.

## Iteration

So instead we iterate over range $1, 2, ..., n$ and for each iteration we multiply the result of past iteration by 2.

The formula would look like this:

$$f(n) = 2 \times f(n - 1)$$

We multiply by 2 because for each $i$ in $1, 2, ..., n$ there are two possible binary values: `0` and `1`.

Also on each iteration we apply $\mod 10^9 + 7$ to account for possible overflow.

For example if $n = 3$ and `res = 1`:

$$2 \times 1 \mod (10^9 + 7) = 2$$

$$2 \times 2 \mod (10^9 + 7) = 4$$

$$2 \times 4 \mod (10^9 + 7) = 8$$

So after iterating three times `res = 8`. This would be the same as $2^3 = 8$.

This is possible because of the following rule in modulo arithmetic:

$$x^2 \mod c = (x \mod c \times x \mod c) \mod c$$

## Code

In Rust 🦀 code:

```rust
fn main() {
    const MOD: u64 = 1_000_000_007; // 10 ^ 9 + 7

    let n: u64 = std::io::read_to_string(std::io::stdin())
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    let res = (0..n).fold(1, |res, _| 2 * res % MOD);

    println!("{res}");
}
```
