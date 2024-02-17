fn main() {
    // T is a 'Generic'. Its type can literally be anything.
    // This is just an example of what is already in the standard library.
    // It's not really required for the following code
    enum Option<T> {
        None,
        Some(T),
    }

    // its now an i32
    let some_number = Some(5);
    // its now a char
    let some_char = Some('e');
    // its now a string
    let some_string = Some(String::from("Hello"));

    let nothing: Option<String> = None;
    // With Option, we need to define the type when we call 'Some' for example.
}
