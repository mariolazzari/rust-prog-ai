fn main() {
    ownership();
    modifiable();
}

fn ownership() {
    let numbers = vec![1, 2, 3];
    let slice = &numbers[..];
    println!("slice {:?}", slice);
}

fn modifiable() {
    let mut numbers = vec![1, 2, 3];
    let slice = &mut numbers[..];
    slice[0] = 10;
    println!("slice {:?}", slice);
}
