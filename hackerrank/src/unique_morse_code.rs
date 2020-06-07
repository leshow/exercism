// 804. Unique Morse Code Words
// Easy
// International Morse Code defines a standard encoding where each letter is
// mapped to a series of dots and dashes, as follows: "a" maps to ".-", "b" maps
// to "-...", "c" maps to "-.-.", and so on. For convenience, the full table for
// the 26 letters of the English alphabet is given below: [".-","-...","-.-.","
// -..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.",
// "--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."] Now, given a
// list of words, each word can be written as a concatenation of the Morse code
// of each letter. For example, "cba" can be written as "-.-..--...", (which is
// the concatenation "-.-." + "-..." + ".-"). We'll call such a concatenation,
// the transformation of a word. Return the number of different transformations
// among all words we have. Example:
// Input: words = ["gin", "zen", "gig", "msg"]
// Output: 2
// Explanation:
// The transformation of each word is:
// "gin" -> "--...-."
// "zen" -> "--...-."
// "gig" -> "--...--."
// "msg" -> "--...--."
// There are 2 different transformations, "--...-." and "--...--.".

pub fn unique_morse_representations(words: Vec<String>) -> i32 {
    use std::collections::HashSet;
    let alphas = [
        ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
        "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--",
        "--..",
    ];
    let words = words
        .into_iter()
        .map(|word| {
            word.chars()
                .map(|c| alphas[(c as u8 - b'a') as usize])
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<HashSet<_>>();

    println!("{:?}", words);
    words.len() as i32
}
