// °C = (°F − 32) × 5/9
// °F = °C × 9/5 + 32

use std::io;

fn main() {
    let fahrenheit = take_input();
    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{fahrenheit}°F is {celsius:.2}°C");
}

fn take_input() -> f64 {
    loop {
        println!("Please input the temperature in Fahrenheit: ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<f64>() {
            Ok(value) => return value,
            Err(_) => println!("That's not a valid number, please try again."),
        }
    }
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}