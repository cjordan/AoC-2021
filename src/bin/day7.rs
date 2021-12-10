use aoc_2021::read_input;

fn main() {
    let input = read_input(7);

    println!("The answer to part one is: {:?}", solve(&input));
    println!("The answer to part two is: {:?}", solve2(&input));
}

fn solve(input: &str) -> usize {
    let positions: Vec<i32> = input
        .split(',')
        .map(|n| n.trim_end().parse().unwrap())
        .collect();
    let last_pos = *positions.iter().max().unwrap();

    let mut cheapest = usize::MAX;
    for pos in 0..last_pos {
        let mut fuel_use = 0;
        for &pos2 in &positions {
            fuel_use += (pos - pos2).abs() as usize;
        }
        cheapest = cheapest.min(fuel_use);
    }
    cheapest
}

fn triag(n: usize) -> usize {
    (n * (n + 1)) / 2
}

fn solve2(input: &str) -> usize {
    let positions: Vec<i32> = input
        .split(',')
        .map(|n| n.trim_end().parse().unwrap())
        .collect();
    let last_pos = *positions.iter().max().unwrap();

    let mut cheapest = usize::MAX;
    for pos in 0..last_pos {
        let mut fuel_use = 0;
        for &pos2 in &positions {
            let diff = (pos - pos2).abs() as usize;
            fuel_use += triag(diff);
        }
        cheapest = cheapest.min(fuel_use);
    }
    cheapest
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_data() -> &'static str {
        "16,1,2,0,4,2,7,1,2,14"
    }

    #[test]
    fn example() {
        let input = get_test_data();
        assert_eq!(solve(input), 37);
    }

    #[test]
    fn example2() {
        let input = get_test_data();
        assert_eq!(solve2(input), 168);
    }
}
