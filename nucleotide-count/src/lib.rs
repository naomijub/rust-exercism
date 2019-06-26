use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let valid_dna = dna
        .chars()
        .map(|x| "ACTG".find(x))
        .map(|z| match z {
            None => false,
            Some(_) => true,
        })
        .fold(true, |acc, val| acc && val);
    
    match valid_dna {
        false => Err('X'),
        true => match "AGTC".find(nucleotide) 
            {
                None  => Err(nucleotide),
                Some(_) => Ok(dna.matches(nucleotide).count())
            }
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut nucleotide_count = HashMap::new();
    nucleotide_count.insert('A', count('A', dna).unwrap());
    nucleotide_count.insert('T', count('T', dna).unwrap());
    nucleotide_count.insert('C', count('C', dna).unwrap());
    nucleotide_count.insert('G', count('G', dna).unwrap());

    Ok(nucleotide_count)
}
