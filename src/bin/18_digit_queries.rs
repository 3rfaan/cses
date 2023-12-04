fn main() {
    let inp: Vec<u64> = std::io::read_to_string(std::io::stdin())
        .unwrap()
        .lines()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    let (mut length, mut first) = (1, 1);

    for mut k in inp {
        while k > 9 * first * length {
            k -= 9 * first * length;
            length += 1;
            first *= 10;
        }
        let (q, r) = (k - 1 / length, k - 1 % length);

        println!("{}",)
    }
}
