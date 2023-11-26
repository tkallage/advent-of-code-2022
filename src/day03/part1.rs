use std::collections::HashSet;

pub fn main(filepath: &str) {
    let input_file_content: String = match std::fs::read_to_string(filepath) {
        Ok(content) => content,
        Err(error) => panic!("Error while reading the file: {:?}", error),
    };

    let mut prio_sum = 0;

    for line in input_file_content.lines() {
        let len = line.len();

        let first_half: Vec<char> = line[0..len / 2].chars().collect();
        let second_half: Vec<char> = line[len / 2..len].chars().collect();

        let duplicates = intersection(first_half, second_half);

        println!("{:?}", duplicates);
        println!(
            "{:?}",
            duplicates
                .iter()
                .map(|c| get_priority(*c))
                .collect::<Vec<i32>>()
        );

        duplicates
            .iter()
            .map(|c| get_priority(*c))
            .for_each(|prio| prio_sum += prio);
    }

    println!("Prio sum: {}", prio_sum);
}

fn intersection(a: Vec<char>, b: Vec<char>) -> Vec<char> {
    let mut result = HashSet::<char>::new();

    for a in a {
        if b.contains(&a) {
            result.insert(a);
        }
    }

    result.into_iter().collect()
}

fn get_priority(c: char) -> i32 {
    const ASCII_CODE_LOWER_A: i32 = 'a' as i32;
    const ASCII_CODE_LOWER_Z: i32 = 'z' as i32;
    const ASCII_CODE_UPPER_A: i32 = 'A' as i32;
    const ASCII_CODE_UPPER_Z: i32 = 'Z' as i32;

    let ascii_code = c as i32;

    if ascii_code >= ASCII_CODE_LOWER_A && ascii_code <= ASCII_CODE_LOWER_Z {
        return ascii_code - ASCII_CODE_LOWER_A + 1;
    }
    if ascii_code >= ASCII_CODE_UPPER_A && ascii_code <= ASCII_CODE_UPPER_Z {
        return ascii_code - ASCII_CODE_UPPER_A + 27;
    }

    panic!("Invalid character");
}
