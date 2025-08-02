struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
impl User {
    fn new_user(username: String, email: String) -> User {
        User {
            username,
            email,
            sign_in_count: 1,
            active: true,
        }
    }
    fn get_username(&self) -> &String {
        &self.username
    }
    fn get_email(&self) -> &String {
        &self.email
    }
    fn get_sign_in_count(&self) -> u64 {
        self.sign_in_count
    }
    fn get_active(&self) -> bool {
        self.active
    }
}
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x: String = "hello".to_string();
    println!("The value of x is: {}", x);
    let user1 = User::new_user("osmait".to_string(), "osmait@gmail.com".to_string());
    println!("The username of user1 is: {}", user1.get_username());
    println!("The email of user1 is: {}", user1.get_email());
    println!(
        "The sign_in_count of user1 is: {}",
        user1.get_sign_in_count()
    );
    println!("The active of user1 is: {}", user1.get_active());
}
