use std::collections::HashMap;

use crate::day01::build_count_lookup;

pub fn total_similarity_score(vec1: Vec<i32>, vec2: Vec<i32>) -> i32 {
    let count_lookup = build_count_lookup(vec2);
    let sum_similarity_score = vec1
        .iter()
        .map(|&v1| map_value_to_score(&count_lookup, v1))
        .sum::<i32>();
    sum_similarity_score
}

fn map_value_to_score(count_lookup: &HashMap<i32, i32>, value_to_lookup: i32) -> i32 {
    score(
        value_to_lookup,
        count_lookup.get(&value_to_lookup).unwrap_or(&0).to_owned(),
    )
}

fn score(value: i32, count: i32) -> i32 {
    value * count
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> (Vec<i32>, Vec<i32>) {
        (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3])
    }

    #[test]
    fn get_3s_score_of_9() {
        let (_vec1, vec2) = get_test_data();
        let count_lookup = build_count_lookup(vec2);
        let result = map_value_to_score(&count_lookup, 3);
        assert_eq!(result, 9);
    }

    #[test]
    fn get_4s_score_of_4() {
        let (_vec1, vec2) = get_test_data();
        let count_lookup = build_count_lookup(vec2);
        let result = map_value_to_score(&count_lookup, 4);
        assert_eq!(result, 4);
    }

    #[test]
    fn get_2s_score_of_0() {
        let (_vec1, vec2) = get_test_data();
        let count_lookup = build_count_lookup(vec2);
        let result = map_value_to_score(&count_lookup, 2);
        assert_eq!(result, 0);
    }

    #[test]
    fn get_1s_score_of_0() {
        let (_vec1, vec2) = get_test_data();
        let count_lookup = build_count_lookup(vec2);
        let result = map_value_to_score(&count_lookup, 1);
        assert_eq!(result, 0);
    }

    #[test]
    fn get_total_similarity_score() {
        let (vec1, vec2) = get_test_data();
        let actual = total_similarity_score(vec1, vec2);
        let expected = 31;
        assert_eq!(actual, expected);
    }
}
