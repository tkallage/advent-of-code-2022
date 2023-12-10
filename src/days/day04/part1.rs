use regex::Regex;

pub fn main(filepath: &str) {
    let input_file_content: String = match std::fs::read_to_string(filepath) {
        Ok(content) => content,
        Err(error) => panic!("Error while reading the file: {:?}", error),
    };

    let mut result = 0;

    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();

    for line in input_file_content.lines() {
        // let ranges = line
        //     .split(",")
        //     .map(|range_str| {
        //         let splitted: Vec<u32> = range_str.split("-").map(|x| x.parse().unwrap()).collect();
        //         (splitted[0], splitted[1])
        //     })
        //     .collect::<(_, _)>();

        let m = match re.captures(line) {
            Some(content) => content,
            _ => panic!("Error matching regex for line {}", line),
        };
        let a_min: u32 = m.get(1).unwrap().as_str().parse().unwrap();
        let a_max: u32 = m.get(2).unwrap().as_str().parse().unwrap();
        let b_min: u32 = m.get(3).unwrap().as_str().parse().unwrap();
        let b_max: u32 = m.get(4).unwrap().as_str().parse().unwrap();

        let one_includes_the_other = if a_min <= b_min && a_max >= b_max {
            true
        } else if b_min <= a_min && b_max >= a_max {
            true
        } else {
            false
        };

        // println!(
        //     "a: [{}, {}], b: [{}, {}] :: {}",
        //     a_min, a_max, b_min, b_max, one_includes_the_other
        // );

        if one_includes_the_other {
            result += 1;
        }
    }

    println!("Result: {}", result);
}
