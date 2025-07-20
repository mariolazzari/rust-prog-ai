fn main() {
    ownership();
    modifiable();

    let vec = vec![1, 2, 3, 4, 5];

    // get specific index
    let third = vec[2];
    println!("3rd value: {}", third);

    // get last index
    let last = vec.last().unwrap();
    println!("Last value: {}", last);

    // get 1st value with match
    match vec.first() {
        Some(val) => println!("1st value: {}", val),
        None => println!("Vector is empty"),
    };

    // usize
    get_item(3);

    // adding elements
    let mut vec = vec![1, 2, 3];
    vec.push(4);
    println!("{:?}", vec);

    // entends
    let vec2 = [5, 6];
    vec.extend(vec2);
    println!("{:?}", vec);

    // appends
    let mut other_nums = vec![7, 8];
    vec.append(&mut other_nums);
    println!("{:?}", vec);

    // insert
    vec.insert(0, 0);
    println!("{:?}", vec);
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

fn get_item(idx: usize) {
    let vec = vec![1, 2, 3, 4, 5];
    let val = vec[idx];

    println!("Value: {}", val);
}
