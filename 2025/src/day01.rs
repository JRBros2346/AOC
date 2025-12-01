fn main() {
    let mut dial: isize = 50;
    let mut pass = 0;
    let mut pass434c49434b = 0;
    for line in include_str!("input/day01.txt").trim().lines() {
        let delta = line[1..].parse::<isize>().unwrap();
        let first;
        match &line[0..1] {
            "L" => {
                first = (dial + 99).rem_euclid(100) + 1;
                dial -= delta
            }
            "R" => {
                first = 100 - dial;
                dial += delta
            }
            _ => unreachable!(),
        };
        if first < delta {
            pass434c49434b += 1 + ((delta - 1 - first) / 100);
        }
        dial = dial.rem_euclid(100);
        if dial == 0 {
            pass += 1;
            pass434c49434b += 1;
        }
    }
    println!("{pass}");
    println!("{pass434c49434b}");
}
