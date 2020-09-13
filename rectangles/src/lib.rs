#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn build_rectangle(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    pub fn square(size: u32) -> Rectangle {
        Rectangle::build_rectangle(size, size)
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
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
    #[should_panic(expected = "HEEELP")]
    fn result_panic() {
        let rect = Rectangle::build_rectangle(3, 4);

        rect.return_panic(true);
    }

    #[test]
    #[ignore]
    fn result_will_fail_with_result() -> Result<(), ()> {
        let rect = Rectangle::build_rectangle(3, 4);

        rect.return_result(false)?;

        Ok(())
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn result_will_not_panic_and_fail() {
        let rect = Rectangle::build_rectangle(3, 4);

        rect.return_panic(false);
    }

    #[test]
    #[ignore]
    #[should_panic(expected = "HEEEEEEEEEEEEEEEEEEEELP")]
    fn result_will_panic_but_with_wrong_message_and_fail() {
        let rect = Rectangle::build_rectangle(3, 4);

        rect.return_panic(false);
    }
}
