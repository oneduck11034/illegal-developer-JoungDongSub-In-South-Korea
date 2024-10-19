use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Specify the path to the file you want to read
    let file_path = "example.txt";

    // Open the file
    let mut file = File::open(file_path)?;

    // Create a string to hold the contents
    let mut contents = String::new();

    // Read the file contents into the string
    file.read_to_string(&mut contents)?;

    // Print the contents
    println!("File contents:\n{}", contents);

    Ok(())
}

