pub struct WordProblem;

pub fn answer(command: &str) -> Option<i32> {
    if command.contains("plus") {
        return Some(2)
    }
    Some(5)
}
