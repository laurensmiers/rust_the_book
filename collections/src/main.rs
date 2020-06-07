use std::collections::HashMap;

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

fn string_tests() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // Format does not take ownership, so it copies the s1/2/3 into a new String and stores it in s
    let s = format!("{}-{}-{}", s1, s2, s3);

    println!(
        "All references still valid after format: {}  : {} + {} + {}",
        s, s1, s2, s3
    );

    // Adding strings means taking ownership of the first one, so s1 is no longer valid after this
    // This also means that it copies more efficiently than format!(), since we don't make a whole new string this time
    let s = s1 + "-" + &s2 + "-" + &s3;

    println!("{}", s);

    println!("s2 and s3 are still valid: {} - {}", s2, s3);

    // s1 is no longer valid, value was moved in addition above
    // println!("s1 is no longer valid: {}", s1);

    // String indexing is not supported
    // let first_char = &s[0];

    // Iterating over strings

    // Will print out 6 chars (even though you would expect 4, see diacritics)
    println!("Print chars:");
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // Will print out 18 bytes (byte != char, valid Unicode values can be > 1 byte)
    println!("Print bytes:");
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}

fn create_hash_map_with_zip() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut _scores: HashMap<_, _> =
    // let mut scores: HashMap<String, i32> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
}

fn hash_map_insertion() {
    // Inserting can move ownership

    let field_name = String::from("Score");
    let field_value = 10;

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // Types that don't implement Copy trait will move ownership, f.e. String
    // println!("field_name is no longer valid: {} - {}", field_name, field_value);
    println!("field_value is still valid: {}", field_value);
}

fn hash_map_tests() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 10);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).expect("No team with that name!"); // get() returns Option<&V>
    println!("score of {} team is {}", team_name, score);

    println!("Original scores");
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    println!("Updating score of {}", team_name);
    scores.insert(team_name, 20);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    println!("Inserting yellow and NOT updating Blue");
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    println!("Update a value based on the old value");
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    create_hash_map_with_zip();
    hash_map_insertion();
}

fn list_of_integers_ex() {
    let mut int_vec = vec![2, 2, 4, 3, 2, 1, 6];

    println!("Vec: {:?}", int_vec);

    let mut mean = 0;
    for i in &int_vec {
        mean += i;
    }
    mean /= int_vec.len();
    println!("Mean value: {}", mean);

    println!("Sort the list");
    int_vec.sort();

    println!("{:?}", int_vec);

    let median = int_vec.get(int_vec.len()/2).expect("Bad index for median");
    println!("median: {}", median);

    println!("Creating modes...");
    let mut map = HashMap::new();
    for i in &int_vec {
        let count = map.entry(i).or_insert(0);
	*count += 1;
    }

    let mut max_mode = (0, 0);
    for (number, mode) in &map {
        if max_mode.1 < *mode {
	    max_mode.0 = **number;
	    max_mode.1 = *mode;
	}
    }

    println!("max_mode: number: {}, mode: {}", max_mode.0, max_mode.1);
}

fn convert_to_pig_latin(word: &str) -> String {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let first_letter = word.chars().nth(0).expect("Bad word provided");
    let first_letter_is_vowel = vowels.contains(&first_letter);

    if first_letter_is_vowel {
        return format!("{}-hay", word);
    } else {
        return format!("{}-{}ay", &word.clone()[1..], first_letter);
    }
}

fn pig_latin_ex(text: &str) {
    let mut piglatin_text = String::new();

    for word in text.split_whitespace() {
        piglatin_text.push_str(&(convert_to_pig_latin(word) + " "));
    }

    piglatin_text.pop();

    println!("Normal   text: '{}'", text);
    println!("Piglatin text: '{}'", piglatin_text);
}


fn main() {
    vector_tests();

    string_tests();

    hash_map_tests();

    list_of_integers_ex();

    pig_latin_ex("first apple");
}
