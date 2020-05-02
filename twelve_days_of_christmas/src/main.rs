struct ChristmasDay {
    day: String,
    gift: String,
}

fn main() {
    let christmas_days: [ChristmasDay; 12] = [
        ChristmasDay {
            day: "first".to_string(),
            gift: "a Partridge in a Pear Tree".to_string(),
        },
        ChristmasDay {
            day: "second".to_string(),
            gift: "Two Turtle Doves".to_string(),
        },
        ChristmasDay {
            day: "third".to_string(),
            gift: "Three French Hens".to_string(),
        },
        ChristmasDay {
            day: "fourth".to_string(),
            gift: "Four Calling Birds".to_string(),
        },
        ChristmasDay {
            day: "fifth".to_string(),
            gift: "Five Goldon Rings".to_string(),
        },
        ChristmasDay {
            day: "sixth".to_string(),
            gift: "Six Geese a Laying".to_string(),
        },
        ChristmasDay {
            day: "seventh".to_string(),
            gift: "Sevens Swans a Swimming".to_string(),
        },
        ChristmasDay {
            day: "eigth".to_string(),
            gift: "Eight Maids a Milking".to_string(),
        },
        ChristmasDay {
            day: "ninth".to_string(),
            gift: "Nine Ladies Dancing".to_string(),
        },
        ChristmasDay {
            day: "tenth".to_string(),
            gift: "Ten Lords a Leaping".to_string(),
        },
        ChristmasDay {
            day: "eleventh".to_string(),
            gift: "Eleven Pipers Piping".to_string(),
        },
        ChristmasDay {
            day: "twelfth".to_string(),
            gift: "12 Drummers Drumming".to_string(),
        },
    ];

    for (i, elem) in christmas_days.iter().enumerate() {
        println!();
        println!(
            "On the {} day of Christmas, my true love sent to me:",
            elem.day
        );
        for j in (0..i + 1).rev() {
            if j == 0 && i > 0 {
                print!("And ");
            }
            println!("{}", christmas_days[j].gift);
        }
    }
}
