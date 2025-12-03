fn main() {
    let mut joltage0 = 0;
    let mut joltage1 = 0;
    for line in include_str!("input/day03.txt").trim().lines() {
        joltage0 += max_jolt(&line.chars().collect::<Vec<_>>(), 2);
        joltage1 += max_jolt(&line.chars().collect::<Vec<_>>(), 12);
    }
    println!("{joltage0}");
    println!("{joltage1}");
}

fn max_jolt(line: &[char], flips: usize) -> usize {
    if flips == 1 {
        return line.iter().max().unwrap().to_digit(10).unwrap() as usize;
    }
    let first = line[..line.len() - flips + 1]
        .iter()
        .max()
        .unwrap()
        .to_digit(10)
        .unwrap() as usize;
    let pos = line
        .iter()
        .position(|&c| c.to_digit(10).unwrap() == first as u32)
        .unwrap();
    first * 10usize.pow(flips as u32 - 1) + max_jolt(&line[pos + 1..], flips - 1)
}
