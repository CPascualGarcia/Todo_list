
use iced::{Element,Length,Renderer,Task,Theme};
use iced::widget::{Button,Container,Text,column,text_editor};
// use iced::widget::{text,text_editor};

// use std::io;
use std::sync::Arc;

use rusqlite::{Connection,OpenFlags};


// TO DO
// For the Read feature, only the first word is displayed!
// After adding one element, the Read feature first returns an error, then the proper answer


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


#[derive(Debug)]
enum AppError {
    // StdError(std::error::Error),
    IcedError(Arc<iced::Error>),
    RSQLError(Arc<rusqlite::Error>),
    // IO(io::ErrorKind)
}


impl Clone for AppError {
    fn clone(&self) -> Self {
        match self {
            AppError::IcedError(err) => AppError::IcedError(err.clone()),
            AppError::RSQLError(err) => AppError::RSQLError(err.clone()),
            // AppError::IO(err) => AppError::IO(err.clone()),
        }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            // AppError::StdError(err) => write!(f, "Standard error: {}", err),
            AppError::IcedError(err) => write!(f, "Iced error: {}", err),
            AppError::RSQLError(err) => write!(f, "Rusqlite error: {}", err),
            // AppError::IO(err) => write!(f, "IO error: {}", err),
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
    content_add: text_editor::Content,
    query: String,
    result: String,
    result_add: String
}

#[derive(Debug,Clone)]
enum Message {
    TextEditorAction(text_editor::Action),
    TextEditorActionAdd(text_editor::Action),
    QueryDo,
    QueryChange
}



impl DBEditor {
    fn new(connection:Connection) -> (Self, Task<Message>) {
        (
            Self {
            db_conn: connection,
            content: text_editor::Content::with_text("Write here the no. of the line (e.g. 5)"),
            content_add: text_editor::Content::with_text("Write here as: <line no.> <task>"),
            
            query: String::new(),
            result: String::new(),
            result_add: String::new(),
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
                self.query = self.content.text();
            },
            Message::TextEditorActionAdd(action) => {
                self.content_add.perform(action);
                self.query = self.content_add.text();
            },
            Message::QueryDo => {
                
                match self.query.trim().parse::<usize>() {
                    Ok(query_input) => {
                        self.result = db_reader(&self.db_conn, &query_input).unwrap();
                    },
                    Err(_) => {
                        self.result = "Error parsing query".to_string();
                    }
                };
                // Task::perform(async move{
                //     db_writer(&self.db_conn, "mustard", &self.content.as_text().parse::<usize>()).unwrap()},
                //      Message::TextEditorAction);
                
            },
            Message::QueryChange => {
                let inputs = parser_input(&self.query); 

                if inputs.len() < 2 {
                    self.result_add = "error parsing query".to_string();
                    ()
                };
                
                match (inputs[0].parse::<usize>(), inputs[1].parse::<String>()) {
                    (Ok(line), Ok(contents_line)) => {
                        db_writer(&self.db_conn, contents_line, line).unwrap();
                        self.result_add = "New task added".to_string();
                    },
                    _ => {
                        self.result_add = "Unable to parse query".to_string();
                    }
                }
            }
        }

        iced::Task::none()
    }

    fn view(&self) -> Element<'_,Message> {
        let query_input = 5 as usize;
        let result = db_reader(&self.db_conn, &query_input).unwrap();

        // Verification of an entry
        let display = Text::new("Check task at given line number: ");
        
        let input = iced::widget::TextEditor::new(&self.content)
            .on_action(Message::TextEditorAction);

        let exec_button = Button::new("Search")
        .on_press(Message::QueryDo);

        let output = Text::new(&self.result);
        //
        
        // Addition/modification of an entry
        let display_add: Text<'_, Theme, Renderer> = Text::new("Add/overwrite task: ");

        let input_add: iced::widget::TextEditor<'_, _, Message> = iced::widget::TextEditor::new(&self.content_add)
            .on_action(Message::TextEditorActionAdd);

        let exec_button_add: iced::widget::Button<'_, Message, Theme, Renderer> = Button::new("Add")
        .on_press(Message::QueryChange);

        let output_add: Text<'_, Theme, Renderer> = Text::new(&self.result_add);
        //

        let layout = column![
            Text::new(result),
            display,
            input,
            exec_button,
            output,
            ///////////////////// TODO move this to another column
            display_add,
            input_add,
            exec_button_add,
            output_add
            /////////////////////
            ];
        
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










fn parser_input(input: &str) -> Vec<String> {
    input
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}


fn db_reader(conn: &Connection, x: &usize) -> Result<String, AppError> {

    // Verify the entry exists
    if db_verify(conn, *x) == false {
        // println!("Entry does not exist in database.");
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

fn db_writer(conn: &Connection, buffer: String, x: usize) -> Result<(), std::io::Error> {
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