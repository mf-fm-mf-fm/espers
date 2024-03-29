use crate::app::Message;
use crate::widgets::ToIced;
use espers::game::Game;
use espers::records::Outfit;
use iced::{
    widget::{column, text, Container},
    Element, Length,
};

impl ToIced for Outfit {
    fn to_iced(&self, game: &Game) -> Element<Message> {
        column![
            Container::new(text("EDID").size(30)).padding(10),
            self.edid.to_iced(game),
            Container::new(text("Inventory").size(30)).padding(10),
            self.inventory.to_iced(game),
        ]
        .width(Length::Fill)
        .padding(10)
        .into()
    }
}
