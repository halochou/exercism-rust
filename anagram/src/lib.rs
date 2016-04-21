fn is_anagram(left : &str, right : &str) -> bool {
    if left.to_lowercase() == right.to_lowercase() {
        return false;
    }
    let mut sorted_left : Vec<char> = left.to_lowercase().chars().collect();
    sorted_left.sort();

    let mut sorted_right : Vec<char> = right.to_lowercase().chars().collect();
    sorted_right.sort();

    sorted_left == sorted_right
}

pub fn anagrams_for<'a>(key : &str, inputs : &[&'a str]) -> Vec<&'a str> {
    inputs.iter()
        .filter(|input| is_anagram(key, input))
        .cloned()
        .collect()
}
