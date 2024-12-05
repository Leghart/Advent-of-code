use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
fn load_data() -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let data = std::fs::read_to_string(file!().replace("/code.rs", "/data.txt")).unwrap();

    let mut b = data.split("\n\n");
    let rules_raw = b.next().unwrap_or_default();
    let updates_raw = b.next().unwrap_or_default();

    let rules: Vec<(usize, usize)> = rules_raw
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .filter_map(|entry| {
            let parts: Vec<&str> = entry.split('|').collect();
            if parts.len() == 2 {
                Some((parts[0].parse().ok()?, parts[1].parse().ok()?))
            } else {
                None
            }
        })
        .collect();

    let updates: Vec<Vec<usize>> = updates_raw
        .lines()
        .map(|line| {
            line.split(',')
                .filter_map(|x| x.parse::<usize>().ok())
                .collect()
        })
        .collect();

    (rules, updates)
}

pub fn part1() -> Option<usize> {
    let (rules, updates) = load_data();
    let mut count = 0;
    for update in &updates {
        let mut incorrect = false;
        for rule in &rules {
            if update.contains(&rule.0) && update.contains(&rule.1) {
                if update.iter().position(|&x| x == rule.0)
                    >= update.iter().position(|&x| x == rule.1)
                {
                    incorrect = true;
                }
            }
        }
        if !incorrect {
            if update.len() % 2 == 0 {
                panic!()
            } else {
                count += update[update.len() / 2];
            }
        }
    }

    Some(count)
}

pub fn part2() -> Option<usize> {
    let (rules, updates) = load_data();
    let mut count = 0;
    let mut all_rules_with_update: Vec<(Vec<usize>, Vec<(usize, usize)>)> = Vec::new();
    for update in &updates {
        let mut incorrect = false;
        let mut all_rules: Vec<(usize, usize)> = Vec::new();
        for rule in &rules {
            if update.contains(&rule.0) && update.contains(&rule.1) {
                all_rules.push(rule.clone());
                if update.iter().position(|&x| x == rule.0)
                    >= update.iter().position(|&x| x == rule.1)
                {
                    incorrect = true;
                }
            }
        }
        if incorrect {
            all_rules_with_update.push((update.clone(), all_rules));
        }
    }

    for update_rules in all_rules_with_update {
        let result = topological_sort(update_rules.0, update_rules.1);

        if result.len() % 2 == 0 {
            panic!()
        } else {
            count += result[result.len() / 2];
        }
    }

    Some(count)
}

fn topological_sort(updates: Vec<usize>, rules: Vec<(usize, usize)>) -> Vec<usize> {
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();

    for (u, v) in rules {
        graph.entry(u).or_insert_with(Vec::new).push(v);
    }

    let mut visited = HashSet::new();
    let mut temp_visited = HashSet::new();
    let mut result = Vec::new();

    for &update in &updates {
        if !visited.contains(&update) {
            dfs(update, &graph, &mut visited, &mut result, &mut temp_visited);
        }
    }

    result.reverse();

    result
}

fn dfs(
    node: usize,
    graph: &HashMap<usize, Vec<usize>>,
    visited: &mut HashSet<usize>,
    result: &mut Vec<usize>,
    temp_visited: &mut HashSet<usize>,
) {
    if visited.contains(&node) {
        return;
    }

    temp_visited.insert(node);

    if let Some(neighbors) = graph.get(&node) {
        for &neighbor in neighbors {
            dfs(neighbor, graph, visited, result, temp_visited);
        }
    }

    temp_visited.remove(&node);
    visited.insert(node);
    result.push(node);

    return;
}
