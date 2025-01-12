
use iced::{Element,Length,Task,Theme};
use iced::widget::{Container,Text,column};
// use iced::widget::{text,text_editor};

use std::io;
// use std::path::Path;
// use std::sync::Arc;

// use tokio;
// use rfd;

use rusqlite::{Connection,OpenFlags};



fn main() -> Result<(),AppError> {
    let db_path: &str = "TodoList.db"; // Prepare the path to the database
    db_setup(db_path).unwrap();       // Set database
    let conn = Connection::open_with_flags(db_path,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE).unwrap();
    

    // Set the app
    iced::application("To-Do DBEditor", DBEditor::update, DBEditor::view)
    .theme(DBEditor::theme)
    .run_with(|| DBEditor::new(conn))?;
    Ok(())
}

// fn db_setup(db_path: &str) -> Result<(), AppError> {
//     let conn = Connection::open_with_flags(db_path,
//         OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE).unwrap();
//     // Create basic table
//     conn.execute(
//         "CREATE TABLE IF NOT EXISTS tasks (id INTEGER PRIMARY KEY, task TEXT NOT NULL)", 
//         ());
//     Ok(())
// }

#[derive(Debug)]
enum AppError {
    StdError(Box<dyn std::error::Error>),
    IcedError(iced::Error),
    IO(io::ErrorKind)
}

impl From<iced::Error> for AppError {
    fn from(e: iced::Error) -> Self {
        AppError::IcedError(e)
    }
}


struct DBEditor {
    db_conn: Connection,
}

#[derive(Debug,Clone)]
enum Message {
    
}



impl DBEditor {
    fn new(connection:Connection) -> (Self, Task<Message>) {
        (
            Self {
            db_conn: connection,
            // db_path: "TodoList.db".to_string(),
            // db_buffer: "".to_string(),
            // db_index: 0
        },
        Task::none()
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            
        }
    }

    fn view(&self) -> Element<Message> {
        // todo!()
        // db_writer(&self.db_conn, "mustard", 14).unwrap();
        let query_input = 5;
        let result = db_reader(&self.db_conn, query_input as usize).unwrap();

        let layout = column![Text::new(result)];
        
        
        Container::new(layout)
            .align_x(iced::Center)
            .align_y(iced::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dracula
    }
}












fn db_reader(conn: &Connection, x: usize) -> Result<String, rusqlite::Error> {

    // Verify the entry exists
    if db_verify(conn, x) == false {
        println!("Entry does not exist in database.");
        return Ok("NONE".to_string());
    }
    
    let mut stmt = conn.prepare("SELECT * FROM tasks WHERE id = ?1")?;
    let mut rows = stmt.query(&[&x])?;

    let row = rows.next().unwrap().unwrap();
    let task: String = row.get(1)?;

    Ok(task)
}


fn db_verify(conn: &Connection, x: usize) -> bool {

    // Check that the entry does not exist
    let mut stmt = conn.prepare("SELECT * FROM tasks WHERE id = ?1").unwrap();
    let mut rows = stmt.query(&[&x]).unwrap();
    if rows.next().unwrap().is_some() {
        return true}
    else {return false};
}

pub fn db_writer(conn: &Connection, buffer: &str, x: usize) -> Result<(), std::io::Error> {
    // Insert rows into the table
    let mut stmt = conn.prepare(
        "INSERT OR REPLACE INTO tasks (id, task) VALUES (?1, ?2)").unwrap();
    stmt.execute((x, buffer)).unwrap();

    Ok(())
}

fn db_setup(db_path: &str) -> Result<(), std::io::Error> {
    let conn = Connection::open_with_flags(db_path,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE).unwrap();

    // Create basic table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS 
        tasks (id INTEGER PRIMARY KEY, task TEXT NOT NULL)", 
        ()).unwrap();

    Ok(())
}