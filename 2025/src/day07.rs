use std::collections::HashSet;
fn main() {
    let mut input = include_str!("input/day07.txt").trim().lines();
    let mut beam: HashSet<_> = input
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .filter_map(|(i, c)| if c == 'S' { Some(i) } else { None })
        .collect();
    let mut timeline = vec![0; 1000];
    for &pos in &beam {
        timeline[pos] = 1;
    }
    let mut splits = 0;
    for line in input.map(|c| c.chars().collect::<Vec<_>>()) {
        let mut new_beam = HashSet::new();
        for pos in beam {
            if line[pos] == '^' {
                splits += 1;
                new_beam.extend([pos - 1, pos + 1]);
                timeline[pos - 1] += timeline[pos];
                timeline[pos + 1] += timeline[pos];
                timeline[pos] = 0;
            } else {
                new_beam.insert(pos);
            }
        }
        beam = new_beam;
    }
    println!("{splits}");
    println!("{}", timeline.into_iter().sum::<i64>())
}
