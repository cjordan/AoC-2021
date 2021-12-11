use aoc_2021::read_input;
use ndarray::Array2;

// Assume 5x5
const NUM_ROWS: usize = 5;
const NUM_COLS: usize = 5;

fn main() {
    let input = read_input(4);
    println!("The answer to part one is: {:?}", solve(&input));
    println!("The answer to part two is: {:?}", solve2(&input));
}

#[derive(Debug)]
struct Board(Array2<u8>);

impl Board {
    // Returns `None` if the draws don't make bingo, otherwise returns the sum
    // of the undrawn numbers.
    fn bingo(&self, draws: &[u8]) -> Option<u16> {
        let mut good_rows = self
            .0
            .rows()
            .into_iter()
            .filter(|row| row.iter().all(|r| draws.contains(r)));
        let mut good_cols = self
            .0
            .columns()
            .into_iter()
            .filter(|col| col.iter().all(|c| draws.contains(c)));
        if good_rows.next().or_else(|| good_cols.next()).is_some() {
            let total_sum = self.0.map(|&n| n as u16).sum();
            let marked_sum: u16 = self
                .0
                .iter()
                .filter(|n| draws.contains(n))
                .map(|&n| n as u16)
                .sum();
            Some(total_sum - marked_sum)
        } else {
            None
        }
    }
}

fn parse_input(i: &str) -> (Vec<u8>, Vec<Board>) {
    let mut draws = vec![];
    let mut boards = vec![];
    let mut board = vec![];
    for (i, line) in i.lines().enumerate() {
        if i == 0 {
            draws = line.split(',').map(|n| n.parse().unwrap()).collect();
            continue;
        }

        if line.trim().is_empty() {
            if !board.is_empty() {
                boards.push(Board(
                    Array2::from_shape_vec((NUM_ROWS, NUM_COLS), board).unwrap(),
                ));
            }
            board = vec![];
            continue;
        } else {
            for n in line.split_whitespace().map(|n| n.parse().unwrap()) {
                board.push(n);
            }
        }
    }
    // Get the last board.
    if !board.is_empty() {
        boards.push(Board(
            Array2::from_shape_vec((NUM_ROWS, NUM_COLS), board).unwrap(),
        ));
    }

    (draws, boards)
}

fn solve(input: &str) -> u32 {
    let (draws, boards) = parse_input(input);
    // No point checking before there are 5 draws.
    for i in 4..draws.len() {
        for board in &boards {
            if let Some(sum) = board.bingo(&draws[0..=i]) {
                return sum as u32 * draws[i] as u32;
            }
        }
    }
    panic!("oh noes");
}

fn solve2(input: &str) -> u32 {
    let (draws, boards) = parse_input(input);
    for i in (4..draws.len()).rev() {
        let results: Vec<Option<u16>> = boards.iter().map(|b| b.bingo(&draws[0..=i])).collect();
        let failed_boards: Vec<&Board> = boards
            .iter()
            .zip(results.iter())
            .filter(|(_, r)| r.is_none())
            .map(|(b, _)| b)
            .collect();
        let board = match failed_boards.len() {
            0 => continue,
            1 => failed_boards[0],
            _ => panic!("too many boards"),
        };
        let sum = board.bingo(&draws[0..=i + 1]).unwrap() as u32;
        return sum * draws[i + 1] as u32;
    }
    panic!("oh noes");
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_data() -> &'static str {
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19
        
         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6
        
        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7"
    }

    #[test]
    fn parse_correctly() {
        let input = get_test_data();
        let (draws, boards) = parse_input(input);
        assert_eq!(draws[0], 7);
        assert_eq!(*draws.last().unwrap(), 1);
        assert_eq!(boards.len(), 3);
        assert_eq!(boards[0].0[(2, 2)], 14);
        assert_eq!(boards[1].0[(2, 2)], 7);
        assert_eq!(boards[2].0[(2, 2)], 23);
    }

    #[test]
    fn example() {
        let input = get_test_data();
        assert_eq!(solve(input), 4512);
    }

    #[test]
    fn example2() {
        let input = get_test_data();
        assert_eq!(solve2(input), 1924);
    }
}
