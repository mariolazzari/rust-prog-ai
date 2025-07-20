fn divide(x: i32, y: i32) -> Option<i32> {
    if y == 0 { None } else { Some(x / y) }
}

fn main() {
    let a = 10;
    let b = 2;
    let c = 0;

    let res = divide(a, b);
    match res {
        Some(v) => println!("Result: {}", v),
        None => println!("Divide by 0"),
    }

    // dangerous
    println!("{}", res.unwrap());

    let res = divide(a, c);
    match res {
        Some(v) => println!("Result: {}", v),
        None => println!("Divide by 0"),
    }
}
