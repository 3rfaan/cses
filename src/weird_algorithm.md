# WEIRD ALGORITHM

| var | desc             |
| --- | ---------------- |
| $n$ | Positive integer |

This algorithm takes a positive integer and applies one of two operations based on the following conditionals:

1. **If $n$ is even:** Divide $n$ by two: $n \div 2$
2. **If $n$ is odd:** Multiply $n$ by $3$ and add $1$: $n \times 3 + 1$

This assessment can be solved by using a simple if-else condition.

We assure if $n$ is even by checking if the remainder of $n \div 2$ is 0 using modulo operation. So the formula would be:

$$n \mod 2 = 0$$

If this equation is true then we know $n$ is even and we divide $n$ by $2$ and then reassigning the result to $n$. If the equation is false we know that $n$ is odd so we multiply $n$ by $3$ and add $1$, then reassigning the result back to $n$.

We repeat this process until $n = 1$.

## Code

In Rust 🦀 code:

```rust
fn main() {
    let mut inp: u64 = std::io::read_to_string(std::io::stdin())
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    print!("{inp}");
    while inp > 1 {
        if inp % 2 == 0 { inp /= 2; }
        else { inp = inp * 3 + 1; }
        print!(" {inp}");
    }
    println!();
}
```
