# REPETITIONS

We get a string representing a DNA sequence. Our task is to find the longest repetition of a character in the sequence, or in other words the longest substring containing one type of character. The possible characters are A, C, G and T.

We can solve this task by iterating over each character and comparing it with the following character. If it is the same character we increment a counter variable, say `cnt`. If it is not the same character we store `cnt` in `max` if `cnt` is higher than `max` which stores the highest count. We repeat this process for each character in the string.

We could solve this approach by indexing. Say we have a string `s`, we can compare the character in first index `s[0]` with the second character `s[0 + 1]`. Using an iterator `0` would be replaced by generic `i`.

But in Rust we can create a [peekable iterator](https://doc.rust-lang.org/std/iter/struct.Peekable.html) which allows us to use the `peek()` method to see what value is in the next index. Consider this example from the Rust documentation:

```rust
let xs = [1, 2, 3];

let mut iter = xs.iter().peekable();

// peek() lets us see into the future
assert_eq!(iter.peek(), Some(&&1));
assert_eq!(iter.next(), Some(&1));

assert_eq!(iter.peek(), Some(&&2));
assert_eq!(iter.next(), Some(&2));`
```

As you can see when we `peek()` we get back the next element in the collection before the iterator calls the `next()` method.

## Code

In Rust 🦀 code:

```rust
fn main() {
    let inp = std::io::read_to_string(std::io::stdin()).unwrap();

    let mut dna = inp.chars().peekable();
    let (mut cnt, mut max) = (1, 1);

    while let (Some(c), Some(&n)) = (dna.next(), dna.peek()) {
        if c == n { cnt += 1; }
        else {
            max = std::cmp::max(max, cnt);
            cnt = 1;
        }
    }
    println!("{max}");
}
```
