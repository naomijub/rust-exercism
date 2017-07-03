extern crate regex;

use regex::Regex;

pub fn reply(s: &str) -> &str {
    if is_screaming(&s) {
        return "Whoa, chill out!"
    }
    "Whatever."
}

fn is_screaming(s: &str) -> bool {
    let re = Regex::new(r"[AZ]").unwrap();
    re.is_match(&s)
}
