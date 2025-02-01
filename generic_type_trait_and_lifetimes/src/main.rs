use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn main() {
    longest_with_an_announcement(
        &String::from("announcing a new product"),
        &String::from("You'll never guess it!"),
        String::from("fresh off the press!!"),
    );
}
