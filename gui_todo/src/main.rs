
use iced::Element;
use iced::widget::{text,text_editor};
use iced::widget::{row,column,container};

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
    Print,
    Edit(text_editor::Action),
    TextEditorAction(text_editor::Action)
}



impl Editor {

    fn new() -> Self {
        Self {
            // content: text_editor::Content::new(),
            content: text_editor::Content::with_text( "This is a text editor"),
            text_input: String::new()
        }
    }

    fn title(&self) -> String {
        String::from("Text Editor for a ToDo List")
    }   

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Print => println!("Hey"),
            Message::Edit(action) => {
                self.content.perform(action);
            },
            Message::TextEditorAction(action) => {
                self.content.perform(action);
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
}



pub fn main() -> Result<(), iced::Error> {
    iced::application("To-Do Editor", Editor::update, Editor::view)
    .run_with(|| (Editor::new(), iced::Task::none()))
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