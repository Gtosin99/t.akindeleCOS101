use std::fs::File;
use std::io::{Write, Result};

fn main() -> Result<()> {
    let lager = vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout = vec!["Legend", "Turbo King", "Williams"];
    let non_alcoholic = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

    let mut file = File::create("data.txt").expect("Failed to create");

    write_drinks(&mut file, &lager)?;
    write_drinks(&mut file, &stout)?;
    write_drinks(&mut file, &non_alcoholic)?;

    println!("\nThe high-quality drinks have been saved in the file");

    Ok(())
}

fn write_drinks(file: &mut File, drinks: &[&str]) -> Result<()> {
    let drinks_str = drinks.join(", ");
    file.write_all(drinks_str.as_bytes())?;
    file.write_all(b"\n")?;
    Ok(())
}
