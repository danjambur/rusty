fn main() {
    // this string is fixed in length. It will be added to the stack, as at compile time, its size is known.
    // this s value is valid in the scope of main. It can be used and accepted. It will no longer be in scope after the last bracket of main
    let _global_s = "hello";
    {
        // the scope has been set by the curly braces wrapping this 's' string. It can be used after initialisation.
        let _scoped_s = "hello";
    }
    // _scoped_s is no longer valid
    // _global_s is still now valid

    // this string length could be unknown at compile time.
    // therefore, its added to the heap instead of the stack.

    let mut hello = String::from("hello");
    hello.push_str(" world!");
    println!("{hello}");

    let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{s1}"); - this won't work. to get this to work - you need to;
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    let s = String::from("hello"); // s is now in scope!

    takes_ownership(s); // s's value moves into the function..
                        // from now on, it is no longer valid

    let x = 5; // x is now in scope

    makes_copy(x); // x is now in the scope of the function, but i32 is a copy, so you can still use it!

    let y = x;
    println!("x = {x}, y = {y}");
    // this works! because x is an integer.
    // integer sizes are always known, so therefore will always live on the stack.
    // you don't need to explicitly clone it.
}

// here, x goes out of scope. Then s.
// However - the value of s has been moved.

fn takes_ownership(some_string: String) {
    // some_string is in scope
    println!("{some_string}")
} // here, some_string is removed from scope and 'drop' is called. The backing memory is now free!

fn makes_copy(some_integer: i32) {
    //some_integer is in scope
    println!("{some_integer}")
} // Here, some_integer is also removed from scope
