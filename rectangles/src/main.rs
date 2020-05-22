#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn build_rectangle(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn square(size: u32) -> Rectangle {
        Rectangle::build_rectangle(size, size)
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect)
    );

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    println!(
        "The rectangle {:#?} can hold itself obviously: {}.",
        rect,
        rect.can_hold(&rect),
    );

    let square = Rectangle::square(3);

    println!(
        "The area of the square rectangle {:#?} is {} square pixels.",
        square,
        square.area()
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
