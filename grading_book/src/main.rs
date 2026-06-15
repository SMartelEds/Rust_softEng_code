use std::io;

// What is this?
// An enum is a type with a fixed list of possible values.
// Why do we use it?
// A grade can be classified into one clear category.
enum GradeStatus {
    Excellent,
    Pass,
    Retake,
    Invalid,
}

fn main() {
    // What is this?
    // A tuple groups values by position. Here, each student is (name, grade).
    // Why do we use it?
    // Vec lets the gradebook grow while the program runs.
    let mut students: Vec<(String, u8)> = Vec::new();

    loop {
        print_menu();
        let choice: String = read_input("Choose an option:");

        match choice.as_str() {
            "1" => add_student(&mut students),
            "2" => display_students(&students),
            "3" => search_student_menu(&students),
            "4" => update_grade(&mut students),
            "5" => display_average(&students),
            "6" => display_best_student(&students),
            "7" => display_classifications(&students),
            "8" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Please choose a number from 1 to 8.");
                continue;
            }
        }
    }
}

fn print_menu() {
    println!();
    println!("=== Gradebook CLI ===");
    println!("1. Add a student");
    println!("2. Display all students");
    println!("3. Search for a student");
    println!("4. Update a student's grade");
    println!("5. Display class average");
    println!("6. Display best student");
    println!("7. Display grade classifications");
    println!("8. Quit");
}

fn read_input(prompt: &str) -> String {
    println!("{prompt}");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

fn read_grade(prompt: &str) -> u8 {
    loop {
        let input: String = read_input(prompt);

        match input.parse::<u8>() {
            Ok(grade) => return grade,
            Err(_) => {
                println!("Please enter a whole number from 0 to 255.");
                continue;
            }
        }
    }
}

// What is this?
// &mut means this function borrows the vector and is allowed to change it.
// Why do we use it?
// Adding a student pushes a new tuple into the vector.
fn add_student(students: &mut Vec<(String, u8)>) {
    let name: String = read_input("Student name:");

    if name.is_empty() {
        println!("Name cannot be empty.");
        return;
    }

    let grade: u8 = read_grade("Student grade:");

    students.push((name, grade));

    println!("Student added.");
}

// What is this?
// & means this function borrows the vector for reading only.
// Why do we use it?
// Displaying students should not change the gradebook.
fn display_students(students: &[(String, u8)]) {
    if students.is_empty() {
        println!("No students yet.");
        return;
    }

    let mut index: usize = 0;

    while index < students.len() {
        let position: usize = index + 1;
        println!("{position}. {} - {}", students[index].0, students[index].1);

        index += 1;
    }
}

fn search_student_menu(students: &[(String, u8)]) {
    let name: String = read_input("Search name:");

    match find_student_index(students, &name) {
        Some(index) => println!("Found: {} - {}", students[index].0, students[index].1),
        None => println!("No student named '{name}' was found."),
    }
}

fn find_student_index(students: &[(String, u8)], searched_name: &str) -> Option<usize> {
    let mut index: usize = 0;

    while index < students.len() {
        if students[index].0.eq_ignore_ascii_case(searched_name) {
            return Some(index);
        }

        index += 1;
    }

    None
}

fn update_grade(students: &mut [(String, u8)]) {
    let name: String = read_input("Student name to update:");

    match find_student_index(students, &name) {
        Some(index) => {
            let new_grade: u8 = read_grade("New grade:");
            students[index].1 = new_grade;
            println!("Grade updated.");
        }
        None => println!("No student named '{name}' was found."),
    }
}

fn calculate_average(students: &[(String, u8)]) -> f64 {
    if students.is_empty() {
        return 0.0;
    }

    let mut total: u32 = 0;

    for student in students {
        total += u32::from(student.1);
    }

    total as f64 / students.len() as f64
}

fn display_average(students: &[(String, u8)]) {
    if students.is_empty() {
        println!("No students yet.");
        return;
    }

    let average: f64 = calculate_average(students);
    println!("Class average: {average:.2}");
}

fn display_best_student(students: &[(String, u8)]) {
    if students.is_empty() {
        println!("No students yet.");
        return;
    }

    let mut best_index: usize = 0;

    for index in 1..students.len() {
        if students[index].1 > students[best_index].1 {
            best_index = index;
        }
    }

    println!(
        "Best student: {} - {}",
        students[best_index].0, students[best_index].1
    );
}

fn classify_grade(grade: u8) -> GradeStatus {
    if grade > 100 {
        GradeStatus::Invalid
    } else if grade >= 90 {
        GradeStatus::Excellent
    } else if grade >= 50 {
        GradeStatus::Pass
    } else {
        GradeStatus::Retake
    }
}

fn display_classifications(students: &[(String, u8)]) {
    if students.is_empty() {
        println!("No students yet.");
        return;
    }

    for student in students {
        let passed: bool = student.1 >= 50 && student.1 <= 100;

        match classify_grade(student.1) {
            GradeStatus::Excellent => {
                println!("{}: Excellent (passed: {passed})", student.0);
            }
            GradeStatus::Pass => {
                println!("{}: Pass (passed: {passed})", student.0);
            }
            GradeStatus::Retake => {
                println!("{}: Retake (passed: {passed})", student.0);
            }
            GradeStatus::Invalid => {
                println!("{}: Invalid grade (passed: {passed})", student.0);
            }
        }
    }
}
