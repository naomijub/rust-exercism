pub struct WordProblem;

pub fn answer(command: &str) -> Option<i32> {
    let words: Vec<&str> = command.clone().trim_end_matches('?').split_whitespace().collect();

    match command {
        c if c.contains("plus") => Some(words.iter()
                                    .filter(|w| w.parse::<i32>().is_ok())
                                    .map(|s| s.parse::<i32>().unwrap())
                                    .fold(0,|acc, n| acc + n)),
        c if c.contains("minus") => Some(-words.iter()
                                    .filter(|w| w.parse::<i32>().is_ok())
                                    .map(|s| s.parse::<i32>().unwrap())
                                    .fold(0,|acc, n| -acc + n)),
        c if c.contains("multiplied") => Some(words.iter()
                                    .filter(|w| w.parse::<i32>().is_ok())
                                    .map(|s| s.parse::<i32>().unwrap())
                                    .fold(1,|acc, n| acc * n)),
        _ => Some(5)
    }
}
