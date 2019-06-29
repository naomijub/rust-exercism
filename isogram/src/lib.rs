use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    let char_map: HashMap<char, i32> =  candidate
      .to_lowercase()
      .chars()
      .filter(|c| c.is_alphanumeric())
      .fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });
    char_map
        .values()
        .fold(true,|acc,x| 
            if x > &1 { acc && false } 
            else { acc } )
}
