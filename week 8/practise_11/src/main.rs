fn main() {

// an array of numbers 

let numbers = [1, 2, 4, 5]; 
println!("Original array = {:?}" ,numbers);

// create of 2nd and 3rd 
let slicel =  &numbers [1..3]; 
println!("2nd and 3rd elements sliced = {:?}", slicel);

// omit the start index  
let slice2 = &numbers[..3];

// This was the slice starts from Index O and goes up to index 3 (exclusive) 
println!("Index 0 to index 3 sliceed =  {:?}", slice2);

// omit the end Index 
let slice3 =  &numbers [2..];

// This means the slice starts from Index 2 and goes up to indes 5 (exclusive)

println!("index 2 to index 5 sliced = {:?}",slice3 );

// omit the start index and the end index 
// reference the whole array 
let slice4 = &numbers [..];

// This means the slice starts from Index0 and go up to index 5 
println!("index 0 to index 5 sliced = {:?}" , slice4);

}