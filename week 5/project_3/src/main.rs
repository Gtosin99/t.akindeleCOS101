use std::io;

fn main() {
    // Display the menu
    println!("Menu:");
    println!("P = Poundo Yam/Edinkaiko Soup - N3,200");
    println!("F = Fried Rice & Chicken - N3,000");
    println!("A = Amala & Ewedu Soup - N2,500");
    println!("E = Eba & Egusi Soup - N2,000");
    println!("W = White Rice & Stew - N2,500");

    // Input food type and quantity
    println!("Enter the food type (P/F/A/E/W): ");
    let food_type = read_input_char();

    println!("Enter the quantity: ");
    let quantity: f32 = read_input() as f32;

    // Calculate total charges
    let total_charges = match food_type {
        'P' => 3200.0 * quantity,
        'F' => 3000.0 * quantity,
        'A' => 2500.0 * quantity,
        'E' => 2000.0 * quantity,
        'W' => 2500.0 * quantity,
        _ => {
            println!("Invalid food type. Please select a valid option.");
            return;
        }
    };

    // Apply discount if total charges are greater than N10,000
    let discount: f32 = if total_charges > 10000.0 {
        0.05 * total_charges
    } else {
        0.0
    };

    // Calculate the final total after applying the discount
    let final_total = total_charges - discount;

    // Display the total charges
    println!("Total Charges: N{}", final_total);
}

// Function to read input from the user
fn read_input_char() -> char {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().chars().next().expect("Please enter a valid character")
}

fn read_input() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please enter a valid number")
}
