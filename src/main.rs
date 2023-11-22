use iced::{Element, Sandbox, Settings, Color, Background} ;
use iced::widget::{text_input,  container, };
use iced::window;

pub fn main() -> iced::Result {
    Tocomple::run(Settings {
        window: window::Settings {
            size: (400, 650),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Default)]
struct Tocomple {
    // text_input_state: text_input::State,
    input_text: String,
}
#[derive(Debug, Clone)]
enum Message {
    TextInputChanged(String),
}

impl Sandbox for Tocomple {
    type Message = Message;

    fn new() -> Tocomple {
        Tocomple::default()
    }
    fn title(&self) -> String {
        String::from("kafka client")
    }
    fn update(&mut self, message: Message)   {
        match message {
            Message::TextInputChanged(input) => {
                self.input_text = input;
            }
        }
    }
    fn view(&self) -> Element<Message> {
      let input = text_input(&self.input_text, "host").on_input(Message::TextInputChanged);

        container(input)
            .width(200)
            .height(100)
            .center_x()
            .center_y()
            .into()
    }
}
