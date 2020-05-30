fn vector_tests() {
    let mut v: Vec<i32> = Vec::new();
    let mut c = vec![1, 2, 3];

    println!("Elements in vector:");
    for el in v.iter() {
        println!("{}", el);
    }

    println!("Elements in vector:");
    for el in c.iter() {
        println!("{}", el);
    }

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    c.push(4);

    println!("Elements in vector after update:");
    for el in &v {
        println!("{}", el);
    }

    println!("Elements in vector after update:");
    for el in c.iter() {
        // Just another way for iterating over a vector
        println!("{}", el);
    }

    let t: &i32 = &v[2];
    println!("The third element is {}", t);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("ERROR: no third element"),
    }

    // Following code panics because we go out-of-bounds
    // let _does_not_exist = &v[100];

    // Folling code does not panic but returns None
    let does_not_exist = v.get(100);

    match does_not_exist {
        Some(temp) => println!("ERROR: None should have been returned {}", temp),
        None => println!("Out-of-bounds: get returns None"),
    }

    // vector that holds multiple types using enums
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("----------");
    println!("Vec with different types:");
    for v in &row {
        match v {
            SpreadsheetCell::Int(i) => println!("Integer: {}", i),
            SpreadsheetCell::Float(f) => println!("Float: {}", f),
            SpreadsheetCell::Text(s) => println!("String: {}", s),
        }
    }
}

fn main() {
    vector_tests();
}
