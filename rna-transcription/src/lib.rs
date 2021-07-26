const VALID_DNA: &str = "ACTG";
const VALID_RNA: &str = "UGAC";

#[derive(Debug, PartialEq)]
pub struct Dna<'a>(&'a str);

#[derive(Debug, PartialEq)]
pub struct Rna(String);

impl<'a> Dna<'a> {
    // unimplemented!("Construct new Dna from '{}' string. If string contains invalid
    //     nucleotides return index of first invalid nucleotide", dna);
    pub fn new(dna: &'a str) -> Result<Dna, usize> {
        let res = dna.chars().enumerate().try_for_each(|(i, v)| {
            if let false = VALID_DNA.contains(v) {
                return Err(i);
            }
            Ok(())
        });
        if let Err(s) = res {
            Err(s)
        } else {
            Ok(Self(dna))
        }
    }

    pub fn into_rna(&self) -> Rna {
        
        Rna (
            self.0.chars().map(|s| VALID_RNA.as_bytes()[VALID_DNA.find(s).unwrap()] as char).collect::<String>()
            )
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let res = rna.chars().enumerate().try_for_each(|(i, v)| {
            if let false = VALID_RNA.contains(v) {
                return Err(i);
            }
            Ok(())
        });
        if let Err(s) = res {
            Err(s)
        } else {
            Ok(Self(rna.to_string()))
        }
    }
}
