use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    // Define the file path
    let file_path = "example.txt";
    // Open the file for reading
    let file = std::fs::File::open(file_path)?;
    // Create a buffered reader for the file
    let reader = std::io::BufReader::new(file);

    // Initialize a variable to store the total word count
    let mut word_count = 0;

    // Iterate over each line in the file
    for line in reader.lines() {
        // Split the line into words and count them
        if let Ok(line) = line {
            word_count += line.split_whitespace().count();
        }
    }

    // Print the total number of words in the file
    println!("Total words: {}", word_count);

    // Return successful program completion
    Ok(())
}
