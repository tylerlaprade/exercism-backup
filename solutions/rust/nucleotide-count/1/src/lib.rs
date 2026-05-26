use std::collections::HashMap;

const NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !NUCLEOTIDES.contains(&nucleotide) {
        Err(nucleotide)
    } else {
        let mut count = 0;
        for char in dna.chars() {
            if !NUCLEOTIDES.contains(&char) {
                return Err(char);
            }
            if nucleotide == char {
                count += 1;
            }
        }
        Ok(count)
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut hash_map = HashMap::from(NUCLEOTIDES.map(|n| (n, 0)));
    for nucleotide in dna.chars() {
        if let Some(count) = hash_map.get_mut(&nucleotide) {
            *count += 1;
        } else {
            return Err(nucleotide);
        }
    }
    Ok(hash_map)
}
