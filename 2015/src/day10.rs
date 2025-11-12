fn main() {
    let mut input = include_str!("input/day10.txt").trim().to_owned();
    for _ in 0..40 {
        input = look_say(&input);
    }
    println!("{}", input.len());
    for _ in 0..10 {
        input = look_say(&input);
    }
    println!("{}", input.len());
}

fn look_say(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    while let Some(ch) = chars.next() {
        let mut count = 1;
        while let Some(&mext) = chars.peek() {
            if mext == ch {
                chars.next();
                count += 1;
            } else {
                break;
            }
        }
        result.push_str(&count.to_string());
        result.push(ch);
    }
    result
}
