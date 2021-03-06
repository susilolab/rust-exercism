use regex::Regex;

pub fn reply(message: &str) -> &str {
    let msg = message.trim();

    match (is_silence(msg), is_question(msg), is_shouting(msg), not_shouting(msg)) {
        (true, false, false, false) => "Fine. Be that way!",
        (false, true, false, false) => "Sure.",
        (false, false, true, false) => "Whoa, chill out!",
        (false, false, false, true) => "Calm down, I know what I'm doing!",
        (false, false, false, false) => "Whatever.",
        _ => "Sure.",
    }
}

fn is_silence(msg: &str) -> bool {
    msg.trim().is_empty()
}

fn is_shouting(msg: &str) -> bool {
    msg.to_uppercase() == msg && msg.to_lowercase() != msg &&
        !msg.ends_with('?')
}

fn is_question(msg: &str) -> bool {
    msg.ends_with("?") && !is_all_uppercase(msg)
    // let re = Regex::new(r"[A-Z]{3}+\?").unwrap();
    // msg.ends_with('?') && !re.is_match(msg)

    // "aasdadas".matches(std::char::char::)
}

fn not_shouting(msg: &str) -> bool {
    let re = Regex::new(r"[A-Z]\?").unwrap();
    re.is_match(msg)
}

fn is_all_uppercase(word: &str) -> bool {
	let mut res = true;

	for w in word.chars() {
		if w.is_alphabetic() && w.is_uppercase() {
			res = res && true;
		} else if w.is_alphabetic() && !w.is_uppercase() {
			res = res && false;
		}
	}

	res
}
