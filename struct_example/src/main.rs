#[derive(Debug)]
struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u32,
}

struct Color(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 0,
    }
}

fn main() {
    let user = build_user(String::from("Heisenberg"), String::from("mail@mail.com"));
    println!("Hello to the User: {:?}!", user);

    let user2 = User {
        username: String::from("Jesse"),
        email: String::from("mail@mail.com"),
        ..user
    };

    println!("Hello to the User: {:?}!", user2);

    let color = Color(1, 2, 3);

    println!("Pretty colors! {}, {}, {}!", color.0, color.1, color.2);
}
