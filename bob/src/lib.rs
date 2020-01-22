use regex::Regex;

pub fn reply(message: &str) -> &str {
    let msg = message.trim();

    if is_silence(msg) {
        "Fine. Be that way!"
    } else if is_question(msg) {
        "Sure."
    } else if is_shouting(msg) {
        "Whoa, chill out!"
    } else if not_shouting(msg) {
        "Calm down, I know what I'm doing!"
    } else {
        "Whatever."
    }
}

fn is_silence(msg: &str) -> bool {
    msg.trim() == ""
}

fn is_shouting(msg: &str) -> bool {
    msg.to_uppercase() == msg && msg.to_lowercase() != msg &&
        !msg.ends_with("?")
}

fn is_question(msg: &str) -> bool {
    let re = Regex::new(r"[A-Z]{3}+\?").unwrap();
    msg.ends_with("?") && !re.is_match(msg)
}

fn not_shouting(msg: &str) -> bool {
    let re = Regex::new(r"[A-Z]\?").unwrap();
    re.is_match(msg)
}