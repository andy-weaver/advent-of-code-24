use aoc::prelude::*;

const FILENAME: &str = "/home/andy/rust/advent-of-code-24/input/01-historian-histeria.input";

fn main() {
    let reader = Reader::new(FILENAME);
    let parser = Parser::new(&reader.read());
    let raw_data = parser.parse("   ");
    let (mut vec1, mut vec2) = initialize_two_vecs(raw_data.len());

    raw_data.iter().for_each(|v| {
        v.iter().enumerate().for_each(|(i, n)| {
            push_to_vectors(i, &mut vec1, n, &mut vec2);
        });
    });

    sort_two_vectors_in_place(&mut vec1, &mut vec2);

    let total_abs_diff = sum_absolute_differences(vec1, vec2);

    println!("Total difference: {}", total_abs_diff);


}

fn initialize_two_vecs(file_length: usize) -> (Vec<i32>, Vec<i32>) {
    let vec1: Vec<i32> = Vec::with_capacity(file_length);
    let vec2: Vec<i32> = Vec::with_capacity(file_length);
    (vec1, vec2)
}

fn sort_two_vectors_in_place(vec1: &mut [i32], vec2: &mut [i32]) {
    vec1.sort();
    vec2.sort();
}

fn sum_absolute_differences(vec1: Vec<i32>, vec2: Vec<i32>) -> i32 {
    let result = vec1
        .iter()
        .enumerate()
        .map(|(i, n1)| absolute_difference(n1, &vec2[i]))
        .sum::<i32>();
    result
}

fn absolute_difference(n1: &i32, n2: &i32) -> i32 {
    if n1 > n2 {
        n1 - n2
    } else {
        n2 - n1
    }
}

fn push_to_vectors(i: usize, vec1: &mut Vec<i32>, n: &&str, vec2: &mut Vec<i32>) {
    if i == 0 {
        load_vector(vec1, n);
    } else {
        load_vector(vec2, n);
    }
}

fn load_vector(vec1: &mut Vec<i32>, n: &&str) {
    vec1.push(read_int(n));
}

fn read_int(n: &&str) -> i32 {
    n.parse::<i32>().unwrap()
}
