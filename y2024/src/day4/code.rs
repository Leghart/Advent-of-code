#[allow(dead_code)]
fn load_data() -> Vec<Vec<char>> {
    std::fs::read_to_string(file!().replace("/code.rs", "/data.txt"))
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn part1() -> Option<usize> {
    let grid = load_data();
    let word = "XMAS";
    let rows = grid.len();
    let cols = grid[0].len();
    let reverse_word: String = word.chars().rev().collect();

    let mut count = 0;

    let directions = [(0, 1), (1, 0), (1, 1), (1, -1)];

    for i in 0..rows {
        for j in 0..cols {
            for &(dx, dy) in &directions {
                let mut found_forward = true;
                let mut found_reverse = true;

                for k in 0..word.len() {
                    let x = i as isize + k as isize * dx;
                    let y = j as isize + k as isize * dy;

                    if x < 0 || x >= rows as isize || y < 0 || y >= cols as isize {
                        found_forward = false;
                        found_reverse = false;
                        break;
                    }

                    if grid[x as usize][y as usize] != word.chars().nth(k).unwrap() {
                        found_forward = false;
                    }

                    if grid[x as usize][y as usize] != reverse_word.chars().nth(k).unwrap() {
                        found_reverse = false;
                    }
                }

                if found_forward {
                    count += 1;
                }

                if found_reverse {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

pub fn part2() -> Option<usize> {
    let grid = load_data();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            let chars = vec![
                grid[i][j],
                grid[i - 1][j - 1],
                grid[i - 1][j + 1],
                grid[i + 1][j - 1],
                grid[i + 1][j + 1],
            ];

            if chars[0] == 'A'
                && (((chars[1] == 'M' && chars[4] == 'S') || (chars[1] == 'S' && chars[4] == 'M'))
                    && ((chars[2] == 'M' && chars[3] == 'S')
                        || (chars[2] == 'S' && chars[3] == 'M')))
            {
                count += 1
            }
        }
    }

    Some(count)
}
