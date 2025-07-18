struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }

    fn deactivate(&mut self) {
        self.active = false;
    }
}

struct Point(i32, i32, i32);

fn main() {
    let mut user = User::new(
        String::from("mariolazzari"),
        String::from("mario.lazzari@gmail.com"),
        String::from("https://mariolazzari.it"),
    );
    println!("User {} is active? {}", user.username, user.active);

    user.deactivate();
    println!("User {} is active? {}", user.username, user.active);

    let point = Point(1, 2, 3);
    println!("My point: ({},{},{})", point.0, point.1, point.2);
}
