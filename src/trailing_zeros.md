# TRAILING ZEROS

| var | desc                         |
| --- | ---------------------------- |
| $n$ | Number to factorialize: $n!$ |

A factorial $n!$ is computed like this:

$$n! = n \times (n - 1) \times ... \times 2 \times 1$$

For example $4!$:

$$4! = 4 \times 3 \times 2 \times 1 = 24$$

## Dividing by 10

One approach would be to check if the result of factorial $n!$ is divisible by 10. If it is we increment a counter and divide $n!$ by 10, repeating until it's not divisible by 10 anymore. The counter will then contain the amount of trailing zeros.

```rust
let mut fac: u128 = (1..=n).product();
let mut cnt = 0;

while fac % 10 == 0 { cnt += 1; fac /= 10; }

println!("Trailing zeros: {cnt}");
```

But this approach is pretty slow and could lead to integer overflow when dealing with big numbers.

## Prime Factorization

10 can be factorized by prime numbers 2 and 5:

$$10 = 2 \times 5$$

So we have to check for 2 and 5 pairs in the factorial equation. Because there will be more 2's than 5's we can look for 5's only.

$$5! = 1 \times 2 \times 3 \times 4 \times 5 = 120$$
$$5! = (2 \times 5) \times (1 \times 3 \times 4) = 120$$

By the way, $4$ above is not a prime number. It can further be expanded $4 = 2 \times 2$. But we don't care as we already have a pair of 2's and 5's so we don't count the rest of 2's.

Here we have one time $2$ and one time $5$, so we know the factorial $5! = 120$ is divisible by 10 and contains one trailing zero because we found one occurrence of 5.

So we check how many times $5$ fits into $n$ by dividing $n$ by $5$. We assign the result to variable `res` and then divide $n$ by $5$ and reassigning it to $n$.

## Code

In Rust 🦀 code:

```rust
fn main() {
    let mut n: u64 = std::io::read_to_string(std::io::stdin())
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    let mut res = 0;

    while n > 0 { res += n / 5; n /= 5; }

    println!("{res}");
}
```
