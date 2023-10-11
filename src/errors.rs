//rust uses the Result<T,E > for recoverable errors and the panic! for unrecoverable errors.

use std::fs::File;
use std::io::ErrorKind;
fn main(){
    let _new_file_result = File::open("hello.txt");
    let _new_file = match _new_file_result {
         Ok(file)=>file, 
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fe) => fe,
                Err(e) => panic!("Problem creating the file:{:?}",e),
            },
            other_error =>{
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

}