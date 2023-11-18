use std::io;

fn main() {
    // Get user input for 'n' and 'm'
    println!("Enter the value of n:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let n: u32 = input1.trim().parse().unwrap_or(0);

     println!("Enter the value of m:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let m: u32 = input2.trim().parse().unwrap_or(0);

    // Display the multiplication table vertically
    println!("Multiplication Table from 1 to {}:", n);

    for i in 1..=n {
        for j in 1..=m {
            let result = i * j;
            println!("{} x {} = {}", i, j, result);
        }
        println!("-----------------");
    }
}
