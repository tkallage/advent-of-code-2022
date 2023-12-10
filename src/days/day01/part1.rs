pub fn main(filepath: &str) {
    let input_file_content: String = match std::fs::read_to_string(filepath) {
        Ok(content) => content,
        Err(error) => panic!("Error while reading the file: {:?}", error),
    };

    let mut max_calories: i32 = 0;
    let mut current_calories: i32 = 0;

    for line in input_file_content.lines() {
        // parse line as i32
        match line.parse::<i32>() {
            Ok(number) => {
                current_calories += number;
                if current_calories > max_calories {
                    max_calories = current_calories;
                }
            }
            Err(_e) => {
                current_calories = 0;
            }
        };
    }

    println!("Max calories: {}", max_calories)
}
