#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

#[derive(Debug)]
struct AdminUser {
    user: User,
    has_powers: bool
}

impl AdminUser {
    fn make_power_user(user: User) -> AdminUser {
        AdminUser {
            user: user,
            has_powers: true
        }
    }
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
    let admin_user = build_power_user(
        String::from("admin@test.com"),
        String::from("AdminGod")
    );
    let admin_user2 = AdminUser::make_power_user(user1);
    println!("Username2 is {:?}", user2.username);
    println!("Power User is: {:?}", admin_user2);
}

// Using normal struct
fn build_user(email: String, username: String) -> User {
    User {
        email, // instead of email: email -- if the parameter is same its okay
        username,
        sign_in_count: 5,
        active: true
    }
}

// Using existing struct inside of a new struct
fn build_power_user(email: String, username: String) -> AdminUser {
    AdminUser {
        user: User {
            email, // instead of email: email -- if the parameter is same its okay
            username,
            sign_in_count: 5,
            active: true
        },
        has_powers: true
    }
}