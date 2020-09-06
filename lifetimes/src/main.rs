fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// No need to specify lifetime of second param since it is not used in the function
fn longest_always_first_param<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

#[derive(Debug)]
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    // Because of the lifetime elision rules, no need to add lifetimes here, compiler is smart enough to figure it out
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let _s: &'static str = "I have a static lifetime and will live until the program ends.";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    println!(
        "The longest string is maybe {}?",
        longest_always_first_param(string1.as_str(), string2)
    );

    let novel = String::from("Call me Ishmael. Some years ago ...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = Excerpt {
        part: first_sentence,
    };

    println!("excerpt: {:?}", i);
    i.announce_and_return_part("hello");
}
