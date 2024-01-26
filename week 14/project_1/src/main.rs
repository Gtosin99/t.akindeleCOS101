use std::fs;
use std::io;

enum UserRole {
    Administrator,
    ProjectManager,
    Employee,
    Customer,
    Vendor,
}

fn read_database_structure(role: &UserRole) {
    let file_name = match role {
        UserRole::Administrator => "globacom_dbase.sql",
        UserRole::ProjectManager => "project_tb.sql",
        UserRole::Employee => "staff_tb.sql",
        UserRole::Customer => "customer_table_tb.sql",
        UserRole::Vendor => "dataplan_table_tb.sql",
    };

    if let Ok(content) = fs::read_to_string(file_name) {
        println!("Database Structure for {}: \n{}", file_name, content);
    } else {
        println!("Error: File {} not found.", file_name);
    }
}

fn main() {
    println!("Please enter your role (Administrator, ProjectManager, Employee, Customer, Vendor):");

    let mut user_role_str = String::new();
    io::stdin().read_line(&mut user_role_str).expect("Failed to read line");

    // Trim whitespace and convert input to lowercase for case-insensitive comparison
    let user_role = match user_role_str.trim().to_lowercase().as_str() {
        "administrator" => UserRole::Administrator,
        "projectmanager" => UserRole::ProjectManager,
        "employee" => UserRole::Employee,
        "customer" => UserRole::Customer,
        "vendor" => UserRole::Vendor,
        _ => {
            println!("Invalid role entered.");
            return;
        }
    };

    read_database_structure(&user_role);
}
