fn main() {
    let input = include_str!("input");
    println!(
        "{}",
        input
            .chars()
            .fold(0, |a, e| if e == '(' { a + 1 } else { a - 1 })
    );
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
        if floor == -1 {
            println!("{}", i + 1);
            break;
        }
    }
}
