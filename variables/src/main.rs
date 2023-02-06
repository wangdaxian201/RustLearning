use std::io;

fn main() {
    let mut x = 5;
    println!("This value of x is {}", x);
    x = 6;
    println!("This value of x is {}", x);
    const THREE_HOUR_IN_SECONDS: u32 = 3 * 60 * 60;
    println!("this value is {}", THREE_HOUR_IN_SECONDS);
    let spaces = "    ";
    let spaces_len = spaces.len();
    println!("this spaces value is {}", spaces_len);

    let guess: u32 = "42".parse().expect("Not a number");
    println!("this value of is {}", guess);
    celsius_to_fahrenheit(11.0); // 摄氏度转华氏度
    fahrenheit_to_celsius(93.0); // 华氏度转摄氏度

    let num: u32 = fibonacci(11);
    println!("fibonacci: this value of is {num}");
    array_type();
    print_twelve_days_of_christmas();
}

fn array_type() {
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}

fn print_twelve_days_of_christmas() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "a Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Golden Rings",
        "Six Geese a Laying",
        "Seven Swans a Swimming",
        "Eight Maids a Milking",
        "Nine Ladies Dancing",
        "Ten Lords a Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming",
    ];

    for (i, day) in days.iter().enumerate() {
        println!("On the {} day of Christmas, my true love gave to me:", day);
        for j in (0..=i).rev() {
            if j == 0 && i != 0 {
                println!("And {}.", gifts[j]);
            } else {
                println!("{}.", gifts[j]);
            }
        }
        println!("");
    }
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    return celsius * 9.0 / 5.0 + 32.0;
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    return (fahrenheit - 32.0) * 5.0 / 9.0;
}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
