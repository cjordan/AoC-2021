use std::collections::BTreeMap;

use aoc_2021::read_input;

fn main() {
    let input = read_input(12);

    println!("The answer to part one is: {:?}", solve(&input));
    println!("The answer to part two is: {:?}", solve2(&input));
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
enum Cave {
    Start,
    End,
    Small(String),
    Large(String),
}

fn to_cave_type(s: &str) -> Cave {
    if s == "start" {
        Cave::Start
    } else if s == "end" {
        Cave::End
    } else if s.to_lowercase() == s {
        Cave::Small(s.to_string())
    } else {
        Cave::Large(s.to_string())
    }
}

fn parse_input(input: &str) -> BTreeMap<Cave, Vec<Cave>> {
    let mut tree: BTreeMap<Cave, Vec<Cave>> = BTreeMap::new();
    input.lines().for_each(|l| {
        let mut elems = l.split('-');
        let first = to_cave_type(elems.next().unwrap());
        let last = to_cave_type(elems.next().unwrap());
        // Much anger.
        let (first, last) = if last == Cave::Start || first == Cave::End {
            (last, first)
        } else {
            (first, last)
        };

        let add_reverse = !(matches!(&first, &Cave::Start) || matches!(&last, &Cave::End));

        match tree.get_mut(&first) {
            Some(v) => v.push(last.clone()),
            None => {
                tree.insert(first.clone(), vec![last.clone()]);
            }
        }
        if add_reverse {
            match tree.get_mut(&last) {
                Some(v) => v.push(first),
                None => {
                    tree.insert(last, vec![first]);
                }
            }
        }
    });
    tree
}

fn path_to_string(path: &[&Cave]) -> String {
    let mut s = String::new();
    for p in path {
        match p {
            Cave::Small(name) | Cave::Large(name) => s.push_str(name),
            _ => (),
        }
    }
    s
}

fn walk(tree: &BTreeMap<Cave, Vec<Cave>>) -> usize {
    let mut paths = vec![];
    let mut possible_paths: Vec<Vec<&Cave>> = tree[&Cave::Start].iter().map(|p| vec![p]).collect();
    let mut possible_paths2 = vec![];
    while !possible_paths.is_empty() {
        for path in possible_paths.drain(..) {
            let steps = &tree[path.last().unwrap()];
            for step in steps {
                if matches!(step, Cave::End) {
                    let path_str = path_to_string(&path);
                    paths.push(path_str);
                    continue;
                }

                if matches!(step, Cave::Small(_)) && path.contains(&step) {
                    continue;
                } else {
                    let mut path = path.clone();
                    path.push(step);
                    possible_paths2.push(path);
                }
            }
        }
        possible_paths.append(&mut possible_paths2);
    }
    paths.len()
}

fn solve(input: &str) -> usize {
    let tree = parse_input(input);
    walk(&tree)
}

fn too_many_small_caves(step: &Cave, path: &[&Cave]) -> bool {
    let mut double_small_cave: Option<&Cave> = None;
    let mut step_cave_count = 0;
    let unique_small_caves = {
        let mut small_caves: Vec<&&Cave> = path
            .iter()
            .filter(|c| matches!(c, Cave::Small(_)))
            .collect();
        small_caves.sort_unstable();
        small_caves.dedup();
        small_caves
    };
    for small in unique_small_caves {
        let c = path.iter().filter(|&c| c == small).count();
        if *small == step {
            step_cave_count = c;
        }
        match c {
            0 | 1 => (),
            2 => {
                if double_small_cave.is_some() {
                    return true;
                } else {
                    double_small_cave = Some(small);
                }
            }
            _ => {
                return true;
            }
        }
    }
    if let Some(cave) = double_small_cave {
        if cave == step || step_cave_count == 1 {
            return true;
        }
    }
    false
}

fn walk2(tree: &BTreeMap<Cave, Vec<Cave>>) -> usize {
    let mut paths = vec![];
    let mut possible_paths: Vec<Vec<&Cave>> = tree[&Cave::Start].iter().map(|p| vec![p]).collect();
    let mut possible_paths2 = vec![];
    while !possible_paths.is_empty() {
        for path in possible_paths.drain(..) {
            let steps = &tree[path.last().unwrap()];
            for step in steps {
                if matches!(step, Cave::End) {
                    let path_str = path_to_string(&path);
                    paths.push(path_str);
                    continue;
                }

                if matches!(step, Cave::Small(_)) && too_many_small_caves(step, &path) {
                    continue;
                } else {
                    let mut path = path.clone();
                    path.push(step);
                    possible_paths2.push(path);
                }
            }
        }
        possible_paths.append(&mut possible_paths2);
    }
    paths.len()
}

fn solve2(input: &str) -> usize {
    let tree = parse_input(input);
    walk2(&tree)
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_data() -> &'static str {
        "start-A
start-b
A-c
A-b
b-d
A-end
b-end"
    }

    fn get_test_data2() -> &'static str {
        "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"
    }

    fn get_test_data3() -> &'static str {
        "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"
    }

    #[test]
    fn example() {
        let input = get_test_data();
        assert_eq!(solve(input), 10);
    }

    #[test]
    fn example2() {
        let input = get_test_data2();
        assert_eq!(solve(input), 19);
    }

    #[test]
    fn example3() {
        let input = get_test_data3();
        assert_eq!(solve(input), 226);
    }

    #[test]
    fn example4() {
        let input = get_test_data();
        assert_eq!(solve2(input), 36);
    }

    #[test]
    fn example5() {
        let input = get_test_data2();
        assert_eq!(solve2(input), 103);
    }

    #[test]
    fn example6() {
        let input = get_test_data3();
        assert_eq!(solve2(input), 3509);
    }
}
