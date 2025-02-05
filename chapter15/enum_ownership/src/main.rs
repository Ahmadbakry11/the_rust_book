struct Student {
    name: String
}


struct Teacher {
    name: String
}

enum ExamAttendant {
    Examiner(Student),
    Supervisor(Teacher)
}


/*
    While implementing the attendee_type function we made some mistake below
    impl ExamAttendant {
        fn attendee_type(&self) -> Option<Student> {
            match self {
                ExamAttendant::Examiner(student) => Some(student),
                _  => None,
            }
        }
    }

    we made the rtuen type to be of Option<Student>.And this is a mistake
    because we are matching against self which contains references instead of
    the original student params for example.
    So, the results of the matching process is references and
    ExamAttendant::Examiner(student): student here is of type &Student and not Student
    So, the correct solution for the above is to return Option<&Student>
    or here: ExamAttendant::Examiner(student) => Some(student.clone())
*/
impl ExamAttendant {
    fn attendee_type(&self) -> Option<&Student> {
        match self {
            ExamAttendant::Examiner(student) => Some(student),
            _  => None,
        }
    }
}
fn main() {
   let student = Student { name: String::from("Ali") };
   let teacher = Teacher { name: String::from("Kamal") };

   let examiner = ExamAttendant::Examiner(Student { name: String::from("Ali") });
   if let Some(x) = examiner.attendee_type() {
       println!("The student name is {}", x.name)
   }
}
