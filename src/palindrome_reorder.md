# PALINDROME REORDER

| var                    | desc                |
| ---------------------- | ------------------- |
| `String` of length $n$ | String to rearrange |

A palindrome is a word which reads the same forwards and backwards, e. g. _"RACECAR"_.

As an input we get a `String` which we have to rearrange to a palindrome. If it is not possible the output should be `NO SOLUTION`.

## Even occurrences of characters

If we observe the palindrome _"AABBCC"_ we can rearrange it to read _"ABCCBA"_. This is because the occurrence of each character is even. So in this case:

| char | occurrences |
| ---- | ----------- |
| A    | 2           |
| B    | 2           |
| C    | 2           |

## Uneven occurrences of characters

If we take for example _"AABBC"_. This string can be rearranged to a palindrome like this: _"ABCBA"_.

| char | occurrences |
| ---- | ----------- |
| A    | 2           |
| B    | 2           |
| C    | 1           |

The same goes for _"AABBCCC"_. The palindrome would be _"ABCCCBA"_.

So if we have a character which has uneven occurrences in the string we can still create a palindrome by putting the uneven character(s) in the middle of the string.

If we had more characters with uneven occurrences in the string like _"AABBBC"_ it would be impossible to create a palindrome.

| char | occurrences |
| ---- | ----------- |
| A    | 2           |
| B    | 3           |
| C    | 1           |

This is because we can only put one type of character with uneven occurrences in the middle.

## Mapping occurrences of characters

The first step is to create a `HashMap` which has key, value entries. The key should be the character and the value should be its occurrence. If we again take the first example _"AABBC"_, then the map will hold these entries:

```
{
    "A": 2,
    "B": 2,
    "C": 1
}
```

## Check if more than one character has uneven occurrences

We have concluded that there can at most be one type of character with uneven occurrences in the string. So we have to check that there are not more than one single entry in the `HashMap` with an uneven number as value.

## Creating the palindrome

First we create two variables, `even` and `odd`, both of type `String`. Then we traverse over the `HashMap` we have created. First, we need to check if the entry holds a value which is uneven. If it does, we know that the character(s) must be placed in the middle of the palindrome. So we have to append to the `odd` string the character (key of `HashMap`) $n$ times where $n$ is the number of occurrences in the input string (value of `HashMap`).

If the entry holds an even value we append to `even`. But this time we only append half the number of occurrences. Meaning the value of `HashMap` divided by 2. This is because a palindrome is symmetrical. So we have to only do one half and for the second half we just mirror what we have in the first half.

For example, let's say we have a palindrome _"AABBBCCCC"_. We get this `HashMap`:

```
{
    "A": 2,
    "B": 3,
    "C": 4
}
```

**1. Iteration:**

Entry: `{ "A": 2 }`

We append to `even` `"A"`. Because $2 \div 2 = 1$. So `even` holds `"A"`.

**2. Iteration:**

Entry: `{ "B": 1 }`

We append to `odd` `"B"`. Because the value is odd we don't need to divide by 2. So `odd` hols `"B"`.

**3. Iteration:**

Entry: `{ "C": 4 }`

We append to `even` `"CC"`. Because $4 \div 2 = 2$. So `even holds` `"ACC"`.

Then we have to join the strings.

So `even` holds `"ACC"` and `odd` holds `"B"`. When we join these two string together we get `"ACCB"`. We have the first part! Now all we need to do is to take `even`, reverse it, and then append it to the string. So `even` reversed would be `"CCA"`. When we join it with the string we get `"ACCBCCA"`. A palindrome!

## Code

In Rust 🦀 code:

```rust
use std::collections::HashMap;

fn main() {
    let inp = std::io::read_to_string(std::io::stdin()).unwrap();

    let map = inp.trim().chars().fold(HashMap::new(), |mut map, c| {
        map.entry(String::from(c))
            .and_modify(|n| *n += 1)
            .or_insert(1);
        map
    });

    if map.values().filter(|&e| *e % 2 != 0).count() > 1 {
        println!("NO SOLUTION");
        return;
    }

    let (mut even, mut odd) = (String::new(), String::new());

    for (ch, cnt) in map {
        if cnt % 2 != 0 { odd.push_str(&ch.repeat(cnt)); }
        else { even.push_str(&ch.repeat(cnt / 2)); }
    }
    println!("{}{}{}", even, odd, even.chars().rev().collect::<String>());
}
```
