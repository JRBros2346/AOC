const ELAPSED: usize = 2503;

fn main() {
    let mut max = 0;
    let mut deers = Vec::new();
    for line in include_str!("input/day14.txt").trim().lines() {
        let (_, extra) = line.split_once(" can fly ").unwrap();
        let (speed, extra) = extra.split_once(" km/s for ").unwrap();
        let speed = speed.parse::<usize>().unwrap();
        let (flight, extra) = extra
            .split_once(" seconds, but then must rest for ")
            .unwrap();
        let flight = flight.parse::<usize>().unwrap();
        let (rest, _) = extra.split_once(" seconds.").unwrap();
        let rest = rest.parse::<usize>().unwrap();
        let distance =
            speed * (ELAPSED / (flight + rest) * flight + (ELAPSED % (flight + rest)).min(flight));
        max = max.max(distance);
        deers.push([speed, flight, rest]);
    }
    println!("{max}");
    let mut state = vec![(0, 0); deers.len()];
    for t in 1..=ELAPSED {
        for (i, &[speed, flight, rest]) in deers.iter().enumerate() {
            let cycle = flight + rest;
            let pos = (t - 1) % cycle;
            if pos < flight {
                state[i].0 += speed;
            }
        }
        let max = state.iter().map(|(d, _)| *d).max().unwrap();
        for s in &mut state {
            if s.0 == max {
                s.1 += 1;
            }
        }
    }
    println!("{}", state.iter().map(|(_, p)| *p).max().unwrap());
}
