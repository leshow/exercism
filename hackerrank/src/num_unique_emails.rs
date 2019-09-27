fn num_unique_emails(emails: Vec<String>) -> usize {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    for email in emails {
        let part: Vec<&str> = email.split('@').collect();
        let mut name = part[0].split('+').next().unwrap().to_owned();
        name.retain(|c| c != '.');
        set.insert(name + "@" + part[1]);
    }
    set.len() as i32
}

#[test]
fn test_num_unique() {
    assert_eq!(
        num_unique_emails(vec![
            "test.email+alex@leetcode.com".into(),
            "test.e.mail+bob.cathy@leetcode.com".into(),
            "testemail+david@lee.tcode.com".into()
        ]),
        2
    );
    assert_eq!(
        num_unique_emails(vec![
            "test.email+alex@leetcode.com".into(),
            "test.email.leet+alex@code.com".into()
        ]),
        2
    );
}
