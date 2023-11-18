use std::io;

fn main() {
    // Read the distance in miles
    println!("Enter the distance in miles:");
    let mut distance = String::new();
    io::stdin()
        .read_line(&mut distance)
        .expect("Failed to read line");

    // Convert the input to a floating-point number
    let distance: f64 = distance.trim().parse().expect("Invalid input. Please enter a number.");

    // Read the time in hours
    println!("Enter the time in hours:");
    let mut time = String::new();
    io::stdin()
        .read_line(&mut time)
        .expect("Failed to read line");

    // Convert the input to a floating-point number
    let time: f64 = time.trim().parse().expect("Invalid input. Please enter a number.");

    // Calculate the speed in km/hr (1 mile = 1.60934 kilometers)
    let speed_kmph = distance * 1.60934 / time;

    // Display the result
    println!("The speed of the car is {:.3} km/hr", speed_kmph);
}
