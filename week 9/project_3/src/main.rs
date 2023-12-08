fn main() {
    let n = vec![
        "Aigbogun Alamaba Dauda",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let m = vec!["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];

    let g = vec!["South West", "North East", "South South", "South West", "South East"];

    println!("{:<30} | {:<20} | {:<15}", "Name", "Ministry", "Region");
    println!("{:-<30} | {:-<20} | {:-<15}", "", "", ""); // Separator line

    for i in 0..n.len() {
        println!("{:<30} | {:<20} | {:<15}", n[i], m[i], g[i]);
    }
}
