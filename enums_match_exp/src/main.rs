fn main() {
    #[derive(Debug)]
    enum UsState {
        Alaska,
        Allabama,
    }
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        // first we list the match keyword followed by an experession
        // in this case 'coin' (also the parameter).
        match coin {
            // each one of these below is a match 'arm'
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}", state);
                25
            }
        }
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        // when we pass Some(5) to this method (six below)
        // the match will go through the options;
        match x {
            // it will hit none - if its equal to None
            None => None,
            // If we pass a value, it will identify it.
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", five);
    println!("{:?}", six);
    println!("{:?}", none);

    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Quarter(UsState::Alaska));
}
// match is like a sorting tool.
// if i were to put a bunch of money in a coin machine, and each coin type
// Had a specific slot for that type of coin
// Match is like the holes which let the coin pass when it 'matches' the correct
// type of coin.

// When a match executes, it compares the resulting value against the pattern of each arm, in order.
// If a pattern matches the value, the code assocaited with that pattern is executed.
// If the pattern doesn't match the value, it will continue to the next arm.

// This element of rust is super useful. To use enums, bind and matching the variables with the expected outcomes,
//
