use aoc_2021::read_input;
use std::ops::Add;

#[derive(Clone, Copy)]
enum Dir {
    Up(i32),
    Down(i32),
    Forward(i32),
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Pos {
    /// Depth
    d: i32,
    /// Horizontal
    h: i32,
}

impl Add<Dir> for Pos {
    type Output = Pos;

    fn add(self, rhs: Dir) -> Self::Output {
        match (self, rhs) {
            (Pos { d, h }, Dir::Up(n)) => Pos { d: d - n, h },
            (Pos { d, h }, Dir::Down(n)) => Pos { d: d + n, h },
            (Pos { d, h }, Dir::Forward(n)) => Pos { d, h: h + n },
        }
    }
}

fn solve(moves: &[Dir]) -> (Pos, i32) {
    let pos = moves.iter().fold(Pos { d: 0, h: 0 }, |acc, &dir| acc + dir);
    (pos, pos.d * pos.h)
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Pos2 {
    d: i32,
    h: i32,
    aim: i32,
}

impl Add<Dir> for Pos2 {
    type Output = Pos2;

    fn add(self, rhs: Dir) -> Self::Output {
        match (self, rhs) {
            (Pos2 { d, h, aim }, Dir::Up(n)) => Pos2 { d, h, aim: aim - n },
            (Pos2 { d, h, aim }, Dir::Down(n)) => Pos2 { d, h, aim: aim + n },
            (Pos2 { d, h, aim }, Dir::Forward(n)) => Pos2 {
                d: d + n * aim,
                h: h + n,
                aim,
            },
        }
    }
}

fn solve2(moves: &[Dir]) -> (Pos2, i32) {
    let pos = moves
        .iter()
        .fold(Pos2 { d: 0, h: 0, aim: 0 }, |acc, &dir| acc + dir);
    (pos, pos.d * pos.h)
}

fn main() {
    let input = read_input(2);
    let moves: Vec<Dir> = input
        .lines()
        .map(|l| {
            let amount: i32 = l.split_whitespace().last().unwrap().parse().unwrap();
            if l.contains("up") {
                Dir::Up(amount)
            } else if l.contains("down") {
                Dir::Down(amount)
            } else if l.contains("forward") {
                Dir::Forward(amount)
            } else {
                unreachable!()
            }
        })
        .collect();
    println!("The answer to part one is: {:?}", solve(&moves));
    println!("The answer to part two is: {:?}", solve2(&moves));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let moves = [
            Dir::Forward(5),
            Dir::Down(5),
            Dir::Forward(8),
            Dir::Up(3),
            Dir::Down(8),
            Dir::Forward(2),
        ];
        assert_eq!(solve(&moves), (Pos { d: 10, h: 15 }, 150));
    }

    #[test]
    fn example2() {
        let moves = [
            Dir::Forward(5),
            Dir::Down(5),
            Dir::Forward(8),
            Dir::Up(3),
            Dir::Down(8),
            Dir::Forward(2),
        ];
        assert_eq!(
            solve2(&moves),
            (
                Pos2 {
                    d: 60,
                    h: 15,
                    aim: 10,
                },
                900
            )
        );
    }
}
