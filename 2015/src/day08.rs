fn main() {
    fn decode(s: &str) -> usize {
        let mut cnt = 0;
        let mut char = s.chars();
        while let Some(c) = char.next() {
            if c == '"' {
                continue;
            }
            if c == '\\' {
                let next = char.next().unwrap();
                if next == 'x' {
                    char.next();
                    char.next();
                }
            }
            cnt += 1;
        }
        cnt
    }
    fn encode(s: &str) -> usize {
        let mut cnt = 0;
        for c in s.chars() {
            if c == '"' || c == '\\' {
                cnt += 1;
            }
            cnt += 1;
        }
        cnt + 2
    }
    let mut dcount = 0;
    let mut ecount = 0;
    for s in include_str!("input/day08.txt").trim().lines() {
        dcount += s.len() - decode(s);
        ecount += encode(s) - s.len();
    }
    println!("{dcount}");
    println!("{ecount}");
}
