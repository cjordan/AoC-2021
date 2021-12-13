use aoc_2021::read_input;
use ndarray::prelude::*;

fn main() {
    let input = read_input(13);

    println!("The answer to part one is: {:?}", solve(&input));
    println!("The display for part two is:");
    solve2(&input);
}

#[derive(Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

fn parse_input(input: &str) -> (Array2<u8>, Vec<Fold>) {
    let mut coords: Vec<(usize, usize)> = vec![];

    let mut lines = input.lines();
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        let mut split = line.split(',');
        coords.push((
            split.next().unwrap().parse().unwrap(),
            split.next().unwrap().parse().unwrap(),
        ));
    }

    let mut folds = vec![];
    for line in lines {
        let mut split = line.split("fold along ");
        split.next();
        let mut coord = split.next().unwrap().split('=');
        let dir = coord.next().unwrap();
        let num = coord.next().unwrap().parse().unwrap();
        let fold = match dir {
            "x" => Fold::X(num),
            "y" => Fold::Y(num),
            _ => panic!("uh oh"),
        };
        folds.push(fold);
    }

    let max_dim = coords.iter().fold((0, 0), |acc, coord| {
        (acc.0.max(coord.0), acc.1.max(coord.1))
    });
    let mut grid = Array2::from_elem((max_dim.1 + 1, max_dim.0 + 1), 0);
    for (x, y) in coords {
        grid[(y, x)] += 1;
    }

    (grid, folds)
}

fn fold_grid(mut grid: ArrayViewMut2<u8>, folds: Vec<Fold>, num_folds: usize) -> (usize, usize) {
    let mut grid_dim = grid.dim();
    for fold in folds.into_iter().take(num_folds) {
        match &fold {
            Fold::X(n) => {
                for mut row in grid.rows_mut() {
                    for i in 0..*n {
                        let v = row.get_mut(grid_dim.1 - 1 - i).unwrap();
                        let is_set = *v == 1;
                        *v = 0;
                        if is_set {
                            row[i] = 1;
                        }
                    }
                }
            }
            Fold::Y(n) => {
                for mut col in grid.columns_mut() {
                    for i in 0..*n {
                        let v = col.get_mut(grid_dim.0 - 1 - i).unwrap();
                        let is_set = *v == 1;
                        *v = 0;
                        if is_set {
                            col[i] = 1;
                        }
                    }
                }
            }
        }

        match &fold {
            Fold::X(_) => grid_dim.1 = (grid_dim.1 - 1) / 2,
            Fold::Y(_) => grid_dim.0 = (grid_dim.0 - 1) / 2,
        }
    }
    grid_dim
}

fn solve(input: &str) -> usize {
    let (mut grid, folds) = parse_input(input);
    let num_folds = 1;
    fold_grid(grid.view_mut(), folds, num_folds);

    grid.into_iter().filter(|&g| g > 0).count()
}

fn solve2(input: &str) {
    let (mut grid, folds) = parse_input(input);
    let num_folds = folds.len();
    let folded_dims = fold_grid(grid.view_mut(), folds, num_folds);

    let mut chars = Array2::from_elem(folded_dims, ' ');
    chars
        .outer_iter_mut()
        .zip(grid.outer_iter())
        .for_each(|(mut chars, grid)| {
            chars.iter_mut().zip(grid.iter()).for_each(|(chars, grid)| {
                *chars = if *grid == 1 { '#' } else { ' ' };
            });
        });

    // Could just print chars here, but making them strings makes it easier to
    // read (I read the answer wrong once).
    // println!("{:?}", &chars);

    let mut strings = vec![String::new(); chars.dim().0];
    for (string, row) in strings.iter_mut().zip(chars.rows()) {
        for char in row {
            string.push(*char);
        }
    }
    println!("{:#?}", &strings);
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_data() -> &'static str {
        "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"
    }

    #[test]
    fn example() {
        let input = get_test_data();
        assert_eq!(solve(input), 17);
    }

    #[test]
    fn example2() {
        let input = get_test_data();
        // Run "cargo test --bin day13 -- --nocapture" to see this.
        solve2(input);
    }
}
