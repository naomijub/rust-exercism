pub struct School {
    grades: Vec<Student>
}

struct Student(u32,String);

impl School {
    pub fn new() -> School {
        School{grades: vec![]}
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.grades.push(Student(grade,student.to_string()));
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades = self.grades.iter().map(|s| s.0).collect::<Vec<u32>>().to_vec();
        grades.dedup();
        grades.sort();
        grades
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        match self.grades().contains(&grade) {
            true => {let mut students = 
                            self.grades
                            .iter()
                            .filter(|x| x.0 == grade)
                            .map(|s| s.1.to_owned())
                            .collect::<Vec<String>>();
                    students.sort();
                    Some(students)},
            false => None
        }
    }
}
