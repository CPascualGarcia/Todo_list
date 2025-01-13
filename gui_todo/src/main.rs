
use iced::{Element,Length,Task,Theme};
use iced::widget::{Button,Container,Text,TextEditor,column,text_editor};
// use iced::widget::{text,text_editor};

use std::io;
// use std::path::Path;
use std::sync::Arc;

// use tokio;
// use rfd;

use rusqlite::{Connection,OpenFlags};


// TO DO
// Add message logics
// Add editor input


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
    // StdError(std::error::Error),
    IcedError(Arc<iced::Error>),
    RSQLError(Arc<rusqlite::Error>),
    IO(io::ErrorKind)
}


impl Clone for AppError {
    fn clone(&self) -> Self {
        match self {
            AppError::IcedError(err) => AppError::IcedError(err.clone()),
            AppError::RSQLError(err) => AppError::RSQLError(err.clone()),
            AppError::IO(err) => AppError::IO(err.clone()),
        }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            // AppError::StdError(err) => write!(f, "Standard error: {}", err),
            AppError::IcedError(err) => write!(f, "Iced error: {}", err),
            AppError::RSQLError(err) => write!(f, "Rusqlite error: {}", err),
            AppError::IO(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl From<iced::Error> for AppError {
    fn from(e: iced::Error) -> Self {
        AppError::IcedError(Arc::new(e))
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(e: rusqlite::Error) -> Self {
        AppError::RSQLError(Arc::new(e))
    }
}


struct DBEditor {
    db_conn: Connection,
    content: text_editor::Content,
    // query: String 
}

#[derive(Debug,Clone)]
enum Message {
    TextEditorAction(text_editor::Action),
    TextAdded(Result<String, AppError>)
}



impl DBEditor {
    fn new(connection:Connection) -> (Self, Task<Message>) {
        (
            Self {
            db_conn: connection,
            content: text_editor::Content::with_text("Write here the no. of the line (e.g. 5)"),
            
            // query: "".to_string()
            // db_path: "TodoList.db".to_string(),
            // db_buffer: "".to_string(),
            // db_index: 0
        },
        // Task::perform(future, Message::TextAdded)
        Task::none()
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TextEditorAction(action) => {
                self.content.perform(action);
                db_writer(&self.db_conn, "mustard", 14).unwrap();
            },
            Message::TextAdded(result) => {
                match result {
                    Ok(content) => {
                        self.content = text_editor::Content::with_text(&content);
                    },
                    Err(err) => {
                        println!("Error loading file: {}", err);
                    }
                }
            }
        }

        iced::Task::none()
    }

    fn view(&self) -> Element<'_,Message> {
        // todo!()
        // db_writer(&self.db_conn, "mustard", 14).unwrap();
        let query_input = 5;
        let result = db_reader(&self.db_conn, query_input as usize).unwrap();

        let display = Text::new("No. of the line: ");
        
        let input = TextEditor::new(&self.content)
            .on_action(Message::TextEditorAction);
        

        let exec_button = Button::new("Execute")
            .on_press(Message::TextAdded(db_reader(&self.db_conn, query_input as usize)));




        let layout = column![
            exec_button,Text::new(result),display,input];
        
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












fn db_reader(conn: &Connection, x: usize) -> Result<String, AppError> {

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