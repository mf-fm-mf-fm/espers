use crate::app::Message;
use crate::widgets::ToIced;
use espers::game::Game;
use espers::records::Keyword;
use iced::{
    widget::{column, row, text, Container},
    Element, Length,
};

impl ToIced for Keyword {
    fn to_iced(&self, game: &Game) -> Element<Message> {
        column![
            row![
                Container::new(text("EDID")).width(Length::Fill).padding(10),
                self.edid.to_iced(game)
            ],
            row![
                Container::new(text("Color"))
                    .width(Length::Fill)
                    .padding(10),
                self.color.to_iced(game)
            ],
        ]
        .width(Length::Fill)
        .padding(10)
        .into()
    }
}
