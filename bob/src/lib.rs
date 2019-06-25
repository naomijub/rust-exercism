pub fn reply(s: &str) -> &str {
    if is_screaming(&s) {
        return "Whoa, chill out!"
    } else if is_question(&s) {
        return "Sure."
    } else if is_silence(&s) {
        return "Fine. Be that way!"
    }
    "Whatever."
}

fn is_screaming(s: &str) -> bool {
    let chars = s.trim_matches(|c: char| !c.is_alphabetic());
    !chars.is_empty() && chars.to_uppercase() == chars
}

fn is_question(s: &str) -> bool {
    s.trim_end().ends_with("?")
}

fn is_silence(s: &str) -> bool {
    s.trim().is_empty()
}
