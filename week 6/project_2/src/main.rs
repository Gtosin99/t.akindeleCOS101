use std::io;

fn main() {
    println!("Enter Your name");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1);
    let name = input1.trim().to_string();

    println!("Enter number of papers published");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2);
    let number_of_papers :u32 = input2.trim().parse().unwrap_or(0);

    let incentive1 :u32 = 500000;
    let incentive2 :u32 = 800000;
    let incentive3 :u32 = 1000000;
    let incentive4 :u32 = 100000;

    if number_of_papers >= 3 && number_of_papers <= 5{
        println!("Your Incentive is N{}",incentive1 );
    } else if number_of_papers >= 5 && number_of_papers < 10{
        println!("Your incentive is N{}",incentive2 );
    } else if number_of_papers >= 10 {
        println!("Your Incentive is N{}",incentive3 );
    } else if number_of_papers < 3 {
        println!("Your Incentive is N{}",incentive4 );
    }
}
