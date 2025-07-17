fn main() {
    process_numbers(&[1, 2, 3, 4, 5]);

    let chunk = split_string("Hello,World,This,Is,Rust".to_string(), ',', 2);
    println!("The selected chunk is: {}", chunk);
}

fn process_numbers(nums: &[i32]) {
    let mut sum = 0;
    for num in nums {
        sum += num;
    }

    println!("The sum of the numbers is: {}", sum);

    if sum % 2 == 0 {
        println!("The sum is even");
    } else {
        println!("The sum is odd");
    }
}

fn split_string(s: String, delimiter: char, field: usize) -> String {
    let parts: Vec<&str> = s.split(delimiter).collect();
    let res = parts.get(field);

    return res.expect("Something went wrong...").to_string();
}
