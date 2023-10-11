//references and borrowing
// In rust we are not allowed to modify something we have refernece to. we can only modify by using the references as a mutable one 

fn main() {
    let x = 5;
    let y = &x;
    let mut string_ = String::from("Precious");
    {
        let  z = *y + 1;
        println!("the value is {}", z);
    }
    println!("the value of this block is {} ", x);
    string_addition(& mut string_);
}

fn string_addition(string_1: &mut String){
    string_1.push_str(" , how are you doing?")
}
// you can'tborrow more than one mutable reference at a time.but you can do that for immutable references.
//let r1 = &mut s and let r2 =&mut s will give us an error. 
//strings don't copy they use move. 
// for strings we can use String or the string literal &str.
