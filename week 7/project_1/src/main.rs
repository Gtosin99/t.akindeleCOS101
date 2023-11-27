use std::io;

fn main() {
    println!("Select from the following: \n1. Area of trapezium \n2. Area of rhombus \n3. Area of Parallelogram \n4. Area of Cube \n5. Volume of cylinder");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let choice: u32 = input.trim().parse().expect("Invalid input");

    match choice {
        1 => calculate_area_of_trapezium(),
        2 => calculate_area_of_rhombus(),
        3 => calculate_area_of_parallelogram(),
        4 => calculate_area_of_cube(),
        5 => calculate_volume_of_cylinder(),
        _ => println!("Invalid choice. Please select a number from 1 to 5."),
    }
}

fn calculate_area_of_trapezium() {

    println!("Enter the height:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read ");
    let mut height:f64 = input1.trim().parse().unwrap_or(0.0);

    println!("Enter the base 1:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read ");
    let mut base1:f64 = input2.trim().parse().unwrap_or(0.0);

    println!("Enter the base2:");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read");
    let mut base2:f64 = input3.trim().parse().unwrap_or(0.0);

    println!("Calculating area of trapezium...");

    let area_of_trapezium:f64 = (height / 2.0 ) * (base1 + base2);
    println!("The area of the trapezium is {:.2}",area_of_trapezium);

    
}

fn calculate_area_of_rhombus() {

    println!("Enter the value of diagonal 1:");
    let mut input4 = String::new();
    io::stdin().read_line(&mut input4).expect("Failed to read");
    let mut diagonal1:f64 = input4.trim().parse().unwrap_or(0.0);

    println!("Enter value for diagonal 2:"); 
    let mut input5 = String::new();
    io::stdin().read_line(&mut input5).expect("Failed to read");
    let mut diagonal2:f64 = input5.trim().parse().unwrap_or(0.0);
    println!("Calculating area of rhombus...");

    let area_of_rhombus:f64 = 0.5 * diagonal1 * diagonal2;

    println!("The area of the rhombus is {:.2}",area_of_rhombus );    
    
}

fn calculate_area_of_parallelogram() {

    println!("Enter the value of the base");
    let mut input6 = String::new();
    io::stdin().read_line(&mut input6).expect("Failed to read ");
    let mut base:f64 = input6.trim().parse().unwrap_or(0.0);

    println!("Enter the value for altitude");
    let mut input7 = String::new();
    io::stdin().read_line(&mut input7).expect("Failed to read");
    let mut altitude:f64 = input7.trim().parse().unwrap_or(0.0); 
    
    println!("Calculating area of parallelogram...");

    let area_of_parallelogram:f64 = base * altitude;
    println!("The area of the parallelogram is {:.}",area_of_parallelogram );

}

fn calculate_area_of_cube() {

    println!("Enter the length of the side:");

    let mut input8 = String::new();
    io::stdin().read_line(&mut input8).expect("Failed to read");
    let mut length:f64 = input8.trim().parse().unwrap_or(0.0);
    
    println!("Calculating area of cube...");

    let area_of_cube:f64 = 6.0 * length.powi(2);

    println!("The area of the cube is {:.}",area_of_cube ); 
}

fn calculate_volume_of_cylinder() {

    println!("Enter the value of the radius:");
    let mut input9 = String::new();
    io::stdin().read_line(&mut input9).expect("Failed to read");
    let mut radius:f64 = input9.trim().parse().unwrap_or(0.0);

    println!("Enter the value of the height");
    let mut input10 = String::new();
    io::stdin().read_line(&mut input10).expect("Failed to read");
    let mut height1 = input10.trim().parse().unwrap_or(0.0);

    const pi:f64  = 3.142; 
    
    println!("Calculating volume of cylinder...");

    let mut volume_of_cylinder:f64 = pi * radius.powi(2)  * height1;
    println!("The volume of a cylinder is {:.2}",volume_of_cylinder );
}
