// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word and ay is added,
// so first becomes irst-fay.
// Words that start with a vowel have hay added to the end instead (apple becomes apple-hay).
// Keep in mind the details about UTF-8 encoding!

pub fn pig_latin(word: &str) -> String {
    let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
    let mut result: String = String::new();
    let mut chars = word.chars();
    let first_char = chars.next().unwrap();
    if vowels.contains(&first_char) {
        result = format!("{word}-hay");
    } else {
        result = chars.as_str().to_string();
        result = format!("{result}-{first_char}ay")
    }
    println!("res: {result}");
    result
}
