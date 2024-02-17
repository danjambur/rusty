fn main() {
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        // first we list the match keyword followed by an experession
        // in this case 'coin' (also the parameter).
        match coin {
            // each one of these below is a match 'arm'
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}
// match is like a sorting tool.
// if i were to put a bunch of money in a coin machine, and each coin type
// Had a specific slot for that type of coin
// Match is like the holes which let the coin pass when it 'matches' the correct
// type of coin.

// When a match executes, it compares the resulting value against the pattern of each arm, in order.
// If a pattern matches the value, the code assocaited with that pattern is executed.
// If the pattern doesn't match the value, it will continue to the next arm.
