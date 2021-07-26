use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter().fold(BTreeMap::new(), |mut prev, (k, v)| {
        v.iter().for_each(|f| {
            prev.insert((*f).to_ascii_lowercase(), *k);
        });
        prev
    })
}
