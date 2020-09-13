use rectangles::Rectangle;

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

    let rect = Rectangle::build_rectangle(30, 50);

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
