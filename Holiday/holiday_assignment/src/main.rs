#[allow(dead_code)]
#[derive(Debug)]
struct Info {
    company_name: String,
    company_shares: u64,
    company_liabilities: u64,
    year_founded: u64,
}

impl Info {
    fn leverage(&self) -> f64 {
        ((self.company_shares - self.company_liabilities) as f64 / self.company_shares as f64) * 100.0
    }
}

use std::io::{self, Write};
use std::fs::File;

fn main() {
    let companies = vec![
        Info {
            company_name: "Cadbury Nigeria PLC".to_string(),
            company_shares: 15_000_000,
            company_liabilities: 5_500_000,
            year_founded: 1965,
        },
        Info {
            company_name: "Champion Breweries PLC".to_string(),
            company_shares: 25_000_000,
            company_liabilities: 8_000_000,
            year_founded: 1974,
        },
        Info {
            company_name: "Dangote Sugar Refinery PLC".to_string(),
            company_shares: 18_000_000,
            company_liabilities: 10_000_000,
            year_founded: 1970,
        },
        Info {
            company_name: "Flour Mills Nigeria PLC".to_string(),
            company_shares: 32_000_000,
            company_liabilities: 4_000_000,
            year_founded: 1960,
        },
        Info {
            company_name: "Nestle Nigeria PLC".to_string(),
            company_shares: 8_000_000,
            company_liabilities: 1_500_000,
            year_founded: 1961,
        },
        Info {
            company_name: "Unilever Nigeria PLC".to_string(),
            company_shares: 37_000_000,
            company_liabilities: 11_000_000,
            year_founded: 1923,
        },
        Info {
            company_name: "Honeywell Nigeria PLC".to_string(),
            company_shares: 34_000_000,
            company_liabilities: 9_000_000,
            year_founded: 1906,
        },
        Info {
            company_name: "Nigeria Breweries PLC".to_string(),
            company_shares: 30_000_000,
            company_liabilities: 12_000_000,
            year_founded: 1946,
        },
        
    ];

    let mut file = File::create("company_data.txt").expect("Failed to create file");
     writeln!(
        file,
        "{:<30}\t{:<15}\t{:<20}\t{:<15}\t{:<15}",
        "Company Name",
        "Company Shares",
        "Company Liabilities",
        "Year founded",
        "Leverage%"
    )
    .expect("Failed to write to file");

    for company in &companies {
        writeln!(
            file,
            "{:<30}\t{:<15}\t{:<20}\t{:<15}\t{:.2}",
            company.company_name,
            company.company_shares,
            company.company_liabilities,
            company.year_founded,
            company.leverage()
        )
        .expect("Failed to write to file");
    }

   loop {
        // Prompt for company name input
        println!("Enter Company name:");
        let mut input8 = String::new();
        io::stdin().read_line(&mut input8).expect("Failed to read line");
        let name = input8.trim();

        // Prompt for username input
        println!("Enter Username:");
        let mut input9 = String::new();
        io::stdin().read_line(&mut input9).expect("Failed to read line");
        let username = input9.trim();

        // Check if the username is equal to the first four characters of the company name
        if username == &name[..4].to_lowercase() {
            println!("Continuing...");
        } else {
            println!("Invalid Username \nTry again");
            continue;
        }

        // Prompt for password input
        println!("Input Password:");
        let mut input10 = String::new();
        io::stdin().read_line(&mut input10).expect("Failed to read line");
        let password = input10.trim().to_lowercase();

        // Check password conditions
        if password.len() >= 3
            && password.len() <= 8
            && password.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        {
            println!("Password is valid");

            // Find and display company information
            if let Some(company) = companies.iter().find(|&c| c.company_name == name) {
                println!("Company Information:");
                println!("Name: {}", company.company_name);
                println!("Shares: {}", company.company_shares);
                println!("Liabilities: {}", company.company_liabilities);
                println!("Year Founded: {}", company.year_founded);
                println!("Leverage: {:.2}%", company.leverage());
            } else {
                println!("Company not found in the list.");
            }

            break;
        } else {
            println!("Invalid password.
                    \nPassword should be 3 to 8 characters\n It should be letters from [a-z] \nIt should be numbers[0-9]\nNo Uppercase \nNo special Character \nPlease try again.");
            break;
        }
    }
    println!("Do you wish to  create percentage leverage file (yes/no)");
    let mut input99 = String::new();
    io::stdin().read_line(&mut input99).expect("Failed to read line");
    let response = input99.trim();

    if response.to_lowercase() == "yes" {
for company in &companies{

    if company.company_shares >= 20_000_000 { 
        let mut file2 = File::create(format!("{}_leverage.txt", company.company_name)).expect("Failed to create file");
        writeln!(file2,"{:.2}","Percentage Leverage").expect("Failed to write file");
        println!("File created");
    }
}
    }else {
        println!("No File Created");
    }
        

    
    println!("Do you wish to calculate 5% percentage leverage (yes/no)");
    let mut input55 = String::new();
    io::stdin().read_line(&mut input55).expect("Failed to read line");
    let response2 = input55.trim();

    if response2.to_lowercase() == "yes"{
    for company in &companies{ 
    if company.company_liabilities <= 10_000_000 {
        let solve:f64 = 0.05 * company.leverage();
        println!("5% of percentage leverage used by {} is {:.2}%",company.company_name,solve);

    }
    } 

} else {
    println!("No calculations performed");
}

    }