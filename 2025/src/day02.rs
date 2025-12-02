fn main() {
    let input = include_str!("input/day02.txt").trim();
    let mut total0 = 0;
    let mut total1 = 0;
    for s in input.split(',').map(|s| -> (usize, usize) {
        let s = s.split_once('-').unwrap();
        (s.0.parse().unwrap(), s.1.parse().unwrap())
    }) {
        for n in s.0..=s.1 {
            let len = (n as f64).log10().floor() as u32 + 1;
            if n % 10usize.pow(len / 2) == n / 10usize.pow(len / 2) {
                total0 += n;
            }
            let n = n.to_string();
            if n.repeat(2)[1..n.len() * 2 - 1].contains(&n) {
                total1 += n.parse::<usize>().unwrap();
            }
        }
    }
    println!("{total0}");
    println!("{total1}");
}
