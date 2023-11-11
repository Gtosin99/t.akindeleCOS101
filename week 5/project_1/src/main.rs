//finding the roots of 
//a quadratic equation
use std::io;

fn main() {

    let mut input1 = String::new();
    println!("Enter the value of a");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let mut a:f32 = input1.trim().parse().expect("Not a valid number");

    let mut input2 = String::new();
    println!("Enter the value of b");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let mut b:f32 = input2.trim().parse().expect("Not a valid number");

    let mut input3 = String::new();
    println!("Enter the value of c");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let mut c:f32 = input3.trim().parse().expect("Not a valid number");

    let mut d:f32 = b.powi(2) - 4.0 * a * c ;
    println!("The discriminant is {}",d );


    if d > 0.0{
        let root1:f32 = (-b + d.sqrt()) / (2.0 * a);
        let root2:f32 = (-b - d.sqrt()) / (2.0 * a);
        println!("The roots are distinct \nThey are {} and {}",root1,root2);
    }
    else if d == 0.0 {
        let root1:f32 = (-b + d.sqrt()) / (2.0 * a);
        let root2:f32 = (-b - d.sqrt()) / (2.0 * a);
        println!("There is only one real root");
    }
    else if d < 0.0 {
        println!("There are no real roots");
    }


}
