// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
use std::collections::{btree_map::Entry, BTreeMap, BTreeSet};

#[allow(clippy::new_without_default)]
pub struct School {
    students: BTreeMap<u32, BTreeSet<String>>,
}

impl School {
    pub fn new() -> Self {
        Self {
            students: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        match self.students.entry(grade) {
            Entry::Occupied(mut v) => v.get_mut().insert(student.to_string()),
            Entry::Vacant(z) => z.insert(BTreeSet::new()).insert(student.to_string()),
        };
    }

    pub fn grades(&self) -> Vec<u32> {
        self.students.iter().map(|f| *f.0).collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.students.get_key_value(&grade) {
            Some(z) => z.1.clone().into_iter().collect::<Vec<String>>(),

            None => Vec::new(),
        }
    }
}
