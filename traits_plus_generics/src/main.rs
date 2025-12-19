// -------- TRAIT --------
trait ShowInfo {
    fn show_info(&self) -> String;
}

// ------ SHARED STUDENT FIELDS ------
struct StudentCore {
    major: String,
    gpa: f32,
}

// -------- UNDERGRAD --------
struct Undergrad {
    core: StudentCore,
}

impl ShowInfo for Undergrad {
    fn show_info(&self) -> String {
        format!("Undergrad → Major: {}, GPA: {}", self.core.major, self.core.gpa)
    }
}

// -------- GRAD --------
struct Grad {
    core: StudentCore,
    thesis: String,
}

impl ShowInfo for Grad {
    fn show_info(&self) -> String {
        format!(
            "Grad → Major: {}, GPA: {}, Thesis: {}",
            self.core.major, self.core.gpa, self.thesis
        )
    }
}

// -------- ENROLLMENT --------
// Stores any student that implements ShowInfo
struct Enrollment<T: ShowInfo> {
    roster: Vec<T>,
}

impl<T: ShowInfo> Enrollment<T> {
    fn new() -> Self {
        Enrollment { roster: vec![] }
    }

    fn add(&mut self, student: T) {
        self.roster.push(student);
    }

    fn display_all(&self)
    where
        T: ShowInfo,
    {
        self.roster
            .iter()
            .for_each(|student| println!("{}", student.show_info()));
    }
}

// -------- MAIN --------
fn main() {
    let u = Undergrad {
        core: StudentCore {
            major: "Computer Engineering".to_string(),
            gpa: 3.4,
        },
    };

    let g = Grad {
        core: StudentCore {
            major: "Electrical Engineering".to_string(),
            gpa: 3.8,
        },
        thesis: "High-speed Analog-to-Digital Front End".to_string(),
    };

    // separate enrollments by type
    let mut undergrad_enrollment = Enrollment::new();
    undergrad_enrollment.add(u);

    let mut grad_enrollment = Enrollment::new();
    grad_enrollment.add(g);

    undergrad_enrollment.display_all();
    grad_enrollment.display_all();
}
