fn main() {
    // This is quite similar to a struct,
    // except we don't use the struct keyword,
    //  and all the variants are grouped together under the message Type

    fn call(&self) {
        // whatever
    }

    enum Message {
        Quit,                        // Has no data associated
        Move { x: i32, y: i32 },     // has named fields, like a struct
        Write(String),               // Includes a single String
        ChangeColour(i32, i32, i32), // Includes 3 i32 vals
    }
    // This is much nicer than;

    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    };
    struct WriteMessage(String); // Tuple struct
    struct ChangeColourMessage(i32, i32, i32); // Tuple struct

    let m = Message::Write(String::from("Wassup"));
    m.call()
}
