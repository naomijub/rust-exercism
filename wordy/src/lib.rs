// My Objective was to try to find a less verbose answer than the others I saw.
#[derive(Debug, PartialEq, Clone)]
enum Math {
    Plus,
    Minus,
    Divided,
    Multiplied,
    Num(i32),
    Err,
    Start,
}

pub fn answer(command: &str) -> Option<i32> {
    if !command.starts_with("What is") {
        return None;
    }
    let operators = command
        .replace("What is", "")
        .replace("?", "")
        .replace("by", "")
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|c| match c {
            "plus" => Math::Plus,
            "minus" => Math::Minus,
            "multiplied" => Math::Multiplied,
            "divided" => Math::Divided,
            _ if c.parse::<i32>().is_ok() => Math::Num(c.parse::<i32>().unwrap()),
            _ => Math::Err,
        })
        .collect::<Vec<Math>>();

    if operators.len() == 2 {
        match operators.last() {
            Some(Math::Num(n)) => return Some(*n),
            _ => return None,
        }
    }

    let mut current_op = Math::Start;
    let mut val = 0i32;

    operators.into_iter().for_each(|e| match e {
        Math::Num(n) if current_op == Math::Start => {
            val = n;
            current_op = e;
        }
        Math::Num(n) => match current_op {
            Math::Plus => {
                val = val + n;
                current_op = Math::Num(n);
            }
            Math::Minus => {
                val = val - n;
                current_op = Math::Num(n);
            }
            Math::Multiplied => {
                val = val * n;
                current_op = Math::Num(n);
            }
            Math::Divided => {
                val = val / n;
                current_op = Math::Num(n);
            }
            _ => current_op = Math::Err,
        },
        Math::Err => current_op = Math::Err,
        e if current_op.clone() == Math::Plus => current_op = Math::Err,
        e => current_op = e,
    });

    match current_op {
        Math::Num(n) => Some(val),
        _ => None,
    }
}
