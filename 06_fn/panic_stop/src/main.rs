fn main() {
    loop_and_panic(vec![1, 2, -3, 4, 5]);
}

fn loop_and_panic(nums: Vec<i32>) {
    loop {
        for n in &nums {
            if *n < 0 {
                panic!("Negative number encountered: {}", n);
            }
        }
    }
}
