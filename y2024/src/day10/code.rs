#[allow(dead_code)]
fn load_data() -> Vec<Vec<usize>> {
    std::fs::read_to_string(file!().replace("/code.rs", "/data.txt"))
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|num| num.to_digit(10))
                .map(|d| d as usize)
                .collect()
        })
        .collect()
}

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub fn part1() -> Option<usize> {
    let map = load_data();

    let mut starts: Vec<(usize, usize)> = Vec::new();
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if map[r][c] == 0 {
                starts.push((r, c));
            }
        }
    }

    let paths_seen: &mut Vec<Vec<(usize, usize)>> = &mut Vec::new();

    for (i, start) in starts.into_iter().enumerate() {
        paths_seen.push(Vec::new());
        move_next(start.0, start.1, &map, &mut paths_seen[i], true);
    }

    Some(paths_seen.iter().map(|subvec| subvec.len()).sum())
}

pub fn part2() -> Option<usize> {
    let map = load_data();

    let mut starts: Vec<(usize, usize)> = Vec::new();
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if map[r][c] == 0 {
                starts.push((r, c));
            }
        }
    }

    let paths_seen: &mut Vec<Vec<(usize, usize)>> = &mut Vec::new();

    for (i, start) in starts.into_iter().enumerate() {
        paths_seen.push(Vec::new());
        move_next(start.0, start.1, &map, &mut paths_seen[i], false);
    }
    Some(paths_seen.iter().map(|subvec| subvec.len()).sum())
}

fn move_next(
    r: usize,
    c: usize,
    map: &Vec<Vec<usize>>,
    seen: &mut Vec<(usize, usize)>,
    distinct: bool,
) -> usize {
    let mut local_found = 0;
    for dir in DIRECTIONS {
        let nr = r as isize + dir.0;
        let nc = c as isize + dir.1;
        if in_bounds(nr, nc, &map)
            && map[nr as usize][nc as usize] as isize - map[r as usize][c as usize] as isize == 1
        {
            if map[nr as usize][nc as usize] == 9 {
                if distinct {
                    if !seen.contains(&(nr as usize, nc as usize)) {
                        seen.push((nr as usize, nc as usize));
                    }
                } else {
                    seen.push((nr as usize, nc as usize));
                }
                continue;
            }
            local_found = move_next(nr as usize, nc as usize, &map, seen, distinct);
        }
    }
    return local_found;
}

fn in_bounds(r: isize, c: isize, map: &Vec<Vec<usize>>) -> bool {
    if r < 0 || c < 0 || r > map.len() as isize - 1 || c > map[0].len() as isize - 1 {
        return false;
    }
    true
}
