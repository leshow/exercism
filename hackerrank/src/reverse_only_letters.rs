// 917. Reverse Only Letters
// Easy

// Given a string S, return the "reversed" string where all characters that are
// not a letter stay in the same place, and all letters reverse their positions.
// Example 1:

// Input: "ab-cd"
// Output: "dc-ba"

// Example 2:

// Input: "a-bC-dEf-ghIj"
// Output: "j-Ih-gfE-dCba"

// Example 3:

// Input: "Test1ng-Leet=code-Q!"
// Output: "Qedo1ct-eeLg=ntse-T!"

pub fn reverse_only_letters(s: String) -> String {
    let mut letters = s.chars().filter(|c| c.is_alphabetic()).collect::<Vec<_>>();
    let mut ret = String::new();
    for c in s.chars() {
        if c.is_alphabetic() {
            ret.push(letters.pop().unwrap());
        } else {
            ret.push(c);
        }
    }
    ret
}
