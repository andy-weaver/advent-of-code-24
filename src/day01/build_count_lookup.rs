use std::collections::HashMap;

type CountLookup = HashMap<i32, i32>;

pub fn build_count_lookup(vec: Vec<i32>) -> CountLookup {
    let mut count_lookup: CountLookup = HashMap::new();

    vec.iter().for_each(|n| {
        dereference_count_and_increment_in_place(&mut count_lookup, n);
    });

    count_lookup
}

fn dereference_count_and_increment_in_place(lkp: &mut CountLookup, n: &i32) {
    let count = lkp.entry(n.to_owned()).or_insert(0);
    *count += 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_build_count_lookup() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let count_lookup = build_count_lookup(vec);

        assert_eq!(count_lookup.get(&1), Some(&1));
        assert_eq!(count_lookup.get(&2), Some(&1));
        assert_eq!(count_lookup.get(&3), Some(&1));
        assert_eq!(count_lookup.get(&4), Some(&1));
        assert_eq!(count_lookup.get(&5), Some(&1));
        assert_eq!(count_lookup.get(&6), Some(&1));
        assert_eq!(count_lookup.get(&7), Some(&1));
        assert_eq!(count_lookup.get(&8), Some(&1));
        assert_eq!(count_lookup.get(&9), Some(&1));
        assert_eq!(count_lookup.get(&10), Some(&1));
    }

    #[test]
    fn can_build_count_lookup_with_duplicates() {
        let vec = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        let count_lookup = build_count_lookup(vec);

        assert_eq!(count_lookup.get(&1), Some(&10));
    }

    #[test]
    fn can_build_count_lookup_with_negative_numbers() {
        let vec = vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -9];
        let count_lookup = build_count_lookup(vec);

        assert_eq!(count_lookup.get(&-1), Some(&1));
        assert_eq!(count_lookup.get(&-2), Some(&1));
        assert_eq!(count_lookup.get(&-3), Some(&1));
        assert_eq!(count_lookup.get(&-4), Some(&1));
        assert_eq!(count_lookup.get(&-5), Some(&1));
        assert_eq!(count_lookup.get(&-6), Some(&1));
        assert_eq!(count_lookup.get(&-7), Some(&1));
        assert_eq!(count_lookup.get(&-8), Some(&1));
        assert_eq!(count_lookup.get(&-9), Some(&2));
    }

    #[test]
    fn can_build_count_lookup_with_mixed_numbers() {
        let vec = vec![-1, 2, -3, 4, -5, 6, -7, 8, -9, -9];
        let count_lookup = build_count_lookup(vec);

        assert_eq!(count_lookup.get(&-1), Some(&1));
        assert_eq!(count_lookup.get(&2), Some(&1));
        assert_eq!(count_lookup.get(&-3), Some(&1));
        assert_eq!(count_lookup.get(&4), Some(&1));
        assert_eq!(count_lookup.get(&-5), Some(&1));
        assert_eq!(count_lookup.get(&6), Some(&1));
        assert_eq!(count_lookup.get(&-7), Some(&1));
        assert_eq!(count_lookup.get(&8), Some(&1));
        assert_eq!(count_lookup.get(&-9), Some(&2));
    }
}
