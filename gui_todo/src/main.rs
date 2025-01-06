
use iced::Element;
use iced::widget::{text};//text_editor

struct Editor {
    // content: text_editor::Content
}

#[derive(Debug,Clone)]
enum Message {
    Print,
    // Edit(text_editor::Action)
}



impl Editor {

    fn new() -> Self {
        Self {
            // content: text_editor::Content::new(),
        }
    }

    fn title(&self) -> String {
        String::from("ToDo Editor")
    }   

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Print => println!("Hey"),
            // Message::Edit(action) => {
            //     self.content.edit(action);
            // }
        }
        iced::Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        text("Hello!").into() // convert the text into an Element (general widget)
        // text_editor(&self.content).on_edit(Message::Edit).into()
    }
}



pub fn main() -> Result<(), iced::Error> {
    iced::application("To-Do Editor", Editor::update, Editor::view)
        // .run_with(Editor::new);
    .run_with(|| (Editor::new(), iced::Task::none()))
}