use std::collections::HashSet;

pub fn main(filepath: &str) {
    let input_file_content: String = match std::fs::read_to_string(filepath) {
        Ok(content) => content,
        Err(error) => panic!("Error while reading the file: {:?}", error),
    };

    let mut prio_sum = 0;

    let all_lines = input_file_content.lines().collect::<Vec<&str>>();

    for (i_line, _) in all_lines.iter().enumerate().step_by(3) {
        let line_a = all_lines[i_line].chars().collect();

        let line_b = all_lines[i_line + 1].chars().collect();
        let line_c = all_lines[i_line + 2].chars().collect();

        let chars_in_ab = intersection(line_a, line_b);
        let badge = intersection(chars_in_ab, line_c);

        assert!(badge.len() == 1);

        let badge_prio = get_priority(badge[0]);

        prio_sum += badge_prio;
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
