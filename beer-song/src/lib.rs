use std::borrow::Cow;

pub fn verse(n: u8) -> String {
    format!("{} of beer on the wall, {} of beer.\n{}, {} of beer on the wall.\n",
            capitalize(&bottle_tense(n)),
            bottle_tense(n),
            instruction(n),
            bottle_tense(next(n)))
}

pub fn sing(s: u8, e: u8) -> String {
    let verses = (s..e).rev().map(|n| verse(n)).collect::<Vec<String>>();
    verses.join("\n")
}

fn capitalize<'a>(string: &'a str) -> Cow<'a, str> {
    if string.chars()
           .nth(0)
           .unwrap()
           .is_numeric() {
        Cow::Borrowed(string)
    } else {
        let mut first = string.to_owned();
        let last = first.split_off(1);
        Cow::Owned(first.to_uppercase() + &last)
    }
}

fn bottle_tense(n: u8) -> String {
    match n {
        0 => "no more bottles".to_owned(),
        1 => "1 bottle".to_owned(),
        _ => format!("{} bottles", n),
    }
}

fn instruction(n: u8) -> &'static str {
    match n {
        0 => "Go to the store and buy some more",
        1 => "Take it down and pass it around",
        _ => "Take one down and pass it around",
    }
}

fn next(n: u8) -> u8 {
    match n {
        0 => 99,
        _ => n - 1,
    }
}
