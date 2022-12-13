use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    nucleotide_counts(dna)
        .and_then(|dna_count| dna_count.get(&nucleotide).copied().ok_or(nucleotide))
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut nucleotide_map = HashMap::from_iter([('A', 0), ('C', 0), ('G', 0), ('T', 0)]);
    for nucleotide in dna.chars() {
        match nucleotide {
            'A' | 'C' | 'G' | 'T' => {
                nucleotide_map
                    .entry(nucleotide)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
            _ => return Err(nucleotide),
        }
    }
    Ok(nucleotide_map)
}
