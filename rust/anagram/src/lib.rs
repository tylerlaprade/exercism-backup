use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let letter_counts = count_letters(word);
    let lower_word = word.to_lowercase();

    let mut anagrams = HashSet::new();
    for possible_anagram in possible_anagrams {
        if lower_word != possible_anagram.to_lowercase()
            && count_letters(possible_anagram) == letter_counts
        {
            anagrams.insert(*possible_anagram);
        }
    }
    anagrams
}

fn count_letters(word: &str) -> HashMap<char, u32> {
    let mut letter_counts: HashMap<char, u32> = HashMap::new();
    for letter in word.to_lowercase().chars() {
        *letter_counts.entry(letter).or_insert(0) += 1;
    }
    letter_counts
}
