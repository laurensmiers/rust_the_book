use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;
use std::error::Error;

fn open_and_close_with_match(path: &String) -> File {
    // Big block of code with match-expressions for nothing really
    let f = match File::open(path) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    return f;
}

fn open_and_close_without_match(path: &String) -> File {
    let f = File::open(path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(path).unwrap_or_else(|error| {
                panic!("Failed to create file {:?}", error);
            })
        } else {
            panic!("Problem opening file {:?}", error);
        }
    });

    return f;
}

fn read_username_from_file(path: &String) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "hello.txt".to_string();

    println!("Open/create file {}", path);
    open_and_close_with_match(&path);

    // If we get here, the file surely exists
    println!("Remove file {}", path);
    fs::remove_file(&path).expect("Failed to remove file");

    println!("Open/create file {}", path);
    open_and_close_without_match(&path);

    // If we get here, the file surely exists
    println!("Remove file {}", path);
    fs::remove_file(&path).expect("Failed to remove file");

    let username = read_username_from_file(&path).expect("Could not read username from file");

    println!("username: {}", username);

    // Returning error out of main
    let _f = File::open("hello.txt")?;
    fs::remove_file(&path).expect("Failed to remove file");

    Ok(())
}
