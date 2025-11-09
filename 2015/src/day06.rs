#[derive(Debug)]
enum Mode {
    Toggle,
    TurnOn,
    TurnOff,
}

fn main() {
    let mut lights = vec![[false; 1000]; 1000];
    let mut new_lights = vec![[0u8; 1000]; 1000];
    let mut count = 0;
    let mut new_count = 0;
    for inst in include_str!("input/day06.txt").trim().lines() {
        let mode;
        let range;
        match inst.split_once(' ').unwrap() {
            ("toggle", extra) => {
                mode = Mode::Toggle;
                range = extra
            }
            ("turn", extra) => match extra.split_once(' ').unwrap() {
                ("on", extra) => {
                    mode = Mode::TurnOn;
                    range = extra;
                }
                ("off", extra) => {
                    mode = Mode::TurnOff;
                    range = extra;
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
        let (start, end) = range.split_once(" through ").unwrap();
        let start = start.split_once(',').unwrap();
        let start = (
            start.0.parse::<usize>().unwrap(),
            start.1.parse::<usize>().unwrap(),
        );
        let end = end.split_once(',').unwrap();
        let end = (
            end.0.parse::<usize>().unwrap(),
            end.1.parse::<usize>().unwrap(),
        );
        for row in lights.iter_mut().take(end.0 + 1).skip(start.0) {
            for ele in row.iter_mut().take(end.1 + 1).skip(start.1) {
                match mode {
                    Mode::Toggle => {
                        *ele = !*ele;
                        if *ele {
                            count += 1
                        } else {
                            count -= 1
                        };
                    }
                    Mode::TurnOn => {
                        if !*ele {
                            *ele = true;
                            count += 1;
                        }
                    }
                    Mode::TurnOff => {
                        if *ele {
                            *ele = false;
                            count -= 1;
                        }
                    }
                }
            }
        }
        for row in new_lights.iter_mut().take(end.0 + 1).skip(start.0) {
            for ele in row.iter_mut().take(end.1 + 1).skip(start.1) {
                match mode {
                    Mode::Toggle => {
                        *ele += 2;
                        new_count += 2
                    }
                    Mode::TurnOn => {
                        *ele += 1;
                        new_count += 1;
                    }
                    Mode::TurnOff => {
                        if *ele > 0 {
                            *ele -= 1;
                            new_count -= 1;
                        }
                    }
                }
            }
        }
    }
    println!("{count}");
    println!("{new_count}");
}
