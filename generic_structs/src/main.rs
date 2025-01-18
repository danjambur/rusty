#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 22.5, y: 359.3 };
    let integer_and_float = Point { x: 124, y: 30.23 };

    println!("{:?}, {:?}", both_integer.x, both_integer.y);
    println!("{:?}", both_float);
    println!("{:?}", integer_and_float);

    let p = Point { x: 6, y: 453.4 };

    println!("p.x = {}, py.= {}", p.x(), p.y())
}
