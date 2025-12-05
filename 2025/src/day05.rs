use std::ops::RangeInclusive;

fn main() {
    let (fresh, available) = include_str!("input/day05.txt")
        .trim()
        .split_once("\r\n\r\n")
        .unwrap();
    let mut fresh = fresh
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .map(|(x, y)| x.parse::<usize>().unwrap()..=y.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    fresh.sort_unstable_by_key(|k| (*k.start(), *k.end()));
    let fresh: Vec<RangeInclusive<usize>> = fresh.into_iter().fold(vec![], |mut fresh, range| {
        if let Some(last) = fresh.last()
            && last.end() >= range.start()
        {
            let last = fresh.pop().unwrap();
            fresh.push(*last.start()..=*last.end().max(range.end()));
            return fresh;
        }
        fresh.push(range);
        fresh
    });
    let mut cnt = 0;
    for a in available.lines() {
        let a = a.parse::<usize>().unwrap();
        let (Ok(i) | Err(i)) = fresh.binary_search_by_key(&a, |r| *r.start());
        if let Some(r) = fresh.get(i.wrapping_sub(1))
            && r.contains(&a)
        {
            cnt += 1;
        }
    }
    println!("{cnt}");
    println!("{}", fresh.into_iter().fold(0, |acc, r| acc + r.count()));
}
