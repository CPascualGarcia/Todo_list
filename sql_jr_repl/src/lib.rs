use std::fs::{OpenOptions,rename,remove_file};
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
    // let mut stmt = conn.prepare(
    //     "INSERT OR REPLACE INTO tasks (id, task) VALUES (?1, ?2)").unwrap();
    // stmt.execute((1, "sandwich")).unwrap();
        
    // conn.execute(
    //     "INSERT OR REPLACE INTO tasks (id, task) VALUES (?1, ?2)",
    //     (2, "mustard"),
    // ).unwrap();

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


// pub fn db_add(db_path: &str, x: usize) -> Result<(), std::io::Error> {
//     let conn = Connection::open_with_flags(db_path,
//         OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE).unwrap();

//     // Check that the entry does not exist
//     let mut stmt = conn.prepare("SELECT * FROM tasks WHERE id = ?1").unwrap();
//     let mut rows = stmt.query(&[&x]).unwrap();
//     if rows.next().unwrap().is_some() {
//         println!("Entry id already defined in database. Still overwrite? [y/n]");
//         loop {
//             let mut buffer = String::new();
//             std::io::stdin().read_line(&mut buffer).expect("Failed to read line");
//             if buffer.trim() == "y" {
//                 break;
//             }
//             else if buffer.trim() == "n" {
//                 return Ok(());
//             }
//             else {
//                 println!("Still overwrite? [y/n]");
//             }
//         }
//     };
    
//     println!("Provide new entry in database:");
//     let mut buffer = String::new();
//     std::io::stdin().read_line(&mut buffer).expect("Failed to read line");
    
//     // Insert rows into the table
//     let mut stmt = conn.prepare(
//         "INSERT OR REPLACE INTO tasks (id, task) VALUES (?1, ?2)").unwrap();
//     stmt.execute((x, buffer)).unwrap();

//     Ok(())
// }


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

    let mut stmt = conn.prepare("DELETE FROM tasks WHERE id = ?1").unwrap();
    stmt.execute(&[&x]).unwrap();

    // check if the entry does not exist


    Ok(())
}

///////////////////////////////////////////////////////////////////////////////
/// TESTS
///////////////////////////////////////////////////////////////////////////////

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


    // conn.execute(
    //     "INSERT OR REPLACE INTO tasks (id, task) VALUES (?1, ?2)",
    //     (1, "toy"),
    // ).unwrap();

    // conn.execute(
    //     "INSERT OR REPLACE INTO tasks (id, task) VALUES (?1, ?2)",
    //     (2, "mustard"),
    // ).unwrap();

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
