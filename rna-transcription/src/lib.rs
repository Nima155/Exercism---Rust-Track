const VALID_DNA: &str = "ACTG";
const VALID_RNA: &str = "UGAC";
use new_attr::with_attrs;

#[with_attrs("ACTG")]
pub struct Dna;

#[with_attrs("UGAC")]
pub struct Rna;

impl Dna {
    pub fn into_rna(self) -> Rna {
        Rna(self
            .0
            .chars()
            .map(|s| VALID_RNA.as_bytes()[VALID_DNA.find(s).unwrap()] as char)
            .collect::<String>())
    }
}
