
use iced::{Element,Task,Theme};
use iced::widget::{text,text_editor};
use iced::widget::{column};

use std::fmt::format;
// use std::fmt::Error;
use std::io;
use std::path::Path;
use std::sync::Arc;

use tokio;
use rfd;

struct Editor {
    content: text_editor::Content,
    text_input: String
}

#[derive(Debug,Clone)]
enum Message {
    TextEditorAction(text_editor::Action),
    FileLoaded(Result<Arc<String>, Error>)
}



impl Editor {

    fn new() -> (Self, Task<Message>) {
        (
            Self {
            content: text_editor::Content::with_text(include_str!(r"main.rs")),
            // content: text_editor::Content::with_text( "This is a text editor"),
            text_input: String::new()
        }, 
            // Task::none(),
            Task::perform(load_file(
                format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR"))),
                Message::FileLoaded) // We do "Message/src/main.rs"
        )
    }

    fn title(&self) -> String {
        String::from("Text Editor for a ToDo List")
    }   

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::TextEditorAction(action) => {
                self.content.perform(action);
            },
            Message::FileLoaded(result) => {
                match result {
                    Ok(content) => {
                        self.content = text_editor::Content::with_text(&content);
                    },
                    Err(err) => {
                        // println!("Error loading file: {}", err);
                        print!("Error loading file");
                    }
                }
            }
        }
        iced::Task::none()
    }

    fn view(&self) -> Element<'_, Message> {

        let title = text(self.title());
        let input = iced::widget::TextEditor::new(&self.content)
        .on_action(Message::TextEditorAction);
        // input.into()

        column![title,input].align_x(iced::Center).into()

        // Add something that opens the database, reads and displays the contents

    }

    fn theme(&self) -> Theme {
        Theme::Dracula
    }
}



pub fn main() -> Result<(), iced::Error> {
    iced::application("To-Do Editor", Editor::update, Editor::view)
    .theme(Editor::theme)
    .run_with(|| Editor::new())
}

async fn pick_file() -> Result<Arc<String>, Error> {
    let handle = rfd::AsyncFileDialog::new()
            .set_title("Pick a database")
            .pick_file().await.ok_or(Error::DialogClosed)?;

    load_file(handle.path()).await
}

async fn load_file(path: impl AsRef<Path>) -> Result<Arc<String>, Error> {//io::Result<String> {
    // std::fs::read_to_string(path)
    tokio::fs::read_to_string(path)
            .await
            .map(Arc::new)
            .map_err(|err| err.kind())
            .map_err(Error::IO)
}

#[derive(Debug,Clone)]
enum Error {
    DialogClosed,
    IO(io::ErrorKind)
}