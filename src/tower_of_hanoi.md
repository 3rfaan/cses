# TOWER OF HANOI

| var | desc            |
| --- | --------------- |
| $n$ | Number of disks |

There are two rules in the game Tower of Hanoi:

1. On each move we can only move the uppermost disk from a stack to another stack.
2. It is not allowed to place a larger disk on a smaller disk.

## Minimum required steps

For $n$ disks, the minimum number of required steps is $2^n - 1$:

| $n$ | $f(n) = 2^n - 1$ | Steps |
| --- | ---------------- | ----- |
| $1$ | $2^1 - 1$        | $1$   |
| $2$ | $2^2 - 1$        | $3$   |
| $3$ | $2^3 - 1$        | $7$   |

## Recursion

Tower of Hanoi in context of programming is a classical example of recursive programming.

We have three stacks, let's call them `src` for source, `tmp` for temporary and `tgt` for target. The goal is to move all disks from `src` to `tgt` in increasing order.

### $n = 1$

In the simplest case $n = 1$. That means we can directly move the single disk from `src` to `tgt`. In this case the output of our program is:

```
1
1 3
```

The minimum steps required to win the game is to put the disk from stack 1 to stack 3.

### $n = 2$

Let's consider $n = 2$. We know the minimum number of steps is $2^n - 1 = 2^2 -1 = 3$. In this case, at the start of the game we have two disks in `src`. We can only move one disk at a time.

1. Take the smaller top disk in `src` (1) and move it to the temporary stack `tmp` (2).
2. Take the larger disk remaining in `src` (1) and move it to `tgt` (3).
3. Take the smaller disk in `tmp` (2) and move it to `tgt` (3).

The output of the program should look like this:

```
3
1 2
1 3
2 3
```

### $n = 3$

Let's consider $n = 3$. The minimum number of steps is $2^n - 1 = 2^3 - 1 = 7$. In this case, at the start of the game we have three disks ($d_1, d_2, d_3$) in `src` where $d_1$ is the smallest one.

1. Take $d_1$ from `src` (1) and move it to `tgt` (3).
2. Take $d_2$ from `src` (1) and move it to `tmp` (2).
3. Take $d_1$ from `tgt` (3) and move it to `tmp` (2).
4. Take $d_3$ from `src` (1) and move it to `tgt` (3).
5. Take $d_1$ from `tmp` (2) and move it to `src` (1).
6. Take $d_2$ from `tmp` (2) and move it to `tgt` (3).
7. Take $d_1$ from `src` (1) and move it to `tgt` (3).

As you can see we needed seven steps to complete the game. Here is the output:

```
7
1 3
1 2
3 2
1 3
2 1
2 3
1 3
```

We can observe that we basically do the same for any number of disks $n$. We move $n - 1$ disks to `tmp` then moving the remaining largest disk in `src` to `tgt`. Then we use `src` as temporary stack to move all $n - 1$ disks from `temp` to `tgt`. If there is only one disk remaining we move it directly to `tgt`.

## Code

In Rust 🦀 code:

```rust
fn main() {
    let n: u8 = std::io::read_to_string(std::io::stdin())
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    println!("{}", (0b1 << n) - 1);
    hanoi(n, 1, 2, 3);
}

fn hanoi(n: u8, src: u8, tmp: u8, tgt: u8) {
    if n == 0 { return; }

    hanoi(n - 1, src, tgt, tmp);
    println!("{} {}", src, tgt);
    hanoi(n - 1, tmp, src, tgt);
}
```
