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
}

fn main() {
    // we use the struct template to set rect1
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "the area of the rectangle is {} square pixels.",
        // we pass a referrence to area, not the variable.
        rect1.area()
    );

    if rect1.width() {
        println!("the width is: {}", rect1.width);
    }

    println!("rect is: {:?}", rect1)
}
