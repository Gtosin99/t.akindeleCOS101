use std::io;

fn main() {
    let v = vec!["Office Administrator", "Academic", "Lawyer", "Teacher"];

    let a = vec!["Intern", "-", "Paralegal", "Placement"];

    let b = vec!["Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher"];

    let c = vec!["Senior Administrator", "PhD Candidate", "Associate", "Snr Teacher"];

    let d = vec!["Office Manager", "Post-Doc Researcher", "Senior Associate1-2", "Leading Teacher"];

    let e = vec!["Director", "Senior Lecturer", "Senior Associate3-4", "Deputy Principal"];

    let f = vec!["CEO", "Dean", "Partner", "Principal"];

    let mut input2 = String::new();
    println!("Enter your profession from the list \n{:?}", v);
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let choice: &str = input2.trim();

    let mut input1 = String::new();
    println!("Enter your years of working experience:");
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let years: f32 = input1.trim().parse().expect("Invalid input");

    if choice == v[0] && years >= 1.0 && years <= 2.0 {
        println!("You hold the position APS 1-2 \nYou are an {:?}", a[0]);
    } else if choice == v[0] && years >= 3.0 && years <= 5.0 {
        println!("You hold the position APS 3-5 \n You are an {:?}", b[0]);
    } else if choice == v[0] && years >= 5.0 && years <= 8.0 {
        println!("You hold the position APS 5-8 \n You are a {:?}", c[0]);
    } else if choice == v[0] && years >= 8.0 && years <= 10.0 {
        println!("You hold the position ELS 8-10 \n You are an {:?}", d[0]);
    } else if choice == v[0] && years >= 10.0 && years <= 13.0 {
        println!("You hold the position ELS 10-13 \n You are a {:?}", e[0]);
    } else if choice == v[0] && years > 13.0 {
        println!("You hold the position SES \n You are a {:?}", f[0]);
    } else if choice == v[1] && years >= 1.0 && years <= 2.0 {
        println!("You hold the position APS 1-2 ");
    } else if choice == v[1] && years >= 3.0 && years <= 5.0 {
        println!("You hold the position APS 3-5 \n You are a {:?}", b[1]);
    } else if choice == v[1] && years >= 5.0 && years <= 8.0 {
        println!("You hold the position APS 5-8 \n You are a {:?}", c[1]);
    } else if choice == v[1] && years >= 8.0 && years <= 10.0 {
        println!("You hold the position ELS 8-10 \n You are a {:?}", d[1]);
    } else if choice == v[1] && years >= 10.0 && years <= 13.0 {
        println!("You hold the position ELS 10-13 \n You are a {:?}", e[1]);
    } else if choice == v[1] && years > 13.0 {
        println!("You hold the position SES \n You are a {:?}", f[1]);
    } else if choice == v[2] && years >= 1.0 && years <= 2.0 {
        println!("You hold the position APS 1-2 \n You are a {:?}", a[2]);
    } else if choice == v[2] && years >= 3.0 && years <= 5.0 {
        println!("You hold the position APS 3-5 \n You are a {:?}", b[2]);
    } else if choice == v[2] && years >= 5.0 && years <= 8.0 {
        println!("You hold the position APS 5-8 \n You are an {:?}", c[2]);
    } else if choice == v[2] && years >= 8.0 && years <= 10.0 {
        println!("You hold the position ELS 8-10 \n You are an {:?}", d[2]);
    } else if choice == v[2] && years >= 10.0 && years <= 13.0 {
        println!("You hold the position ELS 10-13 \n You are an {:?}", e[2]);
    } else if choice == v[2] && years > 13.0 {
        println!("You hold the position SES \n You are a {:?}", f[2]);
    } else if choice == v[3] && years >= 1.0 && years <= 2.0 {
        println!("You hold the position APS 1-2 \n You are a {:?}", a[3]);
    } else if choice == v[3] && years >= 3.0 && years <= 5.0 {
        println!("You hold the position APS 3-5 \n You are a {:?}", b[3]);
    } else if choice == v[3] && years >= 5.0 && years <= 8.0 {
        println!("You hold the position APS 5-8 \n You are a {:?}", c[3]);
    } else if choice == v[3] && years >= 8.0 && years <= 10.0 {
        println!("You hold the position ELS 8-10 \n You are a {:?}", d[3]);
    } else if choice == v[3] && years >= 10.0 && years <= 13.0 {
        println!("You hold the position ELS 10-13 \n You are an {:?}", e[3]);
    } else if choice == v[3] && years > 13.0 {
        println!("You hold the position SES \n You are an {:?}", f[3]);
    }
}
