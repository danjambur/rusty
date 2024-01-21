fn main() {
    let x: u8 = 255;
    let y: u8 = 1;

    // wrapping_add: This is like a clock. If it's 11 o'clock and you add 2 hours, it becomes 1 o'clock, not 13 o'clock.
    // So, if you have 255 and add 1, it goes back to 0.
    let result = x.wrapping_add(y);
    println!("wrapping_add: {}", result); // prints "0"

    // checked_add: This is like a guard checking if you can add more.
    // If you have 255 and try to add 1, the guard says "No, you can't do that!"
    // and gives back None to indicate that it couldn't add the numbers without going over 255.
    let result = x.checked_add(y);
    match result {
        Some(v) => println!("checked_add: {}", v),
        None => println!("checked_add: overflow"),
    }

    // overflowing_add: This is like a friend who tries to add the numbers anyway, but tells you if it didn't work out.
    // If you have 255 and add 1, your friend gives you 0, but also tells you "Hey, that addition overflowed."
    let (result, overflowed) = x.overflowing_add(y);
    if overflowed {
        println!("overflowing_add: {} with overflow", result);
    } else {
        println!("overflowing_add: {}", result);
    }

    // saturating_add: This is like a bucket that can hold up to 255 units of water.
    // If you try to add more water when it's already full, it just stays at 255.
    // It doesn't overflow and spill water everywhere, it just can't hold more than 255.
    // So, if you have 255 and add 1, you still have 255.
    let result = x.saturating_add(y);
    println!("saturating_add: {}", result); // prints "255"
}
