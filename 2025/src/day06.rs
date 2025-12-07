fn main() {
    let mut input: Vec<Vec<&str>> = include_str!("input/day06.txt")
        .trim()
        .lines()
        .map(|l| l.split_whitespace().collect())
        .collect();
    let ops = input.pop().unwrap();
    let result = (0..ops.len())
        .map(|i| match ops[i] {
            "+" => input
                .iter()
                .map(|r| r[i].parse::<usize>().unwrap())
                .sum::<usize>(),
            "*" => input
                .iter()
                .map(|r| r[i].parse::<usize>().unwrap())
                .product::<usize>(),
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    println!("{}", result.iter().sum::<usize>());

    let mut input = include_str!("input/day06.txt")
        .trim()
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let ops = input
        .pop()
        .unwrap()
        .into_iter()
        .filter(|&c| c != ' ')
        .collect::<Vec<_>>();
    let mut refac = vec![];
    let mut tmp = vec![];
    for i in 0..input[0].len() {
        if let Ok(n) = input
            .iter()
            .filter_map(|r| if r[i] == ' ' { None } else { Some(r[i]) })
            .collect::<String>()
            .parse::<usize>()
        {
            tmp.push(n);
        } else if !tmp.is_empty() {
            refac.push(tmp);
            tmp = vec![];
        }
    }
    if !tmp.is_empty() {
        refac.push(tmp);
    }
    let result = (0..ops.len())
        .map(|i| match ops[i] {
            '+' => refac[i].iter().sum::<usize>(),
            '*' => refac[i].iter().product::<usize>(),
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    println!("{}", result.iter().sum::<usize>());
}
