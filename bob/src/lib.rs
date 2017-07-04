pub fn reply(s: &str) -> &str {
    if is_screaming(&s) {
        return "Whoa, chill out!"
    }
    "Whatever."
}

fn is_screaming(s: &str) -> bool {
    let chars = s.trim_matches(|c: char| !c.is_alphabetic());
    !chars.is_empty() && chars.to_uppercase() == chars
}
