use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn read_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(& mut s)?;
    Ok(s)
}

#[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    // panic!("crash and burn");
    let v = vec![1,2,3,4];
    println!("{:?}", v);
    let f = File::open("hello.txt");
    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating a file {:?}", e),
            },
            other_error => panic!("Problem oprning the file {:?}", other_error),
        },
    };
    // let file = File::open("hello1.txt").unwrap();
    // let file = File::open("hello1.txt").expect("Failed to open heelo1.txt");

    let content = read_from_file();
    println!("{:?}", content);

    let guess = Guess::new(209);
    println!("{:?}", guess);
}
