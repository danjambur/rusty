enum IPAddrKind {
    V4,
    V6,
}

enum IpAddr {
    // IPV4 will always have 4 numbers between 0 and 255.
    // Each variant in an enum can have a different type.
    // Which is what we can't do with a struct
    V4(u8, u8, u8, u8),
    V6(String),
}
fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}
