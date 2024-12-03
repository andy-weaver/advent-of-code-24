use crate::day02::is_safe;

pub fn count_safe_reports(reports: Vec<Vec<i32>>) -> i32 {
    let mut count = 0;

    reports.iter().for_each(|line| {
        let rpt = line.clone();
        if is_safe(rpt) {
            count += 1;
        }
    });

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_reports() -> Vec<Vec<i32>> {
        vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ]
    }

    #[test]
    fn can_count_safe_reports() {
        let reports = get_reports();
        let result = count_safe_reports(reports);
        assert_eq!(result, 2);
    }
}
