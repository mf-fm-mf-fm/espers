use crate::app::Message;
use crate::widgets::ToIced;
use espers::game::Game;
use espers::records::gmst::Value;
use espers::records::GameSetting;
use iced::{
    widget::{column, row, text, Container},
    Element, Length,
};
impl ToIced for Value {
    fn to_iced(&self, game: &Game) -> Element<Message> {
        match self {
            Value::Bool(b) => b.to_iced(game),
            Value::Int(i) => i.to_iced(game),
            Value::Float(f) => f.to_iced(game),
            Value::Str(s) => s.to_iced(game),
            Value::Unknown(u) => u.to_iced(game),
        }
    }
}

impl ToIced for GameSetting {
    fn to_iced(&self, game: &Game) -> Element<Message> {
        let type_text = match self.value {
            Value::Bool(_) => "Boolean",
            Value::Int(_) => "Integer",
            Value::Float(_) => "Float",
            Value::Str(_) => "String",
            Value::Unknown(_) => "Unknown",
        };
        column![
            row![
                Container::new(text("EDID")).width(Length::Fill).padding(10),
                self.edid.to_iced(game)
            ],
            row![
                Container::new(text("Type")).width(Length::Fill).padding(10),
                Container::new(text(type_text))
                    .width(Length::Fill)
                    .padding(10)
            ],
            row![
                Container::new(text("Value"))
                    .width(Length::Fill)
                    .padding(10),
                self.value.to_iced(game)
            ],
        ]
        .width(Length::Fill)
        .padding(10)
        .into()
    }
}
