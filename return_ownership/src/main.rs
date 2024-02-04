fn main() {
    let s1 = gives_ownership(); // gives ownership moves its return value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2);
    // s2 is moved into takes_and_gives_back, which also
    // moves its return value into s3
    println!("s1 = {s1}, \n s3 = {s3}")
}
// Here, s3 goes out of scope and is dropped.
// s2 was moved, so nothing happens.
// s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownershiip will move its return value into the function that calls it
    let some_string = String::from("yours");

    some_string // some_string is being returned here, and moves out to the calling function
}

// this function takes a string and returns one.

fn takes_and_gives_back(a_string: String) -> String {
    // a string comes into scope
    a_string // a_string is returned and moves out to the calling function
}
