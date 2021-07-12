struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

pub fn structs() {
    let user1 = User {
        email: String::from("test@test.com"),
        username: String::from("User1"),
        sign_in_count: 5,
        active: true
    };
    let user2 = build_user(
        String::from("test2@test.com"),
        String::from("User2")
    );

    println!("Username1 is {:?}", user1.username);
    println!("Username2 is {:?}", user2.username);
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // instead of email: email -- if the parameter is same its okay
        username,
        sign_in_count: 5,
        active: true
    }
}