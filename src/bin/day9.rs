use aoc_2021::read_input;
use ndarray::prelude::*;

fn main() {
    let input = read_input(9);
    println!("The answer to part one is: {:?}", solve(&input));
    println!("The answer to part two is: {:?}", solve2(&input));
}

fn solve(input: &str) -> u32 {
    let mut grid = Array2::zeros((input.lines().count(), input.lines().next().unwrap().len()));
    grid.iter_mut()
        .zip(input.lines().flat_map(|l| l.chars()))
        .for_each(|(g, c)| *g = c.to_digit(10).unwrap());
    let x_dim = grid.dim().1 - 1;
    let y_dim = grid.dim().0 - 1;

    let mut minima = vec![];
    let mut possible_x_dirs = vec![];
    let mut possible_y_dirs = vec![];
    for (y, grid_row) in grid.outer_iter().enumerate() {
        'outer: for (x, &height) in grid_row.iter().enumerate() {
            possible_x_dirs.clear();
            possible_y_dirs.clear();

            if x == 0 {
                possible_x_dirs.push(1);
            } else if x == x_dim {
                possible_x_dirs.push(-1);
            } else {
                possible_x_dirs.push(1);
                possible_x_dirs.push(-1);
            }
            if y == 0 {
                possible_y_dirs.push(1);
            } else if y == y_dim {
                possible_y_dirs.push(-1);
            } else {
                possible_y_dirs.push(1);
                possible_y_dirs.push(-1);
            }

            for &x_step in possible_x_dirs.iter() {
                let x_new = x as i32 + x_step;
                let height_new = grid[(y, x_new as usize)];
                if height_new < height {
                    continue 'outer;
                }
            }
            for &y_step in possible_y_dirs.iter() {
                let y_new = y as i32 + y_step;
                let height_new = grid[(y_new as usize, x)];
                if height_new <= height {
                    continue 'outer;
                }
            }
            minima.push((x, y));
        }
    }

    minima.into_iter().map(|(x, y)| grid[(y, x)] + 1).sum()
}

fn get_possible_steps(x: usize, y: usize, x_dim: usize, y_dim: usize) -> (Vec<i8>, Vec<i8>) {
    let mut possible_x_dirs = vec![];
    let mut possible_y_dirs = vec![];

    if x == 0 {
        possible_x_dirs.push(1);
    } else if x == x_dim {
        possible_x_dirs.push(-1);
    } else {
        possible_x_dirs.push(1);
        possible_x_dirs.push(-1);
    }
    if y == 0 {
        possible_y_dirs.push(1);
    } else if y == y_dim {
        possible_y_dirs.push(-1);
    } else {
        possible_y_dirs.push(1);
        possible_y_dirs.push(-1);
    }

    (possible_x_dirs, possible_y_dirs)
}

fn walk_basin(x: usize, y: usize, grid: ArrayView2<u8>, mut walked: ArrayViewMut2<bool>) -> u32 {
    let x_dim = grid.dim().1 - 1;
    let y_dim = grid.dim().0 - 1;

    let mut basin_size = 0;
    let mut to_be_investiaged = vec![];

    let (possible_x_dirs, possible_y_dirs) = get_possible_steps(x, y, x_dim, y_dim);
    for &x_step in &possible_x_dirs {
        let x_new = (x as i32 + x_step as i32) as usize;
        if walked[(y, x_new)] {
            continue;
        } else {
            basin_size += 1;
            walked[(y, x_new)] = true;
            to_be_investiaged.push((x_new, y));
        }
    }
    for &y_step in &possible_y_dirs {
        let y_new = (y as i32 + y_step as i32) as usize;
        if walked[(y_new, x)] {
            continue;
        } else {
            basin_size += 1;
            walked[(y_new, x)] = true;
            to_be_investiaged.push((x, y_new));
        }
    }

    let mut to_be_investigated2 = vec![];
    while !to_be_investiaged.is_empty() {
        for (x, y) in to_be_investiaged.drain(..) {
            let (possible_x_dirs, possible_y_dirs) = get_possible_steps(x, y, x_dim, y_dim);
            for &x_step in &possible_x_dirs {
                let x_new = (x as i32 + x_step as i32) as usize;
                if walked[(y, x_new)] {
                    continue;
                } else {
                    basin_size += 1;
                    walked[(y, x_new)] = true;
                    to_be_investigated2.push((x_new, y));
                }
            }
            for &y_step in &possible_y_dirs {
                let y_new = (y as i32 + y_step as i32) as usize;
                if walked[(y_new, x)] {
                    continue;
                } else {
                    basin_size += 1;
                    walked[(y_new, x)] = true;
                    to_be_investigated2.push((x, y_new));
                }
            }
        }
        to_be_investiaged.append(&mut to_be_investigated2);
    }

    basin_size
}

fn solve2(input: &str) -> u32 {
    let mut grid: Array2<u8> =
        Array2::zeros((input.lines().count(), input.lines().next().unwrap().len()));
    grid.iter_mut()
        .zip(input.lines().flat_map(|l| l.chars()))
        .for_each(|(g, c)| *g = c.to_digit(10).unwrap() as u8);
    let mut walked: Array2<bool> = Array2::from_elem(grid.dim(), false);
    walked.iter_mut().zip(grid.iter()).for_each(|(w, &g)| {
        if g == 9 {
            *w = true;
        }
    });

    let mut basins = vec![];
    for (y, grid_row) in grid.outer_iter().enumerate() {
        for (x, _) in grid_row.iter().enumerate() {
            if walked[(y, x)] {
                continue;
            } else {
                basins.push(walk_basin(x, y, grid.view(), walked.view_mut()));
            }
        }
    }

    basins.sort_unstable_by(|a, b| b.cmp(a));
    basins.into_iter().take(3).product()
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_data() -> &'static str {
        r#"2199943210
3987894921
9856789892
8767896789
9899965678"#
    }

    #[test]
    fn example() {
        let input = get_test_data();
        assert_eq!(solve(input), 15);
    }

    #[test]
    fn example2() {
        let input = get_test_data();
        assert_eq!(solve2(input), 1134);
    }
}
