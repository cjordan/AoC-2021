use aoc_2021::read_input;

fn main() {
    let input = read_input(1);
    let report: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();
    println!("The answer to part one is: {}", solve(&report));
    println!("The answer to part two is: {}", solve2(&report));
}

fn solve(report: &[u32]) -> u32 {
    let mut last = report[0];
    let mut greater_count = 0;
    for &num in &report[1..] {
        if num > last {
            greater_count += 1;
        }
        last = num;
    }
    greater_count
}

fn solve2(report: &[u32]) -> u32 {
    let mut n_minus_2 = report[0];
    let mut n_minus_1 = report[1];
    let mut last_sum = n_minus_2 + n_minus_1 + report[2];

    let mut greater_count = 0;
    for (i, &num) in report.iter().enumerate().skip(3) {
        n_minus_2 = report[i - 2];
        n_minus_1 = report[i - 1];
        let sum = n_minus_2 + n_minus_1 + num;

        if sum > last_sum {
            greater_count += 1;
        }
        last_sum = sum;
    }
    greater_count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let report = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(solve(&report), 7);
    }

    #[test]
    fn example2() {
        let report = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(solve2(&report), 5);
    }
}
