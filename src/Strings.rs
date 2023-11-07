fn main () {
    //creating strings 
    let mut word = String::from("Hello how are you doing?");
    let worded = "How are you doing ?";
    let mut word2 = worded.to_string();
    //updating strings 
    word.push_str("Who is going home today");
    word2.push_str("I am doing great thanks");
    println!("The sentence is {}",word2 );
    println!("The first statement is {}", word );

    let  new_word = word + &word2;
    println!("The long sentence is {}", new_word );

    //iterating over strings
    for w in new_word.chars(){
        println!("{w}", );
    }
    for x in new_word.bytes(){
        println!("{x}", );
    }

}