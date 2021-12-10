use aoc_2021::read_input;

fn main() {
    let input = read_input(6);

    println!("The answer to part one is: {:?}", solve(&input, 80));
    println!("The answer to part two is: {:?}", solve(&input, 256));
}

// fn solve_naive(input: &str, num_days: usize) -> usize {
//     let mut fishs: Vec<u8> = input
//         .split(',')
//         .map(|n| n.trim_end().parse().unwrap())
//         .collect();
//     let mut new_fish = vec![];
//     for _ in 1..=num_days {
//         for fish in fishs.iter_mut() {
//             *fish = if *fish == 0 {
//                 new_fish.push(8);
//                 6
//             } else {
//                 *fish - 1
//             };
//         }
//         fishs.append(&mut new_fish);
//     }

//     fishs.len()
// }

fn solve(input: &str, num_days: usize) -> usize {
    let mut fishs = vec![0; 9];
    input.split(',').for_each(|n| {
        let n: usize = n.trim_end().parse().unwrap();
        fishs[n] += 1;
    });

    let mut new_fish_count = 0;
    let mut reset_fish_count = 0;
    let mut new_fishs = vec![0; 9];
    for _ in 1..=num_days {
        for (i, num_fish) in fishs.iter().enumerate() {
            match i {
                0 => {
                    new_fish_count += num_fish;
                    reset_fish_count += num_fish
                }
                _ => new_fishs[i - 1] += num_fish,
            }
        }
        new_fishs[8] += new_fish_count;
        new_fish_count = 0;
        new_fishs[6] += reset_fish_count;
        reset_fish_count = 0;
        fishs = new_fishs.clone();
        new_fishs.iter_mut().for_each(|i| *i = 0);
    }

    fishs.into_iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_data() -> &'static str {
        "3,4,3,1,2"
    }

    #[test]
    fn example() {
        let input = get_test_data();
        assert_eq!(solve(input, 18), 26);
        assert_eq!(solve(input, 80), 5934);
    }
}
