use std::str::Chars;

struct Parser<'a> {
    chars: Chars<'a>,
    current: Option<char>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let current = chars.next();
        Self { chars, current }
    }

    fn bump(&mut self) {
        self.current = self.chars.next();
    }

    // Returns: ((part1_sum, part2_sum), is_red_string)
    fn parse(&mut self, red_filter: bool) -> (isize, bool) {
        match self.current {
            Some('{') => (self.object(red_filter), false),
            Some('[') => (self.array(red_filter), false),
            Some('"') => (0, self.string_red()),
            Some(c) if c.is_ascii_digit() || c == '-' => (self.number(), false),
            other => panic!("Unexpected '{other:?}'"),
        }
    }

    fn string_red(&mut self) -> bool {
        self.bump();
        let mut s = String::new();
        while let Some(c) = self.current {
            match c {
                '"' => {
                    self.bump();
                    break;
                }
                _ => {
                    s.push(c);
                    self.bump();
                }
            }
        }
        s == "red"
    }

    fn number(&mut self) -> isize {
        let mut num = String::new();
        while let Some(c) = self.current {
            if c.is_ascii_digit() || c == '-' {
                num.push(c);
                self.bump();
            } else {
                break;
            }
        }
        num.parse().unwrap()
    }

    fn array(&mut self, red_filter: bool) -> isize {
        self.bump();
        let mut sum = 0;

        loop {
            if self.current == Some(']') {
                self.bump();
                break;
            }
            sum += self.parse(red_filter).0;

            match self.current {
                Some(',') => self.bump(),
                Some(']') => {
                    self.bump();
                    break;
                }
                other => panic!("Unexpected '{other:?}' in array"),
            }
        }
        sum
    }

    fn object(&mut self, red_filter: bool) -> isize {
        self.bump();
        let mut sum = 0;
        let mut has_red = false;

        loop {
            if self.current == Some('}') {
                self.bump();
                break;
            }
            if self.current != Some('"') {
                panic!("Expected string key in object");
            }
            self.string_red(); // discard key

            if self.current != Some(':') {
                panic!("Expected ':' after key");
            }
            self.bump();

            let (a, is_red) = self.parse(red_filter);
            has_red |= is_red;
            sum += a;

            match self.current {
                Some(',') => self.bump(),
                Some('}') => {
                    self.bump();
                    break;
                }
                other => panic!("Unexpected '{other:?}' in object"),
            }
        }
        if has_red & red_filter { 0 } else { sum }
    }
}

fn main() {
    let input = include_str!("input/day12.txt").trim();
    println!("{:?}", Parser::new(input).parse(false));
    println!("{:?}", Parser::new(input).parse(true));
}
