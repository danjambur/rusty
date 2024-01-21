fn main() {
    let x = 5;
    // because x is already 5, and we are 'shadowing' it, we can declare a new x, and make it 6
    let x = x + 1;
    {
        let x = x * 2;
        // this will print 12 - the value of x above is currently 6, so we double that - to equal 12.
        println!("The value of x in the inner scope is: {x}");
    }
    // this will print 6 - the value of x above is currently 6, and we aren't changing it.
    println!("The value of x in the outer scope is: {x}");
}
