use rand::Rng;

pub fn encode(key: &str, s: &str) -> Option<String> {
    vigenere(key, s, |c, k| (b'a' + (c + k) % 26) as char)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    vigenere(key, s, |c, k| ((26 + c - k) % 26 + b'a') as char)
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut v = Vec::with_capacity(100);
    for _ in 0..=100 {
        let r: u8 = rand::thread_rng().gen_range(0_u8, 26u8);
        v.push(r);
    }
    let key = v.iter().map(|i| (i + b'a') as char).collect::<String>();
    let encoded = encode(&key, s).unwrap();
    (key, encoded)
}

fn get_key_bytes(key: &str) -> Option<Vec<u8>> {
    if key
        .find(|c: char| c.is_uppercase() || c.is_numeric())
        .is_some()
        || key.is_empty()
    {
        return None;
    }
    Some(
        key.as_bytes()
            .iter()
            .cloned()
            .map(|c| c - b'a')
            .collect::<Vec<u8>>(),
    )
}

fn vigenere<F>(key: &str, s: &str, f: F) -> Option<String>
where
    F: Fn(u8, u8) -> char,
{
    let key_bytes = get_key_bytes(&key[..])?;
    let mut res = String::with_capacity(s.len());

    for (c, k) in s
        .as_bytes()
        .iter()
        .cloned()
        .map(|c| c - b'a')
        .zip(key_bytes.into_iter().cycle())
    {
        res.push(f(c, k));
    }

    Some(res)
}
