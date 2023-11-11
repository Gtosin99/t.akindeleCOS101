use std::io;

fn main() {
    // Input: Experience and Age
    println!("Enter employee experience (experienced or inexperienced):");
    let mut experience = String::new();
    io::stdin().read_line(&mut experience).expect("Failed to read line");
    let experience = experience.trim();

    println!("Enter employee age:");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read line");
    let age: u32 = age.trim().parse().expect("Please enter a valid age");

    // Determine Annual Incentive
    let annual_incentive = match (experience, age) {
        ("experienced", a) if a >= 40 => 1560000,
        ("experienced", a) if a >= 30 && a < 40 => 1480000,
        ("experienced", a) if a < 28 => 1300000,
        ("inexperienced", _) => 100000,
        _ => {
            println!("Invalid input");
            return;
        }
    };

    // Output: Annual Incentive
    println!("The annual incentive for the employee is: N{}", annual_incentive);
}
