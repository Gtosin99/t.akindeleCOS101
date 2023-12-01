fn main(){ 

//Name vector

let name = vec!["Mary", "Sam", "Sally", "Greg", "Ade", "Mark", "June"];


// Age vector 
let age = vec![16,17,19,22,20,21,18,231];

print!("\nAge allocation: \n");

//Ιοορ tο iterate elements in vector 
for i in 0..age.len()
{
// iterating through elements in the vector 
print!("{} is {} is () years old\n",name[i],age[i]);
}
}