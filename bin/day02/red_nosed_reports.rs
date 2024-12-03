use aoc::day02::{count_safe_reports, INPUT_FILENAME};
use aoc::util::{Parser, Reader};

fn part1() -> i32 {
    let reader = Reader::new(INPUT_FILENAME);
    let raw = reader.read();
    let parser = Parser::new(&raw);
    let reports = parser
        .split_lines()
        .iter()
        .map(|line| {
            Parser::new(line)
                .split_at(vec![" ", "\t", "  ", "   "])
                .iter()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    count_safe_reports(reports)
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]

