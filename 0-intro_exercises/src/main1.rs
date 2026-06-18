fn main() {
    let s1: String = String::from("BONJOUR");
    let s2: String = s1;
    print_string(&s2);
}

fn print_string(word: &String) {
    println!("{}", word)
}
