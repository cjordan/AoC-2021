use std::fs::File;
use std::io::Read;

pub fn read_input(day: u8) -> String {
    let mut input = String::new();
    File::open(format!("inputs/day{}.txt", day))
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    input
}
