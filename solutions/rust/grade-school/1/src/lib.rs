use std::collections::BTreeMap;

pub struct School {
    roster: BTreeMap<String, u32>,
}

impl School {
    pub fn new() -> School {
        Self {
            roster: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if !self.roster.contains_key(student) {
            self.roster.insert(student.to_string(), grade);
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut output: Vec<u32> = self.roster.values().cloned().collect();
        output.dedup();

        output
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.roster.iter().filter(|(_name, _grade)| **_grade == grade).map(|(name, _grade)| name.clone()).collect()
    }
}
