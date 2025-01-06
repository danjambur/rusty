use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    let greating_file_result = File::open("hello.txt");

    let _greeting_file = match greating_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // an example without a matcher. This is a bit more concise.
    let _greeting_file_unwrap = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // speed run it
    let _greeting_file_speed_run = File::open("hello.txt").unwrap();

    // using the expect method
    // this will give you a more detailed error message - based on what you want to input.

    let _greeting_file_expect = File::open("hello.txt").expect("Failed to open hello.txt");

    // this works the same way as unwrap, but you can specify the error message.
    // this is useful for debugging.

    // Propagating errors
    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }
    let username = read_username_from_file();
    println!("{:?}", username);

    // a more concise way to propagate errors
}
