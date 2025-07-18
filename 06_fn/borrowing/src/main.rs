fn own_vector(vector: &mut Vec<i32>) {
    vector.push(4);
    println!("Vector inside function: {:?}", vector);
}

fn own_integer(mut value: i32) -> i32 {
    value += 1;
    println!("Integer inside function: {}", value);
    value
}

fn own_string(s: &String) {
    println!("String inside function: {}", s);
}

fn main() {
    let mut my_vector = vec![1, 2, 3];
    let my_int = 10;
    let my_string = String::from("Hello, Rust!");

    own_integer(my_int);
    println!("Integer after function call: {}", my_int);

    // own_string(my_string);
    // println!("String after function call: {}", my_string);
    // This line would cause a
    // compile-time error because `my_string` is moved into `own_string`.

    // borrow string
    own_string(&my_string);

    own_vector(&mut my_vector);
    println!("Vector after function call: {:?}", my_vector);
}
