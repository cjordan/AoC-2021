use aoc_2021::read_input;
use ndarray::prelude::*;

fn main() {
    let input = read_input(11);

    println!("The answer to part one is: {:?}", solve(&input, 100));
    println!("The answer to part two is: {:?}", solve2(&input));
}

fn parse_input(input: &str) -> Array2<u8> {
    let mut grid = Array2::zeros((input.lines().count(), input.lines().next().unwrap().len()));
    grid.iter_mut()
        .zip(input.lines().flat_map(|l| l.chars()))
        .for_each(|(g, c)| *g = c.to_digit(10).unwrap() as u8);
    grid
}

fn get_possible_steps(x: usize, y: usize, x_dim: usize, y_dim: usize) -> (Vec<i8>, Vec<i8>) {
    let mut possible_x_dirs = vec![];
    let mut possible_y_dirs = vec![];

    possible_x_dirs.push(0);
    possible_y_dirs.push(0);
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

fn find_all_flashed(mut grid: ArrayViewMut2<u8>, mut flashed: ArrayViewMut2<bool>) -> usize {
    let x_dim = grid.dim().1 - 1;
    let y_dim = grid.dim().0 - 1;

    let mut num_flashed = 0;
    let mut flashed_indices = vec![];
    grid += 1;
    loop {
        for flash in grid
            .iter()
            .zip(flashed.iter())
            .enumerate()
            .filter(|(_, (&g, &flashed))| !flashed && g > 9)
            .map(|(i, _)| i)
        {
            flashed_indices.push(flash);
        }
        if flashed_indices.is_empty() {
            break;
        }

        num_flashed += flashed_indices.len();
        for &i_flash in &flashed_indices {
            let (y, x) = (i_flash / grid.dim().0, i_flash % grid.dim().1);
            flashed[(y, x)] = true;

            let (possible_x_dirs, possible_y_dirs) = get_possible_steps(x, y, x_dim, y_dim);
            for y_step in possible_y_dirs {
                let y_new = (y as isize + y_step as isize) as usize;
                for &x_step in &possible_x_dirs {
                    let x_new = (x as isize + x_step as isize) as usize;
                    grid[(y_new, x_new)] += 1;
                }
            }
        }
        flashed_indices.clear();
    }
    grid.iter_mut().for_each(|g| {
        if *g > 9 {
            *g = 0;
        }
    });

    num_flashed
}

fn solve(input: &str, num_steps: usize) -> usize {
    let mut grid = parse_input(input);
    let mut flashed = Array2::from_elem(grid.dim(), false);
    let mut num_flashes = 0;
    for _ in 0..num_steps {
        num_flashes += find_all_flashed(grid.view_mut(), flashed.view_mut());
        flashed.fill(false);
    }
    num_flashes
}

fn solve2(input: &str) -> usize {
    let mut grid = parse_input(input);
    let mut flashed = Array2::from_elem(grid.dim(), false);
    let mut old_num_flashes = 0;
    let mut step = 0;
    loop {
        step += 1;
        let new_num_flashes = find_all_flashed(grid.view_mut(), flashed.view_mut()) as isize;
        if new_num_flashes - old_num_flashes == grid.len() as isize {
            return step;
        }
        old_num_flashes = new_num_flashes;
        flashed.fill(false);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_data() -> &'static str {
        "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
    }

    #[test]
    fn example() {
        let input = get_test_data();
        assert_eq!(solve(input, 10), 204);
        assert_eq!(solve(input, 100), 1656);
    }

    #[test]
    fn example2() {
        let input = get_test_data();
        assert_eq!(solve2(input), 195);
    }
}
