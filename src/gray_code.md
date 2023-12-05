# GRAY CODE

| var | desc                                      |
| --- | ----------------------------------------- |
| $n$ | Output should be $2^n$ lines of Gray code |

Gray code is represented like binary in 1's and 0's. The difference to binary is that any two successive bit strings differ in exactly one bit. In other words the distance between two adjacent bit strings in a sequence is never more than 1. Let's see a comparison:

| Decimal | Binary | Gray code |
| ------- | ------ | --------- |
| 0       | `0000` | `0000`    |
| 1       | `0001` | `0001`    |
| 2       | `0010` | `0011`    |
| 3       | `0011` | `0010`    |
| 4       | `0100` | `0110`    |
| 5       | `0101` | `0111`    |

We observe that while in binary the distance in the sequence of bit strings might differ, in Gray code the distance is always 1 bit.

That means, if we have `011` the next bit string cannot be `100` because two bits would have to be modified, or in other words the distance between the two bit strings would be two. But `111` would be possible as we only modified the most significant bit (MSB), so the distance is one.

## Binary to Gray Code Conversion

We convert from binary to Gray code by taking the most significant bit of the binary bit string as is, it stays always the same in the corresponding Gray code. The other bits we obtain by performing XOR operations at the index and next index of the binary bit string. Let's see an example of a 3-bit binary number and how we convert it to Gray code:

| $b_2$ | $b_1$ | $b_0$ |
| ----- | ----- | ----- |
| 1     | 0     | 0     |

1. We take the most significant bit $b_2$ and assign it to $g_2$.

| $g_2$ | $g_1$ | $g_0$ |
| ----- | ----- | ----- |
| 1     | -     | -     |

So we can conclude:

$$g_2 = b_2$$

2. We perform an XOR operation on the first bit of the binary string ($b_2$) and the next one ($b_1$):

$$g_1 = b_2 \oplus b_1 = 1 \oplus 0 = 1$$

| $g_2$ | $g_1$ | $g_0$ |
| ----- | ----- | ----- |
| 1     | 1     | -     |

3. We perform another XOR operation this time on $b_1$ and $b_0$:

$$g0 = b_1 \oplus b_0 = 0 \oplus 0 = 0$$

| $g_2$ | $g_1$ | $g_0$ |
| ----- | ----- | ----- |
| 1     | 1     | 0     |

That's all! We have successfully converted a binary bit string to a Gray code bit string. So `100` in binary is `110` in Gray code.

## Gray Code Conversion for each Power of $n$

As stated in the assessment we have to print $2^n$ lines of Gray code, where $n$ is the input given. So for example $n = 2$, we have to print out $2^2 = 4$ lines of Gray code. We can achieve this by iterating through a range from $0, 1, ..., 2^n$, where we convert each integer of the range in each iteration to Gray code directly.

## Code

In Rust 🦀 code:

```rust
fn main() {
    let inp: usize = std::io::read_to_string(std::io::stdin())
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    (0..(0b1 << inp)).for_each(|b| {
        println!("{:<01$b}", (b >> 0b1) ^ b, inp);
    });
}
```

### Explanation

1. We take an input $n$ and assign it to variable `inp`.

2. We iterate over a range from $0, 1, ..., 2^n$. In code we can use the bitwise left shit operation `<<` to raise 2 to the power of `inp`. In this case `0b1 << inp` is the same as $2^n$. So if $n = 2$, we left shift `001` by two bits, resulting in `100` which is 4 in binary.

3. In each operation we take the current integer in the range and right shift i by 1 bit. This is because we want to take the MSB of the binary number without modifying it as stated above. Then we XOR ($\oplus$) every bit in the binary number by the next bit. This will give us Gray code.

4. We must output the resulting number in binary representation. We can use `print!("{:b}", 4)`, which would print out `100`.

5. We need the proper alignment. So if we have $n = 3$ we want every bit string to have left padding of 0 $n$ times. So instead of `1` we want to have `001`. We can achieve this by using a format string `{:<01$b}`. This means we left pad (`<`) with the symbol `0` with a with of `1$` which represents the second argument to `println!()` which in this case is `inp`.
