# PERMUTATIONS

| var | desc                             |
| --- | -------------------------------- |
| $n$ | Range of integers $1, 2, ..., n$ |

In a range from $1, 2, ..., n$ we have to construct a _beautiful permutation_ meaning no adjacent elements differ by 1.

First we have to consider that when $n = 2$ or $n = 3$ then it's not possible to create a beautiful permutation as the elements will always differ by 1 no matter the combination.

Otherwise all we really have to do is creating two iterators which increase by 2 in each iteration. One iterator should start at 2 and the other one at 1.

Let's say we have an iterator over the range `(2..=n)` where $n = 5$ and we increment by 2 in each iteration:

| 2   | 4   |
| --- | --- |

This will be the output of this first iterator.

Now let's look at the second iterator over the range `(1..=n)` where $n = 5$ and we increment by 2 in each iteration:

| 1   | 3   | 5   |
| --- | --- | --- |

When we now combine the outputs we get:

| 2   | 4   | 1   | 3   | 5   |
| --- | --- | --- | --- | --- |

We have a beautiful permutation!

In Rust we can use the `chain()` method to chain together two iterators. When the first iterator iterated over each element, then the second iterator will start iterating.

## Code

In Rust 🦀 code:

```rust
fn main() {
    let inp = std::io::read_to_string(std::io::stdin())
        .unwrap()
        .trim()
        .parse::<u64>()
        .unwrap();

    if inp == 2 || inp == 3 { println!("NO SOLUTION"); }
    else {
        (2..=inp)
            .step_by(2)
            .chain((1..=inp).step_by(2))
            .for_each(|e| print!("{e} "));
        println!("");
    }
}
```
