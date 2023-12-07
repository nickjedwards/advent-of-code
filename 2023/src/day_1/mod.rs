pub fn solve(contents: &String) {
    let lines = contents.lines();

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
