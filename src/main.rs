extern crate htwdresden;
#[macro_use]
extern crate prettytable;

use htwdresden::{Login, Course, Grade};
use prettytable::Table;

fn main() {
    let login = Login::new("", "");
    let courses = Course::get(&login).expect("Failed to get courses");
    let grades = Grade::get(&login, &courses[0]).expect("Failed to get grades");

    let mut table = Table::new();
    table.add_row(row!["Semester", "Date", "Exam", "Credits", "Form", "Try", "Grade", "Status"]);

    for grade in grades {
        table.add_row(row![grade.semester,
                           grade.exam_date,
                           grade.exam_txt,
                           grade.ects_credits,
                           grade.exam_form,
                           grade.try_count,
                           grade.grade,
                           grade.status]);
    }

    table.printstd();
}
