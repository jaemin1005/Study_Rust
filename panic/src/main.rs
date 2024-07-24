use std::{fs::File, io::{self, ErrorKind, Read}};

fn main() {
    //panic!("creash and burn");

    // let v = vec![1, 2, 3];

    // v[99];

    let greeting_file_result = File::open("heelo.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problen creating the file: {:?}",e)
            },

            otehr_error => {
                panic!("Problen opening the file: {:?}", otehr_error)
            }
        }
    };

    let greeting_file = File::open("hello2.txt")
        .expect("hello.txt should be included in this project");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello2.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username){
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello2.txt")?;
    let mut username = String::new();

    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut userName = String::new();
    File::open("hello2.txt")?.read_to_string(&mut userName)?;

    Ok(userName)
}