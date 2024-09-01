use dialoguer::{Select, Input};
use colored::*;

fn main() {
    println!("{}", "🌡️  Welcome to the temperature converter! 🌡️".bright_green());

    loop {
        let options = ["Celsius to Fahrenheit 🔥", "Fahrenheit to Celsius ❄️", "Exit 👋"];
        let selection = if let Ok(input) = Select::new()
            .with_prompt("Select conversion direction")
            .items(&options)
            .default(0)
            .interact()
        {
            input
        } else {
            println!("Failed to get selection. Exiting.");
            return;
        };

        if selection == 2 {
            println!("{}", "Thanks for using the temperature converter! 👋".bright_blue());
            break;
        }

        let temperature: f64 = if let Ok(value) = Input::new()
            .with_prompt("Enter the temperature value")
            .interact_text()
        {
            value
        } else {
            println!("Failed to get temperature input. Exiting.");
            return;
        };

        println!("You entered: {}°{}",
            temperature.to_string().yellow(),
            if selection == 0 { "C".blue() } else { "F".red() });

        let result = match selection {
            0 => celsius_to_fahrenheit(temperature),
            1 => fahrenheit_to_celsius(temperature),
            _ => panic!("Invalid selection"),
        };

        println!("The converted temperature is: {}°{}",
            format!("{:.2}", result).yellow(),
            if selection == 0 { "F".red() } else { "C".blue() });

        println!();
    }
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 1.8) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}