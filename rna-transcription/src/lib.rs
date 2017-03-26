#[derive(Debug, PartialEq)]
pub struct RibonucleicAcid {
    nucleotide: String
}

impl RibonucleicAcid {
    pub fn new<S: Into<String>>(s: S) -> RibonucleicAcid {
        RibonucleicAcid {
            nucleotide: s.into()
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DeoxyribonucleicAcid {
    nucleotide: String
}

impl DeoxyribonucleicAcid {
    pub fn new<S: Into<String>>(s: S) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid {
            nucleotide: s.into()
        }
    }
    pub fn to_rna(&self) -> RibonucleicAcid {
        RibonucleicAcid::new(self.nucleotide.chars().map(DeoxyribonucleicAcid::transcribe).collect::<String>())
    }
    fn transcribe(c: char) -> char {
        match c {
            'G' => 'C',
            'C' => 'G',
            'T' => 'A',
            'A' => 'U',
            _ => panic!("unreachable")
        }
    }
}