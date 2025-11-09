fn main() {
    use std::collections::HashSet;
    let input = include_str!("input");
    let mut houses = HashSet::<(i128, i128)>::new();
    houses.insert((0, 0));
    input.chars().fold((0, 0), |(x, y), c| {
        let x = match c {
            '>' => (x + 1, y),
            '<' => (x - 1, y),
            '^' => (x, y + 1),
            'v' => (x, y - 1),
            _ => (x, y),
        };
        houses.insert(x);
        x
    });
    println!("{}", houses.len());
    let mut houses = HashSet::<(i128, i128)>::new();
    houses.insert((0, 0));
    input.as_bytes().chunks(2).fold([(0, 0), (0, 0)], |a, e| {
        let x = match e[0] as char {
            '>' => (a[0].0 + 1, a[0].1),
            '<' => (a[0].0 - 1, a[0].1),
            '^' => (a[0].0, a[0].1 + 1),
            'v' => (a[0].0, a[0].1 - 1),
            _ => a[0],
        };
        houses.insert(x);
        let y = match e.get(1).copied().unwrap_or(b' ') as char {
            '>' => (a[1].0 + 1, a[1].1),
            '<' => (a[1].0 - 1, a[1].1),
            '^' => (a[1].0, a[1].1 + 1),
            'v' => (a[1].0, a[1].1 - 1),
            _ => a[1],
        };
        houses.insert(y);
        [x, y]
    });
    println!("{}", houses.len());
}
