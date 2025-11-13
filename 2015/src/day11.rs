use std::{fmt::Display, ops::Not};

#[repr(transparent)]
struct Password([u8; 8]);
impl Password {
    fn straight(&self) -> bool {
        for w in self.0.windows(3) {
            if w[0] + 1 == w[1] && w[1] + 1 == w[2] {
                return true;
            }
        }
        false
    }
    fn pairs(&self) -> bool {
        let mut count = 0;
        let mut i = 0;
        while i < 7 {
            if self.0[i] == self.0[i + 1] {
                count += 1;
                i += 2;
            } else {
                i += 1;
            }
            if count >= 2 {
                return true;
            }
        }
        false
    }
    fn valid(&self) -> bool {
        self.straight() && self.pairs()
    }
}
impl Not for Password {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        loop {
            let mut i = 7;
            loop {
                self.0[i] += 1;
                if self.0[i] == b'i' || self.0[i] == b'o' || self.0[i] == b'l' {
                    self.0[i] += 1;
                }
                if self.0[i] > b'z' {
                    self.0[i] = b'a';
                    if i == 0 {
                        break;
                    }
                    i -= 1;
                } else {
                    break;
                }
            }

            if self.valid() {
                return self;
            }
        }
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &c in &self.0 {
            write!(f, "{}", c as char)?;
        }
        Ok(())
    }
}

fn main() {
    let input = include_str!("input/day11.txt").trim();
    let password = Password(input.as_bytes().try_into().unwrap());
    let password = !password;
    println!("{}", password);
    println!("{}", !password);
}
