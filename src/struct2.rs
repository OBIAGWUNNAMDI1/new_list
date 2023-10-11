//using struct to mimic a file and read its content. 
#[derive(Debug)]
struct File {
    name : String,
    data: Vec<u8>,
}
 fn open (f:&mut File) ->bool {
    true
 } 
fn close (f:&mut File) -> bool{
    true
}
fn read( f:&File , save_to:&mut Vec<u8>) ->usize{
    let mut temp = f.data.clone();
    let read_len = temp.len();
    save_to.reserve(read_len);
    save_to.append(&mut temp);
    read_len

}

fn main () {
    let mut f2 = File{
        name: String::from("text.txt"),
        data : vec![67,79,123, 56,90, 67,45],
    };
    let mut buffer : Vec<u8> = vec![];
    open(&mut f2);
    let f2_len = read (&f2, &mut buffer);
    close (&mut f2);

    let text = String::from_utf8_lossy(&buffer);
    println!("{:?}", f2 );
    println!("{} in {} bytes long", &f2.name, f2_len );
    println!("{}", text )
}