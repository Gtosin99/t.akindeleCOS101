use std::io; // Add this line to import the io module

fn main() {
    let v = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H']; // Assuming 'v' is your vector

    let mut input1 = String::new();

    println!("Enter an index value between (0 - 7)");

    io::stdin().read_line(&mut input1).expect("Failed to read input");

    // Index is the non-negative value which is smaller than the size of the Vector
    let index: usize = input1.trim().parse().expect("Invalid Input");


        // Getting value at the given index value
        let ch: char = v[index];

        println!("{} is the character for index [{}]", ch, index);
    
    
}
