// much like generic types, generic (named) lifetime parameters are short
// and don't provide much context.
// they just need to be preceded by an apostrophe: ',
// as shown below.

// This does not change any lifetimes of the values passed to this method!
// It just tells the borrow checker that it will get the concrete lifetime
// that is equal to the smaller lifetimes of x and y.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let test_long_string = String::from("Wassup");
    let test_long_string2 = String::from("wassssssaaaaap");

    println!(
        "the longest string is: {}",
        longest(&test_long_string, &test_long_string2)
    );

    let result;
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    {
        result = longest(&string1, &string2);
    }
    println!("The longest string is '{result}'");
}
