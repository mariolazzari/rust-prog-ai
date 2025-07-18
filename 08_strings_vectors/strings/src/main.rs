fn print_str(s: &str) {
    println!("Print {}", s)
}

fn print_string(s: String) {
    println!("Print {}", s)
}

fn main() {
    let s = "String slice";
    print_str(s);

    let s = String::from("String");
    print_string(s);
}
