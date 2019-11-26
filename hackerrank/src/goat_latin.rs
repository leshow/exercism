pub fn to_goat_latin(s: String) -> String {
    use std::iter;
    let mut r = s
        .split(' ')
        .enumerate()
        .flat_map(|(i, word)| {
            let mut new = vec![];
            let word = word.as_bytes();
            match word[0] {
                b'A' | b'E' | b'I' | b'O' | b'U' | b'o' | b'a' | b'e' | b'i' | b'u' => {
                    new.extend(&word[..]);
                }
                c => {
                    new.extend(&word[1..]);
                    new.push(c);
                }
            }
            new.push(b'm');
            new.push(b'a');
            new.extend(iter::repeat(b'a').take(i + 1));
            new.push(b' ');
            new
        })
        .collect::<Vec<_>>();
    r.pop();
    String::from_utf8(r).unwrap()
}

#[test]
fn test_goat_latin() {
    assert_eq!(
        to_goat_latin("I speak Goat Latin".to_owned()),
        "Imaa peaksmaaa oatGmaaaa atinLmaaaaa".to_owned()
    );
}
