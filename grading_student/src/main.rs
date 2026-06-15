use std::io;

fn main() {
    let mut students: Vec<(String, u8)> = Vec::new();

    loop {
        print_menu();
        let choice: String = read_input("Choose an Option");

        match choice.as_str() {
            "1" => add_students(&mut students),
            "2" => display_students(&students),
            "3" => display_average(&students),
            "4" => {
                println!("Bye!");
                break;
            }
            _ => {
                println!("Please choose a number from 1 to 4.");
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
    println!("3. Display class average");
    println!("4. Quit");
}

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

fn add_students(students: &mut Vec<(String, u8)>) {
    let name = read_input("Student name:");

    if name.is_empty() {
        println!("Name cannot be empty");
        return;
    }

    let grade: u8 = read_grade("Student grade:");

    students.push((name, grade));
    println!("Student added.")
}

fn display_students(students: &Vec<(String, u8)>) {
    if students.is_empty() {
        println!("No students yet!");
        return;
    }

    for student in students {
        println!("{} : {}", student.0, student.1);
    }
}

fn display_average(students: &Vec<(String, u8)>) {
    if students.is_empty() {
        println!("Average : no students!");
        return;
    }
    let average = calculate_average(students);
    println!("Class average : {:.2}", average);
}

fn calculate_average(students: &Vec<(String, u8)>) -> f64 {
    if students.is_empty() {
        return 0.0;
    }
    let mut total = 0;
    for student in students {
        total += student.1;
    }

    total as f64 / students.len() as f64
}

fn read_grade(prompt: &str) -> u8 {
    loop {
        let input = read_input(prompt);
        match input.parse::<u8>() {
            Ok(grade) => return grade,
            Err(_) => {
                println!("Please enter a whole number from 0 to 100.");
                continue;
            }
        }
    }
}
