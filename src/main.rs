use iced::{Element, Sandbox, Settings, Color, Background, Command, Length};
use iced::widget::{text_input, container, scrollable, Button, Text, Column, button};
use iced::widget::TextInput;
use iced::window;

pub fn main() -> iced::Result {
    Tocomple::run(Settings {
        window: iced::window::Settings {
            size: (400, 650),
            ..iced::window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Default)]
struct Tocomple {
    input_text1: String,
    input_text2: String,
    button_state1: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    TextInputChanged1(String),
    TextInputChanged2(String),
    ButtonClicked,
}

impl Sandbox for Tocomple {
    type Message = Message;

    fn new() -> Tocomple {
        Tocomple::default()
    }

    fn title(&self) -> String {
        String::from("kafka client")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TextInputChanged1(text) => {
                self.input_text1 = text;
            }
            Message::TextInputChanged2(text) => {
                self.input_text2 = text;
            }
            Message::ButtonClicked => {
                println!("Input 1: {}", self.input_text1);
                println!("Input 2: {}", self.input_text2);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let textbox1 = text_input("", &self.input_text1)
            .on_input(Message::TextInputChanged1);

        let textbox2 = text_input("", &self.input_text2)
            .on_input(Message::TextInputChanged2);;

        let button =Button::new("Press me!").on_press(Message::ButtonClicked);

        let content = Column::new()
            .spacing(20)
            .padding(40)
            .push(textbox1)
            .push(textbox2)
            .push(button);

        scrollable(
            container(content)
                .width(Length::Fill)
                .padding(40)
                .center_x(),
        )
            .into()
    }

}