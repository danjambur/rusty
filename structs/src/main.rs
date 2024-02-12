// a struct is a 'structure'.. just like an object!
// Debug allows us to print using {:?}
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        // you can use shorthand assignment instead of username: username
        username,
        email,
        sign_in_count: 1,
    }
}
fn main() {
    let user1: User = build_user(
        String::from("someone@example.com"),
        String::from("danjambur"),
    );

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("anotherone@example.com"),
        sign_in_count: user1.sign_in_count,
        // another way of doing this:
        // ..user1
    };

    println!("the user: {:?}", user2);
}
