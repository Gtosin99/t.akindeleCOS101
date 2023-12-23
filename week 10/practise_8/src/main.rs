//declare a structure
struct Employee {
    ceo:String,
    company:String,
    age: u32
}

fn main() {
     let emp1 = Employee{
        company:String::from("Enrst.co"),
        ceo :String::from("Ebibiond Jesse"),
        age:56
    };

     let emp2 = Employee{
        company:String::from("Enrst & Young"),
        ceo :String::from("Ebibiond Jessica"),
        age:51
    };
    //pass emp1 and emp2 to display
    display(emp1);
    display(emp2);


}
fn display (emp:Employee){
    println!("Name is :{} company is {} age is 
        {}",emp.ceo, emp.company,emp.age );
}
