use std::collections::HashMap;

// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[derive(Default)]
pub struct School {
    grade_student_map: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        Self {
            grade_student_map: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.grade_student_map
            .entry(grade)
            .and_modify(|students| students.push(student.to_string()))
            .or_insert_with(|| vec![student.to_string()]);
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades: Vec<_> = self.grade_student_map.keys().copied().collect();
        grades.sort();
        grades
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut students = self
            .grade_student_map
            .get(&grade)
            .cloned()
            .unwrap_or_else(|| Vec::new());
        students.sort();
        students
    }
}
