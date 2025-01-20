#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

// Simple implementation of methods for returning the value of the 'Point' that it is called against
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

// An example of a method that allows for a method to only be accessible on a Point with a CONCRETE
// Type of f64 for example.
impl Point<f64, f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
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
    println!("p.x = {}, py.= {}", p.x(), p.y());

    let point_f32_origin = Point { x: 1.2, y: 1.3 };

    println!("origin = {}", point_f32_origin.distance_from_origin())
}
