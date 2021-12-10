fn get_nums() -> &'static str {
    r#"011110011110
101101001111
000000010101
100111001010
"#
}

fn solve(input: &str) -> i32 {
    // There are 12 bits.
    let mut bit_strings = vec![String::new(); 12];
    for line in input.lines() {
        for (i_bit, bit) in line.chars().enumerate() {
            bit_strings[i_bit].push(bit);
        }
    }

    let mut bit_counts = vec![0; 12];
    for (i, bit_string) in bit_strings.into_iter().enumerate() {
        bit_counts[i] = u16::from_str_radix(&bit_string, 2).unwrap().count_ones();
    }
    dbg!(bit_counts);
    0
}

fn main() {
    let input = get_nums();
    println!("Answer to part one is: {}", solve(input));
}

// use aoc_2021::read_input;

// fn main() {
//     let input = read_input(3);
//     println!("The answer to part one is: {:?}", solve(&input));
//     println!("The answer to part two is: {:?}", solve2(&input));
// }

// fn binary_to_int(b: &str) -> u16 {
//     u16::from_str_radix(b, 2).unwrap()
// }

// fn bit_is_set(value: usize, bit_index: usize) -> bool {
//     (value >> bit_index) & 1 == 1
// }

// fn get_occ(hist: &[usize], num_bits: usize) -> Vec<usize> {
//     let mut occ = vec![0; num_bits];
//     for (v, &h) in hist.iter().enumerate() {
//         for bit_index in 0..num_bits {
//             if bit_is_set(v, bit_index) {
//                 occ[(num_bits - 1) - bit_index] += h as usize;
//             }
//         }
//     }
//     occ
// }

// fn solve(nums: &str) -> ((u32, u32), u32) {
//     let nums: Vec<u16> = nums.lines().map(binary_to_int).collect();
//     // Assume all `nums` are the same length.
//     let num_bits = format!("{:b}", nums.iter().max().unwrap()).len();
//     let max_num = 2_usize.pow(num_bits as u32);

//     let mut hist = vec![0; max_num];
//     for &num in &nums {
//         hist[num as usize] += 1;
//     }
//     let occ = get_occ(&hist, num_bits);

//     let mut gamma_digits = vec![0_u32; num_bits];
//     for (&v, g) in occ.iter().zip(gamma_digits.iter_mut()) {
//         *g = if v > nums.len() / 2 { 1 } else { 0 };
//     }
//     let gamma = gamma_digits
//         .iter()
//         .rev()
//         .enumerate()
//         .fold(0, |acc, (i, &d)| acc + d * 2_u32.pow(i as u32));
//     let epsilon = gamma_digits
//         .iter()
//         .rev()
//         .enumerate()
//         .fold(0, |acc, (i, &d)| {
//             acc + if d == 1 { 0 } else { 1 } * 2_u32.pow(i as u32)
//         });
//     ((gamma, epsilon), gamma * epsilon)
// }

// fn solve2(nums_str: &str) -> ((u32, u32), u32) {
//     let nums: Vec<&str> = nums_str.lines().collect();
//     // Assume all `nums` are the same length.
//     let num_bits = nums_str.split_whitespace().next().unwrap().len();

//     // Oxygen generator rating.
//     let mut oxy_nums = nums.clone();
//     for i_bit in 0..num_bits {
//         let set_count = oxy_nums
//             .iter()
//             .filter(|&&n| n.get(i_bit..i_bit + 1) == Some("1"))
//             .count();
//         let one_is_most_common = set_count * 2 >= oxy_nums.len();
//         oxy_nums = oxy_nums
//             .into_iter()
//             .filter(|&n| {
//                 let is_set = n.get(i_bit..i_bit + 1) == Some("1");
//                 matches!((one_is_most_common, is_set), (true, true) | (false, false))
//             })
//             .collect();
//         if oxy_nums.len() == 1 {
//             break;
//         }
//     }
//     let oxy = binary_to_int(oxy_nums[0]) as u32;

//     // CO2 scrubber rating.
//     let mut co2_nums = nums;
//     for i_bit in 0..num_bits {
//         let set_count = co2_nums
//             .iter()
//             .filter(|&&n| n.get(i_bit..i_bit + 1) == Some("1"))
//             .count();
//         let one_is_most_common = set_count * 2 >= co2_nums.len();
//         co2_nums = co2_nums
//             .into_iter()
//             .filter(|&n| {
//                 let is_set = n.get(i_bit..i_bit + 1) == Some("1");
//                 matches!((one_is_most_common, is_set), (true, false) | (false, true))
//             })
//             .collect();
//         if co2_nums.len() == 1 {
//             break;
//         }
//     }
//     let co2 = binary_to_int(co2_nums[0]) as u32;

//     ((oxy, co2), oxy * co2)
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     fn get_test_data() -> &'static str {
//         "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010"
//     }

//     #[test]
//     fn example() {
//         let nums = get_test_data();
//         assert_eq!(solve(nums), ((22, 9), 198));
//     }

//     #[test]
//     fn example2() {
//         let nums = get_test_data();
//         assert_eq!(solve2(nums), ((23, 10), 230));
//     }
// }
