use std::io;

fn main() {

    println!("Enter Your name:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1);
    let name = input1.trim().to_string();

    println!("Enter email address");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2);
    let email = input2.trim().to_string();

    println!("Enter your department");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3);
    let department = input3.trim().to_string();

    println!("Enter your state of origin");
    let mut input4 = String::new();
    io::stdin().read_line(&mut input4);
    let state_of_origin = input4.trim().to_string();

    println!("Are you a class rep (true/false)");
    let mut input5 = String::new();
    io::stdin().read_line(&mut input5);
    let class_rep : bool = input5.trim().parse().unwrap_or(false);

    println!("Enter Current level");
    let mut input6 = String::new();
    io::stdin().read_line(&mut input6);
    let current_level :u32 = input6.trim().parse().unwrap_or(0);

    println!("Enter your CGPA");
    let mut input7 = String::new();
    io::stdin().read_line(&mut input7);
    let cgpa:f32 = input7.trim().parse().unwrap_or(0.0);

    if class_rep == true && current_level > 100 && cgpa > 4.0{
        println!("{} \n{} \n{} \n{} \nYou are eligible to vote",name, email, department, state_of_origin );
    } else {
        println!("{} \n{} \n{} \n{} \nSorry, You are not eligible to vote",name, email, department, state_of_origin);
    }



}