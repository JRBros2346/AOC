use std::collections::HashMap;

fn main() {
    let input = include_str!("input");
    println!(
        "{}",
        input.lines().fold(0, |a, e| {
            if e.contains("ab") || e.contains("cd") || e.contains("pq") || e.contains("xy") {
                return a;
            }
            let mut vowels = 0;
            let mut has_double = false;
            let mut prev_char = '\0';
            for c in e.chars() {
                if "aeiou".contains(c) {
                    vowels += 1;
                }
                if c == prev_char {
                    has_double = true;
                }
                prev_char = c;
                if vowels >= 3 && has_double {
                    return a + 1;
                }
            }
            a
        })
    );
    println!(
        "{}",
        input.lines().fold(0, |a, e| {
            if e.as_bytes().windows(3).any(|w| w[0] == w[2]) {
                let mut seen = HashMap::<(u8, u8), usize>::new();
                for (i, w) in e.as_bytes().windows(2).enumerate() {
                    let pair = (w[0], w[1]);
                    if let Some(&j) = seen.get(&pair) {
                        // non‑overlap means at least one letter between occurrences
                        if i > j + 1 {
                            return a + 1;
                        }
                    } else {
                        seen.insert(pair, i);
                    }
                }
            }
            a
        })
    );
}
