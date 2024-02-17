// We create a struct for the rectangle
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// area will expect a referenced rectangle struct and returns width * height
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // square returns `Self` - it is an associated function that most languages would use with new
    // the `Self` keywords are aliases for the targeted 'impl' block. In this case it refers to rectangle.
    // This could also be implemented as 'Rectangle'.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // we use the struct template to set rect1
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 30,
    };

    let rect3 = Rectangle::square(200);

    println!("squared {:?}", rect3);

    println!(
        "the area of the rectangle is {} square pixels.",
        // we pass a referrence to area, not the variable.
        rect1.area()
    );

    if rect1.width() {
        println!("the width is: {}", rect1.width);
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    println!("rect is: {:?}", rect1)
}
