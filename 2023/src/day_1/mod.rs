pub fn solve(contents: &String) {
    // Replace spelled numbers with digits.
    let lines: Vec<String> = contents.lines().map(replace_words_with_digit).collect();

    let mut calibrations: Vec<u32> = Vec::new();

    for line in lines {
        // Extract the first number.
        let first_number = line.chars()
            .find_map(|c| c.to_digit(10));
        // Extract the last number.
        let last_number = line.chars()
            .rev()
            .find_map(|c| c.to_digit(10));
        // Smudge 'em together.
        let double_digits = match (first_number, last_number) {
            (Some(first), Some(last)) => first * 10 + last,
            _ => 0,
        };
        // Chuck 'em in the Vec.
        calibrations.push(double_digits);
    }

    // Sum those bad boys.
    println!("Day 1: {}", calibrations.iter().sum::<u32>());
}

fn replace_words_with_digit(line: &str) -> String {
    let words = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let mut matches: Vec<(usize, &str)> = Vec::new();

    for (word, _digit) in &words {
        for (k, v) in line.match_indices(word) {
            matches.push((k, v));
        }
    }

    matches.sort();

    let mut mutated_string = String::from(line);

    for (_k, v) in &matches {
        if let Some(word) = words.iter().find(|(value, _)| value == v) {
            mutated_string = mutated_string.replace(word.0, &[word.1, word.0].join(""));
        }
    }

    mutated_string
}