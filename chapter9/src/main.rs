use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::ErrorKind;

// A Shortcut for Propagating Errors: the ? Operator
// can only be used in functions that return Result<T,E> / Option<T>
// Using ? in a main function with this return type is allowed.
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// example for propagating errors
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e)
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

fn main() {

    /*
    let f: Result<File, std::io::Error> = File::open("hello.txt");
    let f = match f { 
        Ok(file) => file,
        Err(error) => match error.kind() {
         ErrorKind::NotFound => match File::create("hello.text") {
             Ok(fc) => fc,
             Err(e) => panic!("Problem creating the file: {:?}", e)
         }
         other_error => 
            panic!("Problem opening the file: {:?}", other_error)
        }
    };
    */

    /*
    use std::fs::File;
        fn main() {
            let f = File::open("hello.txt").unwrap();
        }
    */

    /*
    fn main() {
        let f = File::open("hello.txt").expect("Failed to open hello.txt");
    }
    */

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // panics
    // let v = vec![1, 2, 3];
    // v[99];

    //panic!("HCF")
}
