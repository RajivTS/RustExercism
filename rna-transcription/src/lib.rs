#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    strand: Vec<char>
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    strand: Vec<char>
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let mut strand = Vec::new();
        for (idx, nucleotide) in dna.char_indices() {
            if !&['A', 'C', 'G', 'T'].contains(&nucleotide) {
                return Err(idx);
            } else {
                strand.push(nucleotide);
            }
        }
        Ok(Self {
            strand
        })
    }

    pub fn into_rna(self) -> Rna {
        let strand = self.strand.into_iter().map(|c| match c {
            'A' => 'U',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            _ => panic!("Illegal nucleotide in DNA strand")
        }).collect();
        Rna {
            strand
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let mut strand = Vec::new();
        for (idx, nucleotide) in rna.char_indices() {
            if !&['A', 'C', 'G', 'U'].contains(&nucleotide) {
                return Err(idx);
            } else {
                strand.push(nucleotide);
            }
        }
        Ok(Self {
            strand
        })
    }
}
