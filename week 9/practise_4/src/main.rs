use std::fs::OpenOptions;
use std::io::Write;

fn main() {

    let mut file = OpenOptions::new().append(true).open("data.txt").expect("cannot open file");
    file.write_all("\nHello Class".as_bytes()).expect("write failed");
    file.write_all("\nThis is the appendage to the document.".as_bytes())
        .expect("write failed");
        println!("file append success");
}




use std::io::Write;

let lager = vec!["33 Export", "Desperados", "Goldberg","Gulder","Heineken","Star"];
let stout = vec!["Legend", "Turbo King", "Williams"];
let non-alcoholic = vec!["Maltina","Amstel Malta","Malta Gold","Fayrouz"];

let mut file = std::fs::File::create("data.txt").expect("Failed to create");
file.write_all(lager.as_bytes()).expect("Write failed");
file.write_all(stout.as_bytes()).expect("Write failed");
println!("\nThe high quality drinks have been saved in the file");







