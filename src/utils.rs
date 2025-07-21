use std::io::{self, Write};
pub fn get_user_choice(min: u32, max: u32) -> Result<u32, String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let choice: u32 = input.trim().parse().unwrap();
    if choice < min || choice > max {
        return Err(format!(
            "Invalid choice. Please enter a number between {} and {}",
            min, max
        ));
    }
    Ok(choice)
}


pub fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn get_user_input_number(prompt: &str) -> u32 {
    let input = get_user_input(prompt);
    if let Ok(num) = input.trim().parse::<u32>() {
        num
    } else {
        println!("Invalid input. Please enter a integer number.");
        get_user_input_number(prompt)
    }
}

pub fn get_user_input_amount(prompt: &str) -> f64 {
    let input = get_user_input(prompt);
    if let Ok(num) = input.trim().parse::<f64>() {
        num
    } else {
        println!("Invalid input. Please enter a float number.");
        get_user_input_amount(prompt)
    }
}


pub fn get_yes_no_input(prompt: &str) -> Result<bool, String> {
    print!("{} (y/n): ", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match input.trim().to_lowercase().as_str() {
                "y" | "yes" => Ok(true),
                "n" | "no" => Ok(false),
                _ => Err("Please enter 'y' or 'n'".to_string()),
            }
        }
        Err(_) => Err("Failed to read input".to_string()),
    }
}