# DIGIT QUERIES

| var | desc                           |
| --- | ------------------------------ |
| $q$ | Number of queries              |
| $k$ | _1-indexed_ position in string |

In an infinite string consisting of positive integers in increasing order we have to find out what digit lies at position $k$.

## Digit Blocks

Building the whole string and iterating over it would take far too long when we consider that $k \le 10^{18}$.

But we can observe that we could split our range in various blocks according to the number of digits:

| Number of digits | range                   | numbers | digits  |
| ---------------- | ----------------------- | ------- | ------- |
| 1 digit          | $1, 2, ..., 9$          | $9$     | $9$     |
| 2 digits         | $10, 11, ..., 99$       | $90$    | $180$   |
| 3 digits         | $100, 101, ..., 999$    | $900$   | $2700$  |
| 4 digits         | $1000, 1001, ..., 9999$ | $9000$  | $36000$ |

We can observe that the numbers increase by $9 * 10^n$ in each block in iteration $0, 1, ..., n$.

Also the amount of digits in each block is calculated by $9 * 10^n * d$ where $d$ is the number of digits (in block 1 $d = 1$, in block 2 $d = 2$, etc.).

So in a `while` loop we conduct these steps:

1. Check if $k$ is bigger than the amount of digits in the current block $k > 9 * 10^n * d$
2. Decrement $k$ by the amount of digits in the current block `k = k - 9 * 10^n * d`
3. We multiply `block` by $10$ and increment `dig` by $1$. `block` represents the current block we are in ($1, 10, 100, 1000, ...$) and `dig` represents the number of digits in the current block ($1, 2, 3, ...$).

## Building the String

After exiting the loop we decremented $k$ by the amount of digits of each block smaller than $k$ in order to consider only the numbers in the range of the block we are looking for.

Then we begin to build the string of the number at index $k$. Because we started at index $1$ we subtract $k - 1$. Then we divide $k - 1$ by `dig`.

$$q = \frac{k - 1}{dig}$$

This will give us the offset in the appropriate block.

Then we need to calculate the index in the number at index $k$. We achieve this by again subtracting $1$ from $k$ and then taking the remainder:

$$r = (k - 1) \mod dig$$

Then we build the string by adding $q$ (offset) to `block`. The last step will be to index into the string at position of $r$ (remainder).

## Code

In Rust 🦀 code:

```rust
fn main() {
    let inp: Vec<usize> = std::io::read_to_string(std::io::stdin())
        .unwrap()
        .lines()
        .skip(1)
        .map(|n| n.parse().unwrap())
        .collect();

    for mut k in inp {
        let (mut block, mut dig) = (1, 1);

        while k > 9 * block * dig {
            k -= 9 * block * dig;

            block *= 10; dig += 1;
        }

        let (quot, rem) = ((k - 1) / dig, (k - 1) % dig);

        if let Some(d) = (block + quot).to_string().chars().nth(rem) {
            println!("{d}");
        }
    }
}
```
