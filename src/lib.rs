pub fn read_file_to_vec_of_parsed_ints(p: &str) -> Vec<usize> {
    std::fs::read_to_string(p)
        .unwrap()
        .split_whitespace()
        .map(|x| -> usize { x.parse().unwrap() })
        .collect()
}
