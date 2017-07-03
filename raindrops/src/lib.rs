pub fn raindrops(num: i32) -> String {
    let mut answer = String::new();
    if num % 3 == 0 {
        answer.push_str("Pling");
    }
    if num % 5 == 0 {
        answer.push_str("Plang");
    }
    if num % 7 == 0 {
        answer.push_str("Plong");
    }
    if answer.is_empty() { 
        let s = format!("{}", num);
        answer.push_str(&s);
    }
    answer
}
