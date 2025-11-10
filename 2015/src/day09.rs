use std::collections::HashMap;

fn main() {
    let input = include_str!("input/day09.txt").trim();
    let mut dist = HashMap::new();
    for line in input.trim().lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let from = parts[0];
        let to = parts[2];
        let d: u32 = parts[4].parse().unwrap();
        dist.insert((from, to), d);
        dist.insert((to, from), d);
    }
    let mut cities = Vec::new();
    for (a, b) in dist.keys() {
        if !cities.contains(a) {
            cities.push(*a);
        }
        if !cities.contains(b) {
            cities.push(*b);
        }
    }
    let mut shortest = u32::MAX;
    let mut longest = 0;
    let mut perm = cities.clone();
    permute(&mut perm, 0, &dist, &mut shortest, &mut longest);
    println!("{shortest}");
    println!("{longest}");
}

fn permute(
    cities: &mut Vec<&str>,
    start: usize,
    dist: &HashMap<(&str, &str), u32>,
    shortest: &mut u32,
    longest: &mut u32,
) {
    if start == cities.len() {
        let mut total = 0;
        for w in cities.windows(2) {
            total += dist.get(&(w[0], w[1])).unwrap();
        }
        if total < *shortest {
            *shortest = total;
        }
        if total > *longest {
            *longest = total;
        }
    } else {
        for i in start..cities.len() {
            cities.swap(start, i);
            permute(cities, start + 1, dist, shortest, longest);
            cities.swap(start, i);
        }
    }
}
