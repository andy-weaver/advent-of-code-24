use aoc::day02::{count_safe_reports, INPUT_FILENAME};
use aoc::util::{Parser, Reader};

fn part1(parser: Parser) -> i32 {
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
    let reader = Reader::new(INPUT_FILENAME);
    let raw = reader.read();
    let parser = Parser::new(&raw);

    let part1_result = part1(parser.clone());
    println!("Part 1: {}", part1_result);
}


