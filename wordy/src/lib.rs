pub struct WordProblem;

pub fn answer(command: &str) -> Option<i32> {
    let words: Vec<&str> = command.clone()
                            .trim_end_matches('?')
                            .split_whitespace()
                            .collect();
    let mapped_nubers = words.iter().filter(|w| w.parse::<i32>().is_ok())
                                .map(|s| s.parse::<i32>().unwrap());
    match command {
        c if c.contains("plus") => Some(mapped_nubers
                                    .fold(0,|acc, n| acc + n)),
        c if c.contains("minus") => Some(-mapped_nubers
                                    .fold(0,|acc, n| -acc + n)),
        c if c.contains("multiplied") => Some(mapped_nubers
                                    .fold(1,|acc, n| acc * n)),
        c if c.contains("divided") => {let num = 1f32 / words.iter()
                                            .filter(|w| w.parse::<i32>().is_ok())
                                            .map(|s| s.parse::<f32>().unwrap())
                                            .fold(1f32,|acc, n| n / acc);
                                        Some(num as i32)},
        _ => Some(5)
    }
}
