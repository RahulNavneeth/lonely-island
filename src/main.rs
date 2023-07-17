use std::{env, io::BufRead};

type Island = Vec<Vec<char>>;

fn is_island(value: char) -> bool {
    value == '1'
}

fn is_valid(island: &Island, row: usize, col: usize) -> bool {
    let rows = island.len();
    let cols = island[0].len();
    row >= 0 as usize && row < rows && col >= 0 as usize && col < cols
}

fn find_island(island: &mut Island) -> u32 {
    let mut visited = Vec::<(usize, usize)>::new();
    let mut total_island: u32 = 0;
    island.iter().enumerate().for_each(|(i, grid)| {
        grid.iter().enumerate().for_each(|(j, cell)| {
            if is_island(*cell) && !visited.contains(&(i, j)) {
                let mut stack = Vec::<(usize, usize)>::new();
                stack.push((i, j));
                while let Some((x, y)) = stack.pop() {
                    let directions: Vec<(isize, isize)> = vec![
                        (0, 1),
                        (1, 0),
                        (0, -1),
                        (-1, 0),
                        (1, 1),
                        (1, -1),
                        (-1, 1),
                        (-1, -1),
                    ];
                    for (k, l) in directions.iter() {
                        let new_x = (x as isize + *k) as usize;
                        let new_y = (y as isize + *l) as usize;
                        if is_valid(&island, new_x, new_y) {
                            let new_cell = island[new_x][new_y];
                            if is_island(new_cell) {
                                if !visited.contains(&(new_x, new_y)) {
                                    visited.push((new_x, new_y));
                                    stack.push((new_x, new_y));
                                }
                            }
                        }
                    }
                }
                total_island += 1;
            };
        })
    });
    total_island
}

fn main() -> std::io::Result<()> {
    let mut island: Island = Vec::new();

    let args = env::args().collect::<Vec<String>>();
    let fs = std::fs::OpenOptions::new()
        .read(true)
        .open(args.get(1).map_or("data.txt", |f| &*f))?;
    let f = std::io::BufReader::new(fs);
    f.lines().for_each(|line| match line {
        Ok(value) => {
            let c = value
                .chars()
                .enumerate()
                .filter_map(|(key, f)| if key % 2 == 0 { Some(f) } else { None })
                .collect::<Vec<char>>();
            island.push(c);
        }
        Err(e) => println!("ERROR {}", e),
    });
    println!("{}", find_island(&mut island));
    Ok(())
}
