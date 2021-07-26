use std::collections::HashMap;

// ACGT
const VALIDS: &str = "ACGT";

fn has_invalid_seq(dna: &str) -> Option<char> {
    let mut invalid = None;
    dna.chars().any(|f| {
        let res = !VALIDS.contains(f);
        if let true = res {
            invalid = Some(f);
        }
        res
    });

    invalid
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !VALIDS.contains(nucleotide) {
        return Err(nucleotide);
    }

    if let Some(z) = has_invalid_seq(dna) {
        return Err(z);
    }
    Ok(dna.chars().into_iter().filter(|f| *f == nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    if let Some(z) = has_invalid_seq(dna) {
        return Err(z);
    }
    let mut ans: HashMap<char, usize> = VALIDS.chars().map(|v| (v, 0)).collect();
    for ch in dna.chars() {
        ans.insert(ch, *ans.get(&ch).get_or_insert(&0) + 1);
    }
    Ok(ans)
}
