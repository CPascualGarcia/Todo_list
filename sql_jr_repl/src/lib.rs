use std::fs::{OpenOptions,rename};
use std::collections::HashMap;
use std::io::prelude::*;
// use std::result;
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

////////////////////////////// DATABASE


pub fn db_setup(db_path: &str) -> Result<(), std::io::Error> {
    let conn = Connection::open_with_flags(db_path,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE).unwrap();

    // Create basic table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS 
        tasks (id INTEGER PRIMARY KEY, task TEXT NOT NULL)", 
        ()).unwrap();

    Ok(())
}

pub fn db_writer(db_path: &str, buffer: &str, x: usize) -> Result<(), std::io::Error> {
    let conn = Connection::open_with_flags(db_path,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE).unwrap();

    // Insert rows into the table
    let mut stmt = conn.prepare(
        "INSERT OR REPLACE INTO tasks (id, task) VALUES (?1, ?2)").unwrap();
    stmt.execute((x, buffer)).unwrap();

    Ok(())
}

pub fn db_add(db_path: &str, x: usize) -> Result<(), std::io::Error> {
    println!("Provide content to be written:");
    let mut buffer: String = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed to read line");
    db_writer(&db_path, &buffer, x as usize)?;

    Ok(())
}



pub fn db_verify(db_path: &str, x: usize) -> bool {
    let conn = Connection::open_with_flags(db_path,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE).unwrap();

    // Check that the entry does not exist
    let mut stmt = conn.prepare("SELECT * FROM tasks WHERE id = ?1").unwrap();
    let mut rows = stmt.query(&[&x]).unwrap();
    if rows.next().unwrap().is_some() {
        return true}
    else {return false};
}



pub fn db_remove(db_path: &str, x: usize) -> Result<(), std::io::Error> {
    let conn = Connection::open_with_flags(db_path,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE).unwrap();

    // Change the entry if it does exist
    if db_verify(db_path, x) == true {
        let mut stmt = conn.prepare("DELETE FROM tasks WHERE id = ?1").unwrap();
        stmt.execute(&[&x]).unwrap();
    } 
    else {
        println!("Entry does not exist in database.");
    }

    Ok(())
}


pub fn db_size(db_path: &str) -> Result<usize, rusqlite::Error> {
    let conn = Connection::open(db_path)?;

    let mut stmt = conn.prepare("SELECT COUNT(*) FROM tasks")?;
    let mut rows = stmt.query([])?;
    let count = rows.next()?.unwrap().get(0)?;

    Ok(count)
}


pub fn db_reader(db_path: &str, x: usize) -> Result<String, rusqlite::Error> {

    // Verify the entry exists
    if db_verify(db_path, x) == false {
        println!("Entry does not exist in database.");
        return Ok("".to_string());
    }
    
    let conn = Connection::open(db_path)?;

    let mut stmt = conn.prepare("SELECT * FROM tasks WHERE id = ?1").unwrap();
    let mut rows = stmt.query(&[&x])?;

    let row = rows.next().unwrap().unwrap();
    let task: String = row.get(1)?;

    Ok(task)
}


pub fn read_all(db_path: &str) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(db_path).unwrap();

    let mut stmt = conn.prepare("SELECT * FROM tasks").unwrap();
    let mut rows = stmt.query(()).unwrap();

    while let Some(row) = rows.next().unwrap() {
        let id: i32 = row.get(0).unwrap();
        let task: String = row.get(1).unwrap();

        println!("ID: {}, Task: {}", id, task);
    }

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////
/// PARSER
///////////////////////////////////////////////////////////////////////////////

