use crate::util::absolute_difference;
pub struct SortedAbsDiffSummer {
    vec1: Vec<i32>,
    vec2: Vec<i32>,
}

impl SortedAbsDiffSummer {
    pub fn new(len: usize) -> SortedAbsDiffSummer {
        SortedAbsDiffSummer {
            vec1: Vec::with_capacity(len),
            vec2: Vec::with_capacity(len),
        }
    }

    fn sort_two_vectors_in_place(&mut self) {
        self.vec1.sort();
        self.vec2.sort();
    }

    fn absolute_difference(&self) -> Vec<i32> {
        self.vec1
            .iter()
            .enumerate()
            .map(|(i, n1)| {
                let n2 = &self.vec2[i];

                absolute_difference(n1, n2)
            })
            .collect()
    }

    fn sum_absolute_differences(&self) -> i32 {
        self.absolute_difference().iter().sum()
    }

    pub fn sum_sorted_absolute_differences(&mut self) -> i32 {
        self.sort_two_vectors_in_place();
        self.sum_absolute_differences()
    }

    pub fn load(&mut self, vec_number: i32, load_vec_with: Vec<i32>) {
        match vec_number {
            1 => load_vec_with.iter().for_each(|n| self.vec1.push(*n)),
            2 => load_vec_with.iter().for_each(|n| self.vec2.push(*n)),
            _ => panic!("Invalid vector number"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_make_data_struct() {
        let data = SortedAbsDiffSummer::new(5);
        assert_eq!(data.vec1.len(), 0);
        assert_eq!(data.vec2.len(), 0);
    }

    #[test]
    fn constructor_is_identical_to_new_method() {
        let data1 = SortedAbsDiffSummer::new(5);
        let vec1: Vec<i32> = Vec::with_capacity(5);
        let vec2: Vec<i32> = Vec::with_capacity(5);
        assert_eq!(data1.vec1.len(), vec1.len());
        assert_eq!(data1.vec2.len(), vec2.len());

        let data2 = SortedAbsDiffSummer { vec1, vec2 };
        assert_eq!(data1.vec1.len(), data2.vec1.len());
        assert_eq!(data1.vec2.len(), data2.vec2.len());
    }

    #[test]
    fn can_calculate_absolute_difference() {
        let mut data = SortedAbsDiffSummer::new(5);
        [1, 2, 3, 4, 5].iter().for_each(|n| data.vec1.push(*n));
        [5, 4, 3, 2, 1].iter().for_each(|n| data.vec2.push(*n));

        let result = data.absolute_difference();
        assert_eq!(result, vec![4, 2, 0, 2, 4]);
    }

    #[test]
    fn can_calculate_absolute_difference_with_empty_vectors() {
        let data = SortedAbsDiffSummer::new(0);
        let result = data.absolute_difference();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn can_calculate_sum_of_absolute_differences() {
        let mut data = SortedAbsDiffSummer::new(5);
        [1, 2, 3, 4, 5].iter().for_each(|n| data.vec1.push(*n));
        [5, 4, 3, 2, 1].iter().for_each(|n| data.vec2.push(*n));

        let result = data.sum_absolute_differences();
        assert_eq!(result, 12);
    }

    #[test]
    fn can_sort_two_vectors_in_place() {
        let mut data = SortedAbsDiffSummer::new(5);
        [1, 2, 3, 4, 5].iter().for_each(|n| data.vec1.push(*n));
        [5, 4, 3, 2, 1].iter().for_each(|n| data.vec2.push(*n));

        data.sort_two_vectors_in_place();

        assert_eq!(data.vec1, vec![1, 2, 3, 4, 5]);
        assert_eq!(data.vec2, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn can_load_vecs() {
        let mut data = SortedAbsDiffSummer::new(5);
        data.load(1, [1, 2, 3, 4, 5].to_vec());
        data.load(2, [5, 4, 3, 2, 1].to_vec());

        assert_eq!(data.vec1, vec![1, 2, 3, 4, 5]);
        assert_eq!(data.vec2, vec![5, 4, 3, 2, 1]);
    }
}
