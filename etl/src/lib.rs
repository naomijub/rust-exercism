use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.into_iter()
        .flat_map(|(value, letters)| letters.iter().map(move |l| (l.to_lowercase(), value)))
        .fold(BTreeMap::new(), |mut acc, (letter, value)| {
            acc.insert(letter.to_string().chars().nth(0).unwrap(), *value);
            acc
        })
}