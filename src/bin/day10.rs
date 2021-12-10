use aoc_2021::read_input;

fn main() {
    let input = read_input(10);

    println!("The answer to part one is: {:?}", solve(&input));
    println!("The answer to part two is: {:?}", solve2(&input));
}

enum Line {
    Complete,
    Incomplete(Vec<char>),
    Corrupt(char),
}

/// The best function name.
fn line_to_line(line: &str, opens: &mut Vec<char>) -> Line {
    for char in line.chars() {
        match char {
            '[' | '(' | '{' | '<' => opens.push(char),
            ']' | ')' | '}' | '>' => match (opens.pop(), char) {
                (Some('['), ']') => (),
                (Some('('), ')') => (),
                (Some('{'), '}') => (),
                (Some('<'), '>') => (),
                (_, char) => return Line::Corrupt(char),
            },
            _ => panic!("Unexpected input!"),
        }
    }
    if opens.is_empty() {
        Line::Complete
    } else {
        Line::Incomplete(opens.clone())
    }
}

fn parse_input(input: &str) -> Vec<Line> {
    let mut opens = vec![];
    input
        .lines()
        .map(|l| {
            opens.clear();
            line_to_line(l, &mut opens)
        })
        .collect()
}

fn solve(input: &str) -> usize {
    parse_input(input)
        .into_iter()
        .filter(|l| matches!(l, Line::Corrupt(_)))
        .map(|l| match l {
            Line::Corrupt(')') => 3,
            Line::Corrupt(']') => 57,
            Line::Corrupt('}') => 1197,
            Line::Corrupt('>') => 25137,
            _ => panic!("wtf"),
        })
        .sum()
}

fn solve2(input: &str) -> usize {
    let mut scores: Vec<usize> = parse_input(input)
        .into_iter()
        .filter(|l| matches!(l, Line::Incomplete(_)))
        .map(|l| match l {
            Line::Incomplete(mut opens) => {
                opens.reverse();
                let mut score = 0;
                for char in opens {
                    score *= 5;
                    score += match char {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => unreachable!(),
                    };
                }
                score
            }
            _ => unreachable!(),
        })
        .collect();

    scores.sort_unstable();
    dbg!(&scores);
    scores[scores.len() / 2]
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_data() -> &'static str {
        "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
    }

    #[test]
    fn example() {
        let input = get_test_data();
        assert_eq!(solve(input), 26397);
    }

    #[test]
    fn example2() {
        let input = get_test_data();
        assert_eq!(solve2(input), 288957);
    }
}
