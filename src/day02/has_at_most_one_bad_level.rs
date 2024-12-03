use crate::day02::is_safe;

pub fn has_at_most_one_bad_level(report: Vec<i32>) -> bool {
    let is_increasing: Option<bool> = None;
    let n_direction_violations = &report.windows(2).fold(0, |acc, w| {
        let diff_ge_0 = w[1] - w[0] >= 0;
        let first_is_larger = !diff_ge_0;
        let second_is_larger = diff_ge_0;
        match is_increasing {
            None => {
                assign_is_increasing_on_first_iteration(diff_ge_0, is_increasing);
                acc
            }
            Some(is_monotone_increasing) => {
                let is_monotone_decreasing = !is_monotone_increasing;

                let mut n_violations = 0;
                n_violations += (first_is_larger & is_monotone_increasing) as i32;
                n_violations += (second_is_larger & is_monotone_decreasing) as i32;

                increment_accumulator(acc + n_violations) // This will be 0 or 1 if the report is valid
            }
        }
    });

    *n_direction_violations <= 1
}

fn increment_accumulator(acc: i32) -> i32 {
    acc + 1
}

fn assign_is_increasing_on_first_iteration(
    diff_ge_0: bool,
    mut is_increasing: Option<bool>,
) -> Option<bool> {
    if diff_ge_0 {
        is_increasing = Some(true);
    } else if !diff_ge_0 {
        is_increasing = Some(false);
    }

    is_increasing
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_at_most_one_bad_level_case_1() {
        let rpt = vec![7, 6, 4, 2, 1];
        assert!(has_at_most_one_bad_level(rpt));
    }

    #[test]
    fn test_has_at_most_one_bad_level_case_2() {
        let rpt = vec![1, 2, 7, 8, 9];
        assert!(!has_at_most_one_bad_level(rpt));
    }

    #[test]
    fn test_has_at_most_one_bad_level_case_3() {
        let rpt = vec![9, 7, 6, 2, 1];
        assert!(!has_at_most_one_bad_level(rpt));
    }

    #[test]
    fn test_has_at_most_one_bad_level_case_4() {
        let rpt = vec![8, 6, 4, 4, 1];
        assert!(has_at_most_one_bad_level(rpt));
    }

    #[test]
    fn test_has_at_most_one_bad_level_case_5() {
        let rpt = vec![1, 3, 6, 7, 9];
        assert!(has_at_most_one_bad_level(rpt));
    }
}
