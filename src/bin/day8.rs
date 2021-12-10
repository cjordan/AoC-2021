use aoc_2021::read_input;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = parse_input(&read_input(8));
    println!("The answer to part one is: {:?}", solve(&input));
    println!("The answer to part two is: {:?}", solve2(&input));
}

fn segments() -> HashMap<&'static str, u32> {
    let mut hm = HashMap::new();
    hm.insert("abcefg", 0);
    hm.insert("cf", 1);
    hm.insert("acdeg", 2);
    hm.insert("acdfg", 3);
    hm.insert("bcdf", 4);
    hm.insert("abdfg", 5);
    hm.insert("abdefg", 6);
    hm.insert("acf", 7);
    hm.insert("abcdefg", 8);
    hm.insert("abcdfg", 9);
    hm
}

fn parse_input(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    input
        .lines()
        .map(|l| {
            let split = l.split_whitespace();
            let unique_signal_patterns = split
                .take(10)
                .map(|s| {
                    let mut chars: Vec<char> = s.chars().collect();
                    chars.sort_unstable();
                    String::from_iter(chars)
                })
                .collect();

            let split = l.split_whitespace();
            let four_digit_output = split
                .skip(11)
                .map(|s| {
                    let mut chars: Vec<char> = s.chars().collect();
                    chars.sort_unstable();
                    String::from_iter(chars)
                })
                .collect();

            (unique_signal_patterns, four_digit_output)
        })
        .collect()
}

fn solve(inputs: &[(Vec<String>, Vec<String>)]) -> usize {
    let mut num_unique = 0;

    for input in inputs {
        for output in &input.1 {
            match output.len() {
                2 => (),
                3 => (),
                4 => (),
                7 => (),
                _ => continue,
            }
            num_unique += 1
        }
    }

    num_unique
}

fn solve_inner(signals: &[String], outputs: &[String]) -> u32 {
    // The element not in common between 1 and 7 is the top segment, i.e. 'a'
    let one = signals
        .iter()
        .chain(outputs.iter())
        .find(|s| s.len() == 2)
        .unwrap();
    let four = signals
        .iter()
        .chain(outputs.iter())
        .find(|s| s.len() == 4)
        .unwrap();
    let seven = signals
        .iter()
        .chain(outputs.iter())
        .find(|s| s.len() == 3)
        .unwrap();
    let mut a = 'z';
    for char in seven.chars() {
        if !one.contains(char) {
            a = char;
            break;
        }
    }
    // The elements not in common between 1 and 4 are the top left and middle
    // segments, i.e. 'b' and 'd'.
    let mut b_or_d = ['z', 'z'];
    let mut index = 0;
    for char in four.chars() {
        if !one.contains(char) {
            b_or_d[index] = char;
            index += 1;
            continue;
        }
    }

    // 2, 3 and 5 are the only numbers with 5 segments. All numbers but 2 have
    // 'f' - use this to identify the character for 'f'.
    let five_digits: Vec<Vec<char>> = signals
        .iter()
        .chain(outputs.iter())
        .filter(|s| s.len() == 5)
        .map(|s| {
            let mut s: Vec<char> = s.chars().collect();
            s.sort_unstable();
            s
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    let not_five_digits: Vec<Vec<char>> = signals
        .iter()
        .chain(outputs.iter())
        .filter(|s| s.len() != 5)
        .map(|s| {
            let mut s: Vec<char> = s.chars().collect();
            s.sort_unstable();
            s
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    let mut in_common_between_not_five_digits = HashSet::new();
    for &char in not_five_digits.iter().flatten() {
        if not_five_digits.iter().all(|f| f.contains(&char)) {
            in_common_between_not_five_digits.insert(char);
        }
    }
    let f = *in_common_between_not_five_digits.iter().next().unwrap();

    // Now that we know what 'f' is, we can 3 and 5 to work out 'b' and 'c'.
    let five_digits_without_2: Vec<&Vec<char>> = five_digits
        .iter()
        .filter(|five| five.contains(&f))
        .collect();
    let mut b_or_c = ['z', 'z'];
    for &char in five_digits_without_2[0] {
        if !five_digits_without_2[1].contains(&char) {
            b_or_c[0] = char;
            break;
        }
    }
    for &char in five_digits_without_2[1] {
        if !five_digits_without_2[0].contains(&char) {
            b_or_c[1] = char;
        }
    }

    // Now we can isolate 'b', 'c' and 'd'.
    let mut b = 'z';
    let mut c = 'z';
    let mut d = 'z';
    for char in b_or_c {
        if b_or_d.contains(&char) {
            b = char;
        } else {
            c = char;
        }
    }
    for char in b_or_d {
        if char == b {
            continue;
        }
        d = char;
        break;
    }

    // More tedium. 2 contains 'e' but 3 and 5 don't. 3 and 5 also have 'f' but
    // not 2.
    let mut e_or_g = ['z', 'z'];
    let two = five_digits.iter().find(|five| !five.contains(&f)).unwrap();
    index = 0;
    for &char in two {
        if char == a || char == c || char == d {
            continue;
        }
        e_or_g[index] = char;
        index += 1;
    }
    let three_or_five = five_digits.iter().find(|five| five.contains(&f)).unwrap();
    let mut e = 'z';
    let mut g = 'z';
    for char in e_or_g {
        if three_or_five.contains(&char) {
            g = char;
        } else {
            e = char;
        }
    }

    // Now we have all letters and can decode the message.
    let segments = segments();

    let mut output_num = [0, 0, 0, 0];
    for (output, num) in outputs.iter().zip(output_num.iter_mut()) {
        let mut output_unscrambled = vec![];
        for char in output.chars() {
            if char == a {
                output_unscrambled.push('a');
            } else if char == b {
                output_unscrambled.push('b');
            } else if char == c {
                output_unscrambled.push('c');
            } else if char == d {
                output_unscrambled.push('d');
            } else if char == e {
                output_unscrambled.push('e');
            } else if char == f {
                output_unscrambled.push('f');
            } else if char == g {
                output_unscrambled.push('g');
            } else {
                panic!("wtf");
            }
        }
        output_unscrambled.sort_unstable();
        let sorted = output_unscrambled.into_iter().collect::<String>();
        *num = segments[sorted.as_str()];
    }

    output_num.into_iter().fold(0, |acc, x| acc * 10 + x)
}

fn solve2(inputs: &[(Vec<String>, Vec<String>)]) -> u32 {
    inputs
        .iter()
        .map(|(signals, outputs)| solve_inner(signals, outputs))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_data() -> &'static str {
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
    }

    #[test]
    fn example2() {
        let input = parse_input(get_test_data());
        assert_eq!(solve2(&input), 5353);
    }
}
