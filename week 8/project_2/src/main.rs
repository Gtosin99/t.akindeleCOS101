use std::io;

fn main() {
    let mut candidates: Vec<(String, usize)> = Vec::new();

    loop {
        println!("Enter Candidates Names (Enter 'next' when finished):");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Failed to read line");
        let name: String = input1.trim().to_string();

        if name.to_lowercase() == "next" {
            break;
        }

        println!("Enter years of experience (Enter '0' when finished):");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read line");
        let years: usize = input2.trim().parse().expect("Invalid input");

        if years == 0 {
            break;
        }

        candidates.push((name, years));
    }

    println!("Candidates entered: {:?}", candidates);

    // Find the candidate with the highest years of experience
    let max_candidate = candidates.into_iter().max_by(|&(_, years1), &(_, years2)| years1.cmp(&years2));

    match max_candidate {
        Some((name, years)) => {
            println!(
                "The candidate with the highest years of experience is {} with {} years.",
                name, years
            );
        }
        None => {
            println!("No candidates entered.");
        }
    }
}
