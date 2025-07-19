fn print_str(s: &str) {
    println!("Print {}", s)
}

fn print_string(s: String) {
    println!("Print {}", s)
}

fn main() {
    // string size known at compilation time
    let s = "String slice";
    print_str(s);

    // dynamic string size
    let s = String::from("String");
    print_string(s);

    // string manipulation
    let sentence = "Ciao io sono Mario Lazzari.";
    println!("{}", sentence);

    // string iteration
    for c in sentence.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => println!("vowel {}", c),
            _ => continue,
        }
    }

    // get first 3 chars of a slice
    println!("{}", &sentence[0..=3]);

    // split string into a vector
    let words: Vec<&str> = sentence.split_whitespace().collect();
    println!("{:?}", words);

    // reverse string
    let reversed = sentence.chars().rev().collect::<String>();
    println!("{:?}", reversed);
}
