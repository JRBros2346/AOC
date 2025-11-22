fn main() {
    let mut people = Vec::new();
    let mut happy: Vec<Vec<isize>> = Vec::new();
    for line in include_str!("input/day13.txt").trim().lines() {
        let (person, extra) = line.split_once(" would ").unwrap();
        let (change, neighbor) = extra
            .split_once(" happiness units by sitting next to ")
            .unwrap();
        let neighbor = &neighbor[..neighbor.len() - 1];
        let (sign, units) = change.split_once(' ').unwrap();
        let units = match sign {
            "gain" => units.parse::<isize>().unwrap(),
            "lose" => -units.parse::<isize>().unwrap(),
            _ => panic!("Unexpected sign"),
        };
        if !people.contains(&person) {
            people.push(person);
            happy.resize(people.len(), vec![0; people.len()]);
            for row in happy.iter_mut() {
                row.resize(people.len(), 0);
            }
        }
        if !people.contains(&neighbor) {
            people.push(neighbor);
            happy.resize(people.len(), vec![0; people.len()]);
            for row in happy.iter_mut() {
                row.resize(people.len(), 0);
            }
        }
        happy[people.iter().position(|&p| p == person).unwrap()]
            [people.iter().position(|&p| p == neighbor).unwrap()] += units;
        happy[people.iter().position(|&p| p == neighbor).unwrap()]
            [people.iter().position(|&p| p == person).unwrap()] += units;
    }
    println!("{}", do_seating(&happy));
    for row in happy.iter_mut() {
        row.push(0);
    }
    happy.push(vec![0; happy.len() + 1]);
    println!("{}", do_seating(&happy));
}

fn do_seating(happy: &[Vec<isize>]) -> isize {
    let n = happy.len();
    let size = 1 << n;
    let mut dp = vec![vec![isize::MIN / 4; n]; size];
    dp[1][0] = 0;
    for mask in 0..size {
        for (u, row) in happy.iter().enumerate().take(n) {
            if mask & (1 << u) == 0 {
                continue;
            }
            let prev = dp[mask][u];
            if prev <= isize::MIN / 8 {
                continue;
            }
            for (v, e) in row.iter().enumerate().take(n) {
                if mask & (1 << v) != 0 {
                    continue;
                }
                let next = mask | (1 << v);
                let cand = prev + e;
                if cand > dp[next][v] {
                    dp[next][v] = cand;
                }
            }
        }
    }
    let full = (1 << n) - 1;
    let mut best = isize::MIN;
    for (u, row) in happy.iter().enumerate().take(n).skip(1) {
        let cand = dp[full][u] + row[0];
        if cand > best {
            best = cand;
        }
    }
    best
}
