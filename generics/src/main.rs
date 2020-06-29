struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    // Different types of Point's
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f32, f32> {
    // Specific types of Point's
    fn distance_from_origin(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let p4 = Point { x: 1.0, y: 1.0 };

    println!("distance: {}", p4.distance_from_origin());
}
