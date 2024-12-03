pub fn data_to_vec(raw_data: Vec<Vec<&str>>) -> (Vec<i32>, Vec<i32>) {
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();

    raw_data.iter().for_each(|v| {
        vec1.push(parse_string_to_int(v[0]));
        vec2.push(parse_string_to_int(v[1]));
    });

    (vec1, vec2)
}

fn parse_string_to_int(n: &str) -> i32 {
    n.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn can_parse_string_to_int() {
        let result = parse_string_to_int("5");
        assert_eq!(result, 5);
    }

    #[test]
    fn can_parse_string_to_int_with_negative_number() {
        let result = parse_string_to_int("-5");
        assert_eq!(result, -5);
    }

    #[test]
    fn can_parse_string_to_int_with_zero() {
        let result = parse_string_to_int("0");
        assert_eq!(result, 0);
    }

    #[test]
    fn can_parse_string_to_int_with_large_number() {
        let result = parse_string_to_int("1234567890");
        assert_eq!(result, 1234567890);
    }

    #[test]
    fn can_parse_string_to_int_with_large_negative_number() {
        let result = parse_string_to_int("-1234567890");
        assert_eq!(result, -1234567890);
    }

    fn make_raw_data<'a>() -> Vec<Vec<&'a str>> {
        vec![vec![&"1", &"2"], vec![&"3", &"4"], vec![&"5", &"6"]]
    }

    #[test]
    fn can_convert_raw_data_to_vectors() {
        let raw_data = make_raw_data();
        let (vec1, vec2) = data_to_vec(raw_data);
        assert_eq!(vec1, vec![1, 3, 5]);
        assert_eq!(vec2, vec![2, 4, 6]);
    }
}
