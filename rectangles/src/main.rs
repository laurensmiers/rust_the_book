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

    // convenience function to show use of Result<T,E> in tests
    fn return_result(&self, is_succes: bool) -> Result<(), ()> {
        if is_succes {
            Ok(())
        } else {
            Err(())
        }
    }

    // convenience function to show panic used in tests
    fn return_panic(&self, is_panic: bool) {
       if is_panic {
       	  panic!("HEEELP");
       }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square() {
        let sq = Rectangle::square(4);

        assert_eq!(sq.width, sq.height);
    }

    #[test]
    fn area() {
        let width = 4;
        let height = 5;
        let rect = Rectangle::build_rectangle(width, height);

        assert_eq!(rect.area(), width * height);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let smaller = Rectangle::build_rectangle(3, 4);
        let larger = Rectangle::build_rectangle(5, 6);

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let smaller = Rectangle::build_rectangle(3, 4);
        let larger = Rectangle::build_rectangle(5, 6);

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn result_success() -> Result<(), ()> {
        let rect = Rectangle::build_rectangle(3, 4);

        rect.return_result(true)?;

        Ok(())
    }

    #[test]
    fn result_will_fail_with_result() -> Result<(), ()> {
        let rect = Rectangle::build_rectangle(3, 4);

        rect.return_result(false)?;

        Ok(())
    }

    #[test]
    #[should_panic(expected = "HEEELP")]
    fn result_panic() {
        let rect = Rectangle::build_rectangle(3, 4);

        rect.return_panic(true);
    }

    #[test]
    #[should_panic]
    fn result_will_not_panic_and_fail() {
        let rect = Rectangle::build_rectangle(3, 4);

        rect.return_panic(false);
    }

    #[test]
    #[should_panic(expected = "HEEEEEEEEEEEEEEEEEEEELP")]
    fn result_will_panic_but_with_wrong_message_and_fail() {
        let rect = Rectangle::build_rectangle(3, 4);

        rect.return_panic(false);
    }

}
