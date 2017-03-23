pub fn reply(talk: &str) -> String {
    if talk.len() == 0 {
        "Fine. Be that way!".to_owned()
    } else if talk.ends_with("?") {
        "Sure".to_owned()
    } else if talk.ends_with("!") && talk.to_uppercase() == talk {
        "Whoa, chill out!".to_owned()
    } else {
        "Whatever.".to_owned()
    }
}
