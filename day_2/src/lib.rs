use std::{fs::read_to_string, path::Path};

pub fn solution<P: AsRef<Path>>(input: P) -> usize {
    let input_file = input.as_ref();
    let ranges = read_to_string(input_file).expect("failed to read input file");

    let mut res = 0;

    for range in ranges.split(",") {
        let mut bounds = range
            .split("-")
            .filter_map(|b| b.trim().parse::<usize>().ok());

        let left = bounds
            .next()
            .unwrap_or_else(|| panic!("expected left bound of range {range} to be present"));

        let right = bounds
            .next()
            .unwrap_or_else(|| panic!("expected right bound of range {range} to be present"));

        for product_id in left..=right {
            let id = format!("{product_id}");

            // Odd length is automatically valid
            if id.len() & 1 == 1 {
                continue;
            }
            let half = id.len() / 2;

            let left = &id[0..half];
            let right = &id[half..];

            if left == right {
                res += product_id;
            }
        }
    }
    res
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::solution;

    #[test]
    fn test_solution() {
        let sample_input = PathBuf::new()
            .join(env!("CARGO_MANIFEST_DIR"))
            .join("test_cases")
            .join("sample.txt");

        assert_eq!(1_227_775_554, solution(sample_input));

        let input = PathBuf::new()
            .join(env!("CARGO_MANIFEST_DIR"))
            .join("test_cases")
            .join("input.txt");

        assert_eq!(23_701_357_374, solution(input));
    }
}
