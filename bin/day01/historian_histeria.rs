use aoc::day01::{data_to_vec, total_similarity_score, SortedAbsDiffSummer};
use aoc::prelude::*;

const INPUT_FILE_PATH: &str = "/home/andy/rust/advent-of-code-24/input/01-historian-histeria.input";

fn get_sorted_abs_diff(vec1: Vec<i32>, vec2: Vec<i32>) -> i32 {
    let mut summer = SortedAbsDiffSummer::new(vec1.len());
    summer.load(1, vec1);
    summer.load(2, vec2);
    summer.sum_sorted_absolute_differences()
}

fn main() {
    let reader = Reader::new(INPUT_FILE_PATH);
    let parser = Parser::new(&reader.read());
    let raw_data = parser.parse("   ");
    let (vec1, vec2) = data_to_vec(raw_data);
    let total_abs_diff = get_sorted_abs_diff(vec1.clone(), vec2.clone());

    println!("Total difference: {}", total_abs_diff);

    let sum_similarity_score = total_similarity_score(vec2, vec1);

    println!("Total similarity score: {}", sum_similarity_score);
}
