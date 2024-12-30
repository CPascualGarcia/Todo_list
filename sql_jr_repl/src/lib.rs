use std::fs::{OpenOptions,rename};
use std::io::prelude::*;

pub fn writer(file1_path: &str,
    x: usize,
    lines: &Vec<String>) -> Result<(), std::io::Error> {
    
    let file2_path = format!("file_NEW.txt");
    let mut file2 = OpenOptions::new().read(true).write(true).create(true).open(&file2_path).unwrap();
    // let mut writer = BufWriter::new(file2);

    
    println!("Provide content to be written:");
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed to read line");

    // Write contents in NEW file
    for i in 0..x {
        write!(file2, "{}\n", lines[i as usize]).expect("Unable to write to file!");
    }
    write!(file2, "{}", buffer).expect("Unable to write to file!");
    for i in x as usize..(lines.len()) {
        write!(file2, "{}\n", lines[i as usize]).expect("Unable to write to file!");
    }
    
    // Overwrite original file
    rename(file2_path, file1_path).expect("Failed to rename file");

    Ok(())
}