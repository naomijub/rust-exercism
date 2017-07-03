pub fn raindrops(num: i32) -> String {
    let mut answer = String::new();
    if num % 3 == 0 {
        answer.push_str("Pling");
    } else {
        answer.push_str("1");
    }
    answer
}
