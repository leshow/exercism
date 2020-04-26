pub struct StrSplit<'a, 'b> {
    haystack: &'a str,
    delimiter: &'b str,
    start: usize,
}

impl<'a, 'b> StrSplit<'a, 'b> {
    pub fn new(haystack: &'a str, delimiter: &'b str) -> Self {
        Self {
            haystack,
            delimiter,
            start: 0,
        }
    }
}

impl<'a, 'b> Iterator for StrSplit<'a, 'b> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let win_len = self.delimiter.len();
        for i in self.start..=(self.haystack.len() - win_len) {
            let window = &self.haystack[i..(i + win_len)];
            if window == self.delimiter {
                let tmp = self.start;
                self.start = i + win_len;
                let ret = &self.haystack[tmp..i];

                // uncomment to not return "" blocks
                // if ret != "" {
                return Some(ret);
            // } else {
            // continue;
            // }
            } else if i + win_len == self.haystack.len() {
                self.start = i + win_len;
                return Some(window);
            }
        }
        None
    }
}

fn until_char(haystack: &str, c: char) -> &str {
    StrSplit::new(haystack, &format!("{}", c)).next().unwrap()
}

#[test]
fn str_split_it_works() {
    let haystack = "a b c d e     f";

    assert_eq!(
        StrSplit::new(haystack, " ").collect::<Vec<_>>(),
        vec!["a", "b", "c", "d", "e", "", "", "", "", "f"]
    );
}

#[test]
fn str_split_it_works_too() {
    let haystack = "aa--b--cc--dd--";

    assert_eq!(
        StrSplit::new(haystack, "--").collect::<Vec<_>>(),
        vec!["aa", "b", "cc", "dd"]
    );
}
