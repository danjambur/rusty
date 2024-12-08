fn main() {
    // This is a type asserted Vector. Because we're starting without any values, Rust's compiler
    //  needs to know what type of values we're going to put in it.
    let _v: Vec<i32> = Vec::new();
    // This is a type inferred Vector. Because we're starting with values, Rust's compiler can
    //  infer the type of values we're going to put in it.
    let _v = vec![1, 2, 3];

    // This is a mutable Vector. We can add values to it.
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // This is a Vector with a value at index 2. We can access it with the index operator.
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // This is a Vector with a value at index 2. We can access it with the get method.
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // iterate over the values in a vector
    for i in &v {
        println!("{}", i);
    }

    // we can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements.
    for i in &mut v {
        // To change the value that the mutable reference refers to,
        // we have to use the * dreference operator to get to the value in i before we can use the += operator.
        *i += 50;
    }

    // Vectors can only store values of the same type. If we try to store a value of a different type, we'll get a compiler error.
    // let _v = vec![1, "two", 3];
    // error[E0308]: mismatched types

    // Vectors can store values of any type. If we want to store values of different types, we can use an enum.
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(06.17),
    ];
} // Vectors are cleaned up when they go out of scope, just like any other variable in Rust.
