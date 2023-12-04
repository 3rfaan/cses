# INCREASING ARRAY

| var                  | desc                    |
| -------------------- | ----------------------- |
| $n$                  | Size of array           |
| $x_1, x_2, ..., x_n$ | Integers array contains |

In this exercise we get an array of $n$ integers. The goal is to have an array in which every element is at least as large as the previous element. We have to output the minimum moves required to achieve this.

If we consider this array:

| 3   | 2   | 5   | 1   | 7   |
| --- | --- | --- | --- | --- |

We can observe here that $3 > 2$. So we have to increment $2$ by $1$.

| 3   | 3   | 5   | 1   | 7   |
| --- | --- | --- | --- | --- |

Then $3 < 5$. We don't have to modify anything here. But in the next pair $5 > 1$. So we have to increment $1$ by $4$.

| 3   | 3   | 5   | 5   | 7   |
| --- | --- | --- | --- | --- |

We now have an array which is increasing. So every element is at least as large as the previous one.

We had to increment $2$ by $1$ and $1$ by $4$. So in total $5$ steps were necessary.

## Indexing

This time we will index into the array rather than peeking because we are mutating its content.

We check in each iteration if `array[i]` is smaller than `array[i - 1]` starting at index 1. Or in other words: Is the current value at index $i$ smaller than the previous value at index $i - 1$?

If it is we get the difference between the two values and store it in a variable. This will represent the amount of moves we had to take. Then we assign the previous value `array[i - 1]` to the current element `array[i]`. That means the current element is now as large as the previous element.

## Code

In Rust 🦀 code:

```rust
fn main() {
    let mut inp: Vec<u64> = std::io::read_to_string(std::io::stdin())
        .unwrap()
        .lines()
        .skip(1)
        .flat_map(|s| s.split_whitespace())
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    let mut res = 0;

    for i in 1..inp.len() {
        if inp[i] < inp[i - 1] {
            res += inp[i - 1] - inp[i];
            inp[i] = inp[i - 1];
        }
    }

    println!("{res}");
}
```
