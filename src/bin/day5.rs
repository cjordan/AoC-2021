use aoc_2021::read_input;
use ndarray::Array2;

fn main() {
    let input = read_input(5);
    println!("The answer to part one is: {:?}", solve(&input));
    println!("The answer to part two is: {:?}", solve2(&input));
}

#[derive(Debug)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

fn parse_input(i: &str) -> Vec<Line> {
    i.lines()
        .map(|l| {
            let mut pairs = l.split(" -> ");
            let first_pair = pairs.next().unwrap();
            let mut coords = first_pair.split(',');
            let x1 = coords.next().unwrap().parse().unwrap();
            let y1 = coords.next().unwrap().parse().unwrap();
            let second_pair = pairs.next().unwrap();
            let mut coords = second_pair.split(',');
            let x2 = coords.next().unwrap().parse().unwrap();
            let y2 = coords.next().unwrap().parse().unwrap();
            Line { x1, y1, x2, y2 }
        })
        .collect()
}

fn solve(input: &str) -> usize {
    let lines: Vec<Line> = parse_input(input)
        .into_iter()
        .filter(|l| l.x1 == l.x2 || l.y1 == l.y2)
        .collect();
    let max_dims = lines.iter().fold((0, 0), |acc, line| {
        let max_x = line.x1.max(line.x2) as usize;
        let max_y = line.y1.max(line.y2) as usize;
        (acc.0.max(max_x), acc.1.max(max_y))
    });

    let mut grid: Array2<u8> = Array2::zeros((max_dims.1 + 1, max_dims.0 + 1));
    lines.into_iter().for_each(|l| {
        // Which dimension is changing?
        if l.x1 == l.x2 {
            let y_small = l.y1.min(l.y2);
            let y_large = l.y1.max(l.y2);
            for y in y_small..=y_large {
                grid[(y, l.x1)] += 1;
            }
        } else {
            let x_small = l.x1.min(l.x2);
            let x_large = l.x1.max(l.x2);
            for x in x_small..=x_large {
                grid[(l.y1, x)] += 1;
            }
        }
    });

    grid.into_iter().filter(|&g| g > 1).count()
}

fn solve2(input: &str) -> usize {
    let lines: Vec<Line> = parse_input(input).into_iter().collect();
    let max_dims = lines.iter().fold((0, 0), |acc, line| {
        let max_x = line.x1.max(line.x2) as usize;
        let max_y = line.y1.max(line.y2) as usize;
        (acc.0.max(max_x), acc.1.max(max_y))
    });

    let mut grid: Array2<u8> = Array2::zeros((max_dims.1 + 1, max_dims.0 + 1));
    lines.into_iter().for_each(|l| {
        let x_small = l.x1.min(l.x2);
        let x_large = l.x1.max(l.x2);
        let y_small = l.y1.min(l.y2);
        let y_large = l.y1.max(l.y2);

        // Which dimension is changing?
        match (l.x1 == l.x2, l.y1 == l.y2) {
            (true, false) => {
                for y in y_small..=y_large {
                    grid[(y, l.x1)] += 1;
                }
            }
            (false, true) => {
                for x in x_small..=x_large {
                    grid[(l.y1, x)] += 1;
                }
            }
            (false, false) => {
                let mut x_iter: Vec<_> = (x_small..=x_large).into_iter().collect();
                let x_iter = if l.x1 > l.x2 {
                    x_iter.reverse();
                    x_iter.into_iter()
                } else {
                    x_iter.into_iter()
                };
                let mut y_iter: Vec<_> = (y_small..=y_large).into_iter().collect();
                let y_iter = if l.y1 > l.y2 {
                    y_iter.reverse();
                    y_iter.into_iter()
                } else {
                    y_iter.into_iter()
                };

                for (x, y) in x_iter.zip(y_iter) {
                    grid[(y, x)] += 1;
                }
            }
            (true, true) => unreachable!(),
        }
    });

    grid.into_iter().filter(|&g| g > 1).count()
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_data() -> &'static str {
        r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#
    }

    #[test]
    fn example() {
        let input = get_test_data();
        assert_eq!(solve(input), 5);
    }

    #[test]
    fn example2() {
        let input = get_test_data();
        assert_eq!(solve2(input), 12);
    }
}
