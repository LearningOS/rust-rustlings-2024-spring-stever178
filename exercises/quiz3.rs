// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

use std::fmt;

// method_1 -----------------------------------------
#[derive(Debug)]
enum Grade {
    Numeric(f32),
    Alphabetic(String),
}

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Grade::Numeric(value) => write!(f, "{}", value),
            Grade::Alphabetic(letter) => write!(f, "{}", letter),
        }
    }
}

// method_2 -----------------------------------------
trait MyDisplay:fmt::Display {}
impl MyDisplay for String {}
impl MyDisplay for f32 {}

// original 
struct ReportCard {
    //grade: Grade,
    grade: Box<dyn MyDisplay>,
    student_name: String,
    student_age: u8,
}

impl ReportCard {
    fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            self.student_name, self.student_age, self.grade
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            //grade: Grade::Numeric(2.1),
            grade: Box::new(f32::from(2.1)), 
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            //grade: Grade::Alphabetic("A+".to_string()),
            grade: Box::new(String::from("A+")),  
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}

