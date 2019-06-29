#[derive(Debug, PartialEq)]
pub struct DNA(String);

#[derive(Debug, PartialEq)]
pub struct RNA(String);

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        if dna.contains("U") { 
            return Err(dna.find("U").unwrap()); 
        } else if dna.contains("X") {
            return Err(dna.find("X").unwrap());
        }
        
        Ok(DNA(dna.to_string()))
    }

    pub fn into_rna(self) -> RNA {
        RNA(self.0.chars()
            .map(|c| match c {
                'C' => "G".to_string(),
                'G' => "C".to_string(),
                'T' => "A".to_string(),
                'A' => "U".to_string(),
                _ => "X".to_string()
            })
            .collect::<String>())
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        if rna.contains("X") || rna.contains("T"){ return Err(0); }

        Ok(RNA(rna.to_string()))
    }
}
