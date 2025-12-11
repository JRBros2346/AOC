use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("input/day09.txt")
        .trim()
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(a, b)| (a.parse::<isize>().unwrap(), b.parse::<isize>().unwrap()))
        .collect::<Vec<_>>();
    let mut max = 0;
    for &a in &input {
        for &b in &input {
            max = max.max(Rect::new(a, b).area());
        }
    }
    println!("{max}");
    let mut walls = input
        .windows(2)
        .chain(Some(
            &[*input.last().unwrap(), *input.first().unwrap()] as &[_]
        ))
        .filter_map(|a| {
            if a[0].0 == a[1].0 {
                Some((a[0].0, a[0].1.min(a[1].1)..=a[0].1.max(a[1].1)))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    walls.sort_unstable_by_key(|&(k, _)| k);
    let polygon = Polygon::new(input.clone());
    max = 0;
    for &a in &input {
        'outer: for &b in &input {
            // let xaxis = (a.0.min(b.0) + 1)..a.0.max(b.0);
            // let yaxis = (a.1.min(b.1) + 1)..a.1.max(b.1);
            if polygon.inside(a.0, b.1) && polygon.inside(b.0, a.1) {
                // for i in xaxis {
                //     if !raycast(i, a.1, &walls) || !raycast(i, b.1, &walls) {
                //         continue 'outer;
                //     }
                // }
                // for j in yaxis {
                //     if !raycast(a.0, j, &walls) || !raycast(b.0, j, &walls) {
                //         continue 'outer;
                //     }
                // }
                max = max.max(Rect::new(a, b).area());
            }
        }
    }
    println!("{max}");
}

struct Rect(isize, isize, isize, isize);
impl Rect {
    fn new(a: (isize, isize), b: (isize, isize)) -> Self {
        Self(
            a.0.min(b.0),
            a.1.min(b.1),
            a.0.max(b.0) + 1,
            a.1.max(b.1) + 1,
        )
    }
    fn area(&self) -> isize {
        (self.2 - self.0) * (self.3 - self.1)
    }
}

struct Polygon {
    vert: Vec<(isize, (isize, isize))>,
    horz: Vec<((isize, isize), isize)>,
}
impl Polygon {
    fn new(mut points: Vec<(isize, isize)>) -> Self {
        points.insert(0, *points.last().unwrap());
        points.push(points[1]);
        let points = points
            .windows(3)
            .map(|corner| {
                let pivot = corner[1];
                let pseudo = if pivot.0 == corner[0].0 {
                    (corner[2].0, corner[0].1)
                } else {
                    (corner[0].0, corner[2].1)
                };
                (
                    if pseudo.0 < pivot.0 {
                        pivot.0
                    } else {
                        pivot.0 + 1
                    },
                    if pseudo.1 < pivot.1 {
                        pivot.1
                    } else {
                        pivot.1 + 1
                    },
                )
            })
            .collect::<Vec<_>>();
        let mut vert = vec![];
        let mut horz = vec![];
        for i in 0..points.len() {
            let a = points[i];
            let b = points[(i + 1) % points.len()];
            if a.0 == b.0 {
                let y0 = a.1.min(b.1);
                let y1 = a.1.max(b.1);
                if y0 != y1 {
                    vert.push((a.0, (y0, y1)));
                }
            } else if a.1 == b.1 {
                let x0 = a.0.min(b.0);
                let x1 = a.0.max(b.0);
                if x0 != x1 {
                    horz.push(((x0, x1), a.1));
                }
            } else {
                unreachable!("Polygon is bad {a:?} {b:?}");
            }
        }
        vert.sort_unstable_by_key(|(k, _)| *k);
        horz.sort_unstable_by_key(|(_, k)| *k);
        Self { vert, horz }
    }
    fn inside(&self, x: isize, y: isize) -> bool {
        let mut inside = false;
        for &(vx, (y0, y1)) in &self.vert {
            if y >= y0 && y < y1 && vx > x {
                inside = !inside;
            }
        }
        inside
    }
}

fn raycast(px: isize, py: isize, walls: &[(isize, RangeInclusive<isize>)]) -> bool {
    let mut inside = false;
    for (x, rng) in walls {
        if x == &px {
            if rng.contains(&py) {
                return true;
            } else {
                continue;
            }
        }
        if x <= &px {
            continue;
        }
        if rng.contains(&py) {
            inside = !inside;
        }
    }
    inside
}
