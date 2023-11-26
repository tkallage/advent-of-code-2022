pub fn main(filepath: &str) -> i32 {
    let input_file_content: String = match std::fs::read_to_string(filepath) {
        Ok(content) => content,
        Err(error) => panic!("Error while reading the file: {:?}", error),
    };

    let mut calories: Vec<i32> = vec![];
    calories.push(0);
    let mut current_calories: &mut i32 = &mut calories[0];

    for line in input_file_content.lines() {
        // parse line as i32
        match line.parse::<i32>() {
            Ok(number) => {
                *current_calories += number;
            }
            Err(_e) => {
                calories.insert(0, 0);
                current_calories = &mut calories[0];
            }
        };
    }

    calories.sort_unstable();

    println!("All calories: {:#?}", calories);

    let top_three = &calories[calories.len() - 3..calories.len()];

    println!("Top three: {:#?}", top_three);

    let result: i32 = top_three.iter().sum();

    println!("Sum of top three: {:#?}", result);

    result
}
