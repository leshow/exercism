// 557. Reverse Words in a String III
// Easy
// Given a string, you need to reverse the order of characters in each word
// within a sentence while still preserving whitespace and initial word order.
// Example 1:
// Input: "Let's take LeetCode contest"
// Output: "s'teL ekat edoCteeL tsetnoc"

pub fn reverse_words(s: String) -> String {
    s.split(" ")
        .map(|word| word.chars().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}
