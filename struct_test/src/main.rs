#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {

    let scale = 2;

    let rect1 = Rectangle {
        width: dbg!(30*scale),
        height: 50,
    };

    // dbg!(&rect1);
    // println!("rect1 as {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        //area(&rect1)
        rect1.area()
    );
}

fn area(rectangle : &Rectangle) -> u32{
    rectangle.width * rectangle.height
}