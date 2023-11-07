// common collections in rust includes . Vector, String and HashMap
//let V:Vec<I32> ==Vec::new();
fn main () {
let mut V= vec![24,35,65];
V.push(40);
V.push(50);
println!("The number in the vector is {}", V.len());
let third_value: &i32 = &V[2];
println!("The third value is {third_value}");
for I in &mut V{

    *I += 50; // * is a dereference operator to get the value in I
    println!("The values are {I}");
}
}