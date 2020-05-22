fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
	Some(i) => Some(i+1),
    }
}

fn main() {
    let x = 5;

    println!("Val: {}, plus one: {}", x, plus_one(Some(x)).unwrap_or(1));

    let x = Some(x);

    println!("Val: {}, plus one: {}", x.unwrap(), plus_one(x).expect("Bad val"));

    let x = None;

    println!("Val of none with default of 1: {}", x.unwrap_or(1));
    println!("Val of none with expect will panic: {}", x.expect("This will be panic because we have a None value"));
}
