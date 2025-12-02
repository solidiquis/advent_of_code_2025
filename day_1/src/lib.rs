use std::{fs::read_to_string, path::Path};

pub fn solution<P: AsRef<Path>>(input: P) -> usize {
    let input_file = input.as_ref();
    let instructions = read_to_string(input_file).expect("failed to read input file");

    let num_dials = 100;
    let dial = (0..num_dials).collect::<Vec<usize>>();
    let mut pos = 50;
    let mut pw = 0;

    for (i, instruction) in instructions.lines().enumerate() {
        let line = i + 1;
        let mut chars = instruction.chars();
        let direction = chars.next().unwrap_or_else(|| {
            panic!("invalid instruction on line {line}. Encountered empty string")
        });
        let num_rotations = chars
            .collect::<String>()
            .parse::<usize>()
            .map(|n| n % num_dials)
            .unwrap_or_else(|_| {
                panic!("invalid instruction on line {line}. Failed to parse number of rotations")
            });

        match direction {
            'R' => pos = (pos + num_rotations) % num_dials,
            'L' => {
                pos = pos
                    .checked_sub(num_rotations)
                    .unwrap_or_else(|| num_dials - (num_rotations - pos))
            }
            _ => panic!("invalid instruction on line {line}. Expected a prefix of 'L' or 'R'"),
        }
        pw += usize::from(dial[pos] == 0)
    }
    pw
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

        assert_eq!(3, solution(sample_input));

        let input = PathBuf::new()
            .join(env!("CARGO_MANIFEST_DIR"))
            .join("test_cases")
            .join("input.txt");

        assert_eq!(1011, solution(input));
    }
}
