use std::io;

const MAX_SIBLINGS: usize = 10;

fn main() {
    println!("How many siblings do you have?");
    let num_siblings: usize = read_line("Number of siblings").trim().parse().expect("Invalid input");

    if num_siblings > MAX_SIBLINGS {
        println!("Sorry, this program supports up to {} siblings.", MAX_SIBLINGS);
        return;
    }

    let mut sibling_data: Vec<(String, u32, String, Option<String>, Option<bool>, Option<String>)> = Vec::new();

    for i in 0..num_siblings {
        println!("Enter details for Sibling {}:", i + 1);

        let name = read_line("Name");
        let age: u32 = read_line("Age").trim().parse().expect("Invalid input");
        let marital_status = read_line("Marital Status (single/married)");

        let (education_details, offspring, city) = if age > 18 {
            if marital_status.to_lowercase() == "single" {
                let status = read_line("Is the sibling a student or a worker").to_lowercase();
                (
                    Some(if status == "student" {
                        format!(
                            "University: {}\nCourse of Study: {}",
                            read_line("University"),
                            read_line("Course of Study")
                        )
                    } else {
                        "Worker".to_string()
                    }),
                    None,
                    None,
                )
            } else {
                (
                    None,
                    Some(read_line("Does the sibling have offspring? (yes/no)").to_lowercase() == "yes"),
                    Some(read_line("What city does the family live in")),
                )
            }
        } else {
            (
                Some(if read_line("Did the sibling write WAEC? (yes/no)").to_lowercase() == "yes" {
                    read_line("Secondary School Attended")
                } else {
                    read_line("Current Class Level")
                }),
                None,
                None,
            )
        };

        sibling_data.push((name, age, marital_status, education_details, offspring, city));

        println!("-----------------------------");
    }

    println!("Sibling Details:");
    for (i, sibling) in sibling_data.iter().enumerate() {
        println!("Sibling {}: {:?}", i + 1, sibling);
    }
}

fn read_line(prompt: &str) -> String {
    println!("Enter {}: ", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}
