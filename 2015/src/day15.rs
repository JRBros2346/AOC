fn main() {
    let input = include_str!("input/day15.txt")
        .trim()
        .lines()
        .map(|line| {
            let (capacity, extra) = line
                .split_once(": capacity ")
                .unwrap()
                .1
                .split_once(", durability ")
                .unwrap();
            let (durability, extra) = extra.split_once(", flavor ").unwrap();
            let (flavor, extra) = extra.split_once(", texture ").unwrap();
            let (texture, calories) = extra.split_once(", calories ").unwrap();
            [
                capacity.parse::<isize>().unwrap(),
                durability.parse::<isize>().unwrap(),
                flavor.parse::<isize>().unwrap(),
                texture.parse::<isize>().unwrap(),
                calories.parse::<isize>().unwrap(),
            ]
        })
        .collect::<Vec<_>>();
    let n = input.len();
    let mut amounts = vec![0; n];
    let mut best = 0;
    let mut best_calories = 0;
    search(0, 100, &mut amounts, &input, &mut best, &mut best_calories);
    println!("{best}");
    println!("{best_calories}");
}

fn search(
    idx: usize,
    remaining: isize,
    amounts: &mut [isize],
    ingredients: &[[isize; 5]],
    best: &mut isize,
    best_calories: &mut isize,
) {
    let n = ingredients.len();
    if idx == n - 1 {
        amounts[idx] = remaining;
        let mut totals = [0; 4];
        let mut calories = 0;
        for (i, &amt) in amounts.iter().enumerate() {
            for p in 0..4 {
                totals[p] += amt * ingredients[i][p];
            }
            calories += amt * ingredients[i][4];
        }
        for t in &mut totals[0..4] {
            if *t <= 0 {
                *t = 0;
            }
        }
        let total = totals.into_iter().product::<isize>();
        if total > *best {
            *best = total
        }
        if calories == 500 && total > *best_calories {
            *best_calories = total
        }
        return;
    }
    for amt in 0..remaining {
        amounts[idx] = amt;
        search(
            idx + 1,
            remaining - amt,
            amounts,
            ingredients,
            best,
            best_calories,
        );
    }
}
