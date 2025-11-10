use std::collections::HashMap;

fn main() {
    let mut edges = HashMap::new();
    for edge in include_str!("input/day07.txt").trim().lines() {
        let (from, to) = edge.split_once(" -> ").unwrap();
        edges.insert(to, from);
    }

    fn output<'a>(
        edges: &'a HashMap<&str, &str>,
        node: &'a str,
        memo: &mut HashMap<&'a str, u16>,
    ) -> u16 {
        if let Ok(num) = node.parse::<u16>() {
            return num;
        }
        if let Some(&cached) = memo.get(node) {
            return cached;
        }
        let edge = edges.get(node).unwrap();
        let parts: Vec<&str> = edge.split_whitespace().collect();
        let value = match parts.len() {
            1 => output(edges, parts[0], memo),
            2 => {
                if parts[0] == "NOT" {
                    !output(edges, parts[1], memo)
                } else {
                    unreachable!();
                }
            }
            3 => {
                if parts[1] == "AND" {
                    output(edges, parts[0], memo) & output(edges, parts[2], memo)
                } else if parts[1] == "OR" {
                    output(edges, parts[0], memo) | output(edges, parts[2], memo)
                } else if parts[1] == "LSHIFT" {
                    output(edges, parts[0], memo) << parts[2].parse::<u16>().unwrap()
                } else if parts[1] == "RSHIFT" {
                    output(edges, parts[0], memo) >> parts[2].parse::<u16>().unwrap()
                } else {
                    unreachable!();
                }
            }
            _ => unreachable!(),
        };
        memo.insert(node, value);
        memo[node]
    }

    let a = output(&edges, "a", &mut HashMap::new());
    println!("{a}");
    let a = a.to_string();
    edges.insert("b", a.as_str());
    let a = output(&edges, "a", &mut HashMap::new());
    println!("{a}");
}