pub fn parser_input(input: &str) -> Vec<String> {
    input
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

pub enum ReturnType {
    Int(i32),
    String(String),
}

pub fn parser_function(input: &str, command_dict: &HashMap<String, Box<dyn Fn() -> ReturnType>>) -> ReturnType {
    let parsed_input = parser_input(input);
    let command = parsed_input[0].clone();
    let command_function = command_dict.get(&command).unwrap();
    return command_function();
    // println!("{}", command_function);
}

///////////////////////////////////////////////////////////////////////////////
/// TESTS
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
use std::fs::remove_file;
// use std::any::Any;


#[test]
fn test_parser_input() {
    let input = "hello world";
    let expected = vec!["hello", "world"];
    assert_eq!(parser_input(input), expected);
}

#[test]
fn test_parser_function() {

    fn display_hello() -> String {
        "hello".to_string()
    }
    fn display_goodbye() -> i32 {
        42
    }
    
    // Define HashMap of functions
    let mut command_dict:  HashMap<String, Box<dyn Fn() -> ReturnType>> = HashMap::new();
    command_dict.insert("goodbye".to_string(), Box::new(|| ReturnType::Int(display_goodbye())));
    command_dict.insert("hello".to_string(), Box::new(|| ReturnType::String(display_hello())));

    // Define input
    let input = "hello world";

    let fn_expected = parser_function(input, &command_dict);

    match fn_expected {
        ReturnType::String(s) => assert_eq!(s, "hello".to_string()),
        ReturnType::Int(_) => panic!("Expected a string"),
    }

}

#[test]
fn test_db_setup() {
    // Call the database
    let db_path: &str = "TodoList_test.db"; // Prepare the path to the database
    db_setup(db_path).unwrap(); // Set database

    let conn = Connection::open_with_flags(db_path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE).unwrap();

    // Prepare example entry
    let index: i32 = 14;
    let entry: &str = "mustard";

    // Add example entry to database
    conn.execute(
        "INSERT OR REPLACE INTO tasks (id, task) VALUES (?1, ?2)",
        (&index, &entry),
    ).unwrap();

    // Assert that the db recorded the entry
    assert_eq!(db_verify(db_path, 14),true);

    // Assert that some other entry does not exist
    assert_eq!(db_verify(db_path, 15),false);

    // Close the database
    conn.close().unwrap();

    // Erase database
    remove_file(db_path).unwrap();

}


#[test]
fn test_db_writer() -> Result<(), Box<dyn std::error::Error>> {
    let db_path: &str = "TodoList_test.db"; // Prepare the path to the database
    db_setup(db_path).unwrap();             // Set database

    // Write to the database
    db_writer(db_path, "mustard", 14).unwrap();

    // Verify that the contents were written
    let conn = Connection::open_with_flags(db_path,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
    ).unwrap();

    let mut stmt = conn.prepare("SELECT * FROM tasks WHERE id = ?1")?;
    let mut rows = stmt.query(&[&14])?;

    // Check that the entry was recorded
    let row = rows.next().unwrap().unwrap();
    let id: i32 = row.get(0).unwrap();
    let task: String = row.get(1).unwrap();

    assert_eq!(id, 14);
    assert_eq!(task, "mustard");

    Ok(())
}

#[test]
fn test_db_empty_entry() -> Result<(), Box<dyn std::error::Error>> {
    let db_path: &str = "TodoList_test.db"; // Prepare the path to the database
    db_setup(db_path).unwrap();       // Set database

    let index: i32 = -1; // Index of non-existent entry

    let conn = Connection::open_with_flags(db_path,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
    ).unwrap();

    // Add a couple of entries
    db_writer(db_path, "toy", 1).unwrap();
    db_writer(db_path, "mustard", 2).unwrap();

    {
    let mut stmt = conn.prepare("SELECT * FROM tasks WHERE id = ?1")?;
    let mut rows = stmt.query(&[&index])?;

    // Assert that no entry -1 exists
    assert!(rows.next().unwrap().is_none());
    };

    // Close the database
    conn.close().unwrap();

    // Erase database
    remove_file(db_path)?;
    Ok(())
}

#[test]
fn test_db_erase() -> Result<(), Box<dyn std::error::Error>> {
    let db_path: &str = "TodoList_test.db"; // Prepare the path to the database
    db_setup(db_path).unwrap();       // Set database

    let conn = Connection::open_with_flags(db_path,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
    ).unwrap();

    // Add a couple of entries
    db_writer(db_path, "toy", 1).unwrap();
    db_writer(db_path, "mustard", 2).unwrap();

    // Erase an entry
    db_remove(db_path, 1).unwrap();

    // Assert that the entry no longer exists
    {
    let mut stmt = conn.prepare("SELECT * FROM tasks WHERE id = ?1")?;
    let mut rows = stmt.query(&[&1])?;
    assert!(rows.next().unwrap().is_none());
    };

    // Close the database
    conn.close().unwrap();

    // Erase database
    remove_file(db_path).unwrap();
    Ok(())
}

#[test]
fn test_db_size() -> Result<(), Box<dyn std::error::Error>> {
    let db_path: &str = "TodoList_test.db"; // Prepare the path to the database
    db_setup(db_path).unwrap();       // Set database

    let conn = Connection::open_with_flags(db_path,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,    
    ).unwrap();

    // Add a couple of entries
    db_writer(db_path, "toy", 1).unwrap();
    db_writer(db_path, "mustard", 2).unwrap();

    // Assert that the size is correct
    let size = db_size(db_path).unwrap();
    assert_eq!(size, 2);

    // Close the database
    conn.close().unwrap();

    // Erase database
    remove_file(db_path).unwrap();
    Ok(())
}

#[test]
fn test_db_verify() -> Result<(), Box<dyn std::error::Error>> {
    let db_path: &str = "TodoList_test.db"; // Prepare the path to the database
    db_setup(db_path).unwrap();       // Set database

    let conn = Connection::open_with_flags(db_path,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
    ).unwrap();

    // Add a couple of entries
    db_writer(db_path, "toy", 1).unwrap();
    db_writer(db_path, "mustard", 2).unwrap();

    // Read the entry whenever it exists
    assert_eq!(db_reader(db_path, 1).unwrap(), "toy");
    assert_eq!(db_reader(db_path, 2).unwrap(), "mustard");
    assert_eq!(db_reader(db_path, 3).unwrap(), "");

    // Close the database
    conn.close().unwrap();

    // Erase database
    remove_file(db_path).unwrap();
    Ok(())
}