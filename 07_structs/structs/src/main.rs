#[derive(Debug)]

struct Person {
    first_name: String,
    last_name: String,
    age: Option<u8>,
}

fn main() {
    let person = Person {
        first_name: "Mario".to_string(),
        last_name: "Lazzari".to_string(),
        age: Some(50),
    };

    println!("{:?}", person);
    println!(
        "My name is {} {} and I'm {:?}.",
        person.first_name,
        person.last_name,
        person.age.unwrap_or(0)
    );

    let person = Person {
        first_name: "Maria".to_string(),
        last_name: "Lazzari".to_string(),
        age: None,
    };

    println!("{:?}", person);
    println!("My name is {} {}", person.first_name, person.last_name);
}
