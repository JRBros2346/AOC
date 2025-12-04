fn main() {
    let grid: &str = include_str!("input/day04.txt").trim();
    let total = grid.chars().filter(|&c| c == '@').count();
    let mut grid: Vec<Vec<_>> = grid.lines().map(|r| r.chars().collect()).collect();
    redo_grid(&mut grid);
    println!(
        "{}",
        total
            - grid
                .iter()
                .flat_map(|r| r.iter().filter(|&&e| e == '@'))
                .count()
    );
    while redo_grid(&mut grid) {}
    println!(
        "{}",
        total
            - grid
                .iter()
                .flat_map(|r| r.iter().filter(|&&e| e == '@'))
                .count()
    );
}

fn redo_grid(grid: &mut Vec<Vec<char>>) -> bool {
    let mut flag = false;
    let mut new_grid = grid.to_vec();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '@' && forkable(grid, i, j) {
                new_grid[i][j] = '.';
                flag = true;
            }
        }
    }
    *grid = new_grid;
    flag
}

fn forkable(grid: &[Vec<char>], i: usize, j: usize) -> bool {
    let mut cnt = 0;
    if let Some(row) = grid.get(i.wrapping_sub(1)) {
        if let Some(&e) = row.get(j.wrapping_sub(1))
            && e == '@'
        {
            cnt += 1;
        }
        cnt += (row[j] == '@') as usize;
        if let Some(&e) = row.get(j + 1)
            && e == '@'
        {
            cnt += 1;
        }
    }
    if let Some(&e) = grid[i].get(j.wrapping_sub(1))
        && e == '@'
    {
        cnt += 1;
    }
    if let Some(&e) = grid[i].get(j + 1)
        && e == '@'
    {
        cnt += 1;
    }
    if let Some(row) = grid.get(i + 1) {
        if let Some(&e) = row.get(j.wrapping_sub(1))
            && e == '@'
        {
            cnt += 1;
        }
        cnt += (row[j] == '@') as usize;
        if let Some(&e) = row.get(j + 1)
            && e == '@'
        {
            cnt += 1;
        }
    }
    cnt < 4
}
