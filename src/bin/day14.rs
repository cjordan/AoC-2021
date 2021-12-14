use aoc_2021::read_input;
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let input = read_input(14);

    println!("The answer to part one is: {:?}", solve(&input));
    println!("The answer to part two is: {:?}", solve2(&input, 40));
}

fn parse_input(input: &str) -> (String, BTreeMap<[char; 2], char>) {
    let mut lines = input.lines();
    let template = lines.next().unwrap().to_string();
    lines.next();

    let mut map = BTreeMap::new();
    for line in lines {
        let mut split = line.split(" -> ");
        let mut pair = split.next().unwrap().chars();
        let c1 = pair.next().unwrap();
        let c2 = pair.next().unwrap();
        let element = split.next().unwrap().chars().next().unwrap();
        map.insert([c1, c2], element);
    }
    (template, map)
}

fn solve(input: &str) -> usize {
    let (template, map) = parse_input(input);

    let mut chain = template;
    let mut chain2 = String::new();
    let mut c1: char;
    for _ in 0..10 {
        let mut chars = chain.chars();
        let mut c2 = chars.next().unwrap();
        for _ in 0..chain.len() - 1 {
            c1 = c2;
            c2 = chars.next().unwrap();
            let element = map[&[c1, c2]];
            chain2.push(c1);
            chain2.push(element);
        }
        chain.clear();
        chain.push_str(&chain2);
        chain.push(c2);
        chain2.clear();
    }

    // This is awful.
    let mut chars: Vec<char> = chain.chars().collect();
    chars.sort_unstable();
    let mut unique_chars = BTreeSet::new();
    for char in &chars {
        unique_chars.insert(*char);
    }
    let mut least = usize::MAX;
    let mut most = 0;
    for char in unique_chars {
        let count = chars.iter().filter(|&&c| c == char).count();
        if count > most {
            most = count;
        }
        if count < least {
            least = count;
        }
    }
    most - least
}

fn solve2(input: &str, num_steps: usize) -> usize {
    let (template, map) = parse_input(input);
    let mut unique_chars = BTreeMap::new();
    for key in map.keys() {
        unique_chars.entry(key[0]).or_insert(0);
        unique_chars.entry(key[1]).or_insert(0);
    }

    let mut current: BTreeMap<[char; 2], usize> = map.iter().map(|(k, _)| (*k, 0)).collect();
    let mut next = current.clone();

    let str_len = template.chars().count();
    let mut chars = template.chars();
    let mut c2 = chars.next().unwrap();
    for _ in 0..str_len - 1 {
        let c1 = c2;
        c2 = chars.next().unwrap();
        current.entry([c1, c2]).and_modify(|r| *r += 1);
    }

    for _ in 0..num_steps {
        for u in unique_chars.values_mut() {
            *u = 0;
        }

        for (k, &v) in current.iter().filter(|(_, &v)| v > 0) {
            let c = map[k];

            next.entry([k[0], c]).and_modify(|r| *r += v);
            next.entry([c, k[1]]).and_modify(|r| *r += v);

            unique_chars.entry(k[0]).and_modify(|r| *r += v);
            unique_chars.entry(c).and_modify(|r| *r += v);
        }
        unique_chars.entry(c2).and_modify(|r| *r += 1);

        for (c, n) in current.iter_mut().zip(next.iter_mut()) {
            *c.1 = *n.1;
            *n.1 = 0;
        }
    }

    let mut least = usize::MAX;
    let mut most = 0;
    for (_, v) in unique_chars.into_iter() {
        if v > most {
            most = v;
        }
        if v < least {
            least = v;
        }
    }
    most - least
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_data() -> &'static str {
        "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"
    }

    #[test]
    fn example() {
        let input = get_test_data();
        assert_eq!(solve(input), 1588);
    }

    #[test]
    fn example2() {
        let input = get_test_data();
        assert_eq!(solve2(input, 10), 1588);
        assert_eq!(solve2(input, 40), 2188189693529);
    }
}
