use std::collections::HashMap;

pub struct CodonsInfo {
   pairs: HashMap<&'static str, &'static str>
}

impl CodonsInfo {
    pub fn name_for(&self, codon: &str) -> Option<&str> {
        if !self.pairs.contains_key(codon) { return None; }
        Some(self.pairs.get(codon).unwrap().to_owned())
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&str>> {
        if !self.pairs.contains_key(&rna[0..3])
            { return None; }

        let rna_codons = self.get_rna_codons(&rna);

        Some(rna_codons
            .into_iter() 
            .map(|c| self.name_for(&c).unwrap())
            .take_while(|c| c != &"stop codon")
            .collect::<Vec<&str>>()
        )
    }

    fn get_rna_codons(&self, rna: &str) -> Vec<String> {
        let rna_chars = String::from(rna).chars().collect::<Vec<char>>();
        rna_chars.chunks(3)
            .map(|c| c.to_vec())
            .map(|v| v.into_iter().collect::<String>())
            .collect::<Vec<String>>()
    }
}

pub fn parse<'a>(pairs: Vec<(&'static str, &'static str)>) -> CodonsInfo {
    let map_pairs = pairs
    .into_iter()
        .fold(HashMap::new(),
            |mut acc, c| { acc.insert(c.0, c.1); acc });

    CodonsInfo{pairs: map_pairs}
}
