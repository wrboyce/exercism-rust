pub fn reply(message: &str) -> &str {
    let m = message.trim();
    match (m.is_empty(), is_angry(m), is_question(m)) {
        (true, _, _) => "Fine. Be that way!",
        (false, true, true) => "Calm down, I know what I'm doing!",
        (false, true, false) => "Whoa, chill out!",
        (false, false, true) => "Sure.",
        _ => "Whatever.",
    }
}

fn is_angry(message: &str) -> bool {
    message.matches(char::is_alphabetic).count() > 0 && message.to_uppercase() == message
}

fn is_question(message: &str) -> bool {
    message.ends_with('?')
}
