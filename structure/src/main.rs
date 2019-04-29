struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(String::from("aee"), String::from("aee@code.com"));

    let user2 = User {
        username: String::from("user2"),
        email: String::from("user2@code.com"),
        ..user1
    };

    println!("{}, {}", user1.username, user1.email);
    println!("{}, {}", user2.username, user2.email);
}
