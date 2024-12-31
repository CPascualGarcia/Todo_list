use std::fs::{OpenOptions,rename};
use std::io::prelude::*;
use rusqlite::{Connection,OpenFlags};

pub fn writer_line(file1_path: &str,
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

pub fn eraser_line(file1_path: &str, x: usize, lines: &mut Vec<String>) -> Result<(), std::io::Error> {
    let file2_path = format!("file_NEW.txt");
    let mut file2 = OpenOptions::new().read(true).write(true).create(true).open(&file2_path).unwrap();
    
    // Remove the content of the chosen line
    lines.remove(x as usize);

    // Write contents in NEW file
    for i in 0 as usize..(lines.len()) {
        write!(file2, "{}\n", lines[i as usize]).expect("Unable to write to file!");
    }

    // Overwrite original file
    rename(file2_path, file1_path).expect("Failed to rename file");

    Ok(())
} 


pub fn db_setup(db_path: &str) -> Result<(), std::io::Error> {
    let conn = Connection::open_with_flags(db_path,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE).unwrap();

    // Create basic table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS 
        tasks (id INTEGER PRIMARY KEY, task TEXT NOT NULL)", 
        ()).unwrap();

    // Insert rows into the table
    let mut stmt = conn.prepare(
        "INSERT OR REPLACE INTO tasks (id, task) VALUES (?1, ?2)").unwrap();
    stmt.execute((13, "sandwich")).unwrap();
        
    conn.execute(
        "INSERT OR REPLACE INTO tasks (id, task) VALUES (?1, ?2)",
        (14, "mustard"),
    ).unwrap();

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////
/// TESTS
///////////////////////////////////////////////////////////////////////////////

#[test]
fn test_db_setup() {
    // Call the database
    let db_path: &str = "TodoList.db"; // Prepare the path to the database
    db_setup(db_path).unwrap(); // Set database

    let conn = Connection::open_with_flags(db_path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE).unwrap();


    // Prepare a query statement
    let mut stmt = conn.prepare("SELECT * FROM tasks WHERE id = ?1").unwrap();
    
    // Query the database
    let mut rows = stmt.query(&[&14]).unwrap();

    let row = rows.next().unwrap().unwrap();
    let id: i32 = row.get(0).unwrap();
    let task: String = row.get(1).unwrap();

    assert_eq!(id, 14);
    assert_eq!(task, "mustard");

}

#[test]
fn test_db_empty_entry() {
    let db_path: &str = "TodoList.db"; // Prepare the path to the database
    db_setup(db_path).unwrap();       // Set database

    let index: i32 = -1; // Index of non-existent entry

    let conn = Connection::open_with_flags(db_path,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
    ).unwrap();

    let mut stmt = conn.prepare("SELECT * FROM tasks WHERE id = ?1").unwrap();
    let mut rows = stmt.query(&[&index]).unwrap();

    // Assert that no entry -1 exists
    assert!(rows.next().unwrap().is_none());
}
