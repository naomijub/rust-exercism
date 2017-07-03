pub fn raindrops(num: i32) -> String {
    let mut answer = String::new();
    if num % 3 == 0 {
        answer.push_str("Pling");
    } else if num % 5 == 0 {
        answer.push_str("Plang");
    } else if num % 7 == 0 {
        answer.push_str("Plong");
    } else {
        let s = format!("{}", num);
        answer.push_str(&s);
    }
    answer
}
