pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let question = message.ends_with('?');
    let yell = message
        .chars()
        .filter(|ch| ch.is_alphabetic())
        .all(|ch| ch.is_uppercase())
        && message.chars().any(|ch| ch.is_alphabetic());
    match (question, yell) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Sure.",
        (false, true) => "Whoa, chill out!",
        _ if message.is_empty() => "Fine. Be that way!",
        _ => "Whatever.",
    }
}

// https://exercism.org/tracks/rust/exercises/bob/solutions/mikechris
fn is_yelling(message: &str) -> bool {
    let have_letters: bool = message.chars().any(|x| x.is_alphabetic());
    message.to_uppercase() == message && have_letters
}
pub fn reply1(message: &str) -> &str {
    match message.trim() {
        m if m.len() == 0 => "Fine. Be that way!",
        m if m.ends_with("?") && is_yelling(m) => "Calm down, I know what I'm doing!",
        m if m.ends_with("?") => "Sure.",
        m if is_yelling(m) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
