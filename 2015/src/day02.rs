fn main() {
    let input = include_str!("input/day02.txt");
    println!(
        "{:?}",
        input.lines().fold((0, 0), |(a, b), e| {
            let mut dims: [_; 3] = e
                .split('x')
                .take(3)
                .map(|s| s.parse::<u128>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            dims.sort_unstable();
            let [x, y, z] = dims;
            (
                a + 2 * (x * y + y * z + z * x) + x * y,
                b + 2 * (x + y) + x * y * z,
            )
        })
    );
}
