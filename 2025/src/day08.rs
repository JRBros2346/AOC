use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

fn main() {
    let junctions: Vec<[isize; 3]> = include_str!("input/day08.txt")
        .trim()
        .lines()
        .map(|j| {
            j.split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect();
    let mut dists = BinaryHeap::new();
    for i in 0..junctions.len() {
        for j in i + 1..junctions.len() {
            dists.push(Reverse((
                (junctions[i][0] - junctions[j][0]).pow(2)
                    + (junctions[i][1] - junctions[j][1]).pow(2)
                    + (junctions[i][2] - junctions[j][2]).pow(2),
                (i, j),
            )));
        }
    }
    let mut circuits = (0..junctions.len()).collect::<Vec<_>>();

    let mut edges = 0;

    for _ in 0..1000 {
        let Reverse((_, (i, j))) = dists.pop().unwrap();
        let i = find(&circuits, i);
        let j = find(&circuits, j);
        if i != j {
            circuits[i] = j;
            edges += 1;
        }
    }

    let mut sets = HashMap::new();
    for jun in 0..junctions.len() {
        sets.entry(find(&circuits, jun))
            .or_insert(HashSet::new())
            .insert(jun);
    }
    let mut sets = sets.values().map(|set| set.len()).collect::<Vec<_>>();
    sets.sort_unstable_by_key(|k| usize::MAX - k);
    println!("{}", sets[0] * sets[1] * sets[2]);

    loop {
        let Reverse((_, (i, j))) = dists.pop().unwrap();
        let a = find(&circuits, i);
        let b = find(&circuits, j);
        if a != b {
            circuits[a] = b;
            edges += 1;
        }
        if edges == junctions.len() - 1 {
            println!("{}", junctions[i][0] * junctions[j][0]);
            break;
        }
    }
}

fn find(circuits: &[usize], mut i: usize) -> usize {
    while i != circuits[i] {
        i = circuits[i];
    }
    i
}
