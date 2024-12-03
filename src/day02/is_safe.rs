use crate::util::absolute_difference;

pub fn is_safe(report: Vec<i32>) -> bool {
    is_monotone(report.as_slice()) && is_diff_between_1_and_3(report)
}

pub fn generic_report_test(report: &[i32], test: fn(Vec<i32>) -> bool) -> bool {
    report.windows(2).all(|w| test(w.to_vec()))
}

pub fn generic_comparison(w: &[i32], comparer: fn(&i32, &i32) -> bool) -> bool {
    comparer(&w[0], &w[1])
}

pub fn is_monotone(report: &[i32]) -> bool {
    generic_report_test(report, |v| v[0] < v[1]) || generic_report_test(report, |v| v[0] > v[1])
}

pub fn is_diff_between_1_and_3(report: Vec<i32>) -> bool {
    generic_report_test(&report, |w| {
        generic_comparison(w.as_slice(), |n1, n2| absolute_difference(n1, n2) >= 1)
            && generic_comparison(w.as_slice(), |n1, n2| absolute_difference(n1, n2) <= 3)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INCREASING_VALUES_REPORT: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    const DECREASING_VALUES_REPORT: [i32; 10] = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    const MIXED_VALUES_REPORT: [i32; 10] = [4, 6, 5, 7, 3, 8, 2, 9, 9, 10];

    #[test]
    fn can_determine_if_either_all_increasing_or_all_decreasing() {
        assert!(is_monotone(INCREASING_VALUES_REPORT.as_ref()));
        assert!(is_monotone(DECREASING_VALUES_REPORT.as_ref()));
        assert!(!is_monotone(MIXED_VALUES_REPORT.as_ref()));
    }

    #[test]
    fn can_determine_if_diff_between_sorted_values_is_between_1_and_3() {
        assert!(is_diff_between_1_and_3(INCREASING_VALUES_REPORT.to_vec()));
        assert!(is_diff_between_1_and_3(DECREASING_VALUES_REPORT.to_vec()));
        assert!(!is_diff_between_1_and_3(MIXED_VALUES_REPORT.to_vec()));
    }

    #[test]
    fn can_determine_if_report_is_safe() {
        assert!(is_safe(INCREASING_VALUES_REPORT.to_vec()));
        assert!(is_safe(DECREASING_VALUES_REPORT.to_vec()));
        assert!(!is_safe(MIXED_VALUES_REPORT.to_vec()));
    }

    #[test]
    fn can_tell_not_safe_when_not_monotone() {
        let report = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1];
        assert!(!is_safe(report));
    }

    #[test]
    fn can_tell_not_safe_when_min_diff_between_values_is_lt_1() {
        let report = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 9];
        assert!(!is_safe(report));
    }

    #[test]
    fn can_tell_not_safe_when_max_diff_between_values_is_gt_3() {
        let report = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 17];
        assert!(!is_safe(report));
    }

    #[test]
    fn recognizes_that_increasingvaluesreport_and_decreasingvaluesreport_are_safe() {
        assert!(is_safe(INCREASING_VALUES_REPORT.to_vec()));
        assert!(is_safe(DECREASING_VALUES_REPORT.to_vec()));
    }

    #[test]
    fn test_is_safe_case_1() {
        let rpt = vec![7, 6, 4, 2, 1];
        assert!(is_safe(rpt));
    }

    #[test]
    fn test_is_safe_case_2() {
        let rpt = vec![1, 2, 7, 8, 9];
        assert!(!is_safe(rpt));
    }

    #[test]
    fn test_is_safe_case_3() {
        let rpt = vec![9, 7, 6, 2, 1];
        assert!(!is_safe(rpt));
    }

    #[test]
    fn test_is_safe_case_4() {
        let rpt = vec![8, 6, 4, 4, 1];
        assert!(!is_safe(rpt));
    }

    #[test]
    fn test_is_safe_case_5() {
        let rpt = vec![1, 3, 6, 7, 9];
        assert!(is_safe(rpt));
    }
}
