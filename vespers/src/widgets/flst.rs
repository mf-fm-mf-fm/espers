use crate::app::Message;
use crate::widgets::ToIced;
use espers::game::Game;
use espers::records::FormList;
use iced::{
    widget::{column, text, Container},
    Element, Length,
};

impl ToIced for FormList {
    fn to_iced(&self, game: &Game) -> Element<Message> {
        column![
            Container::new(text("EDID").size(30)).padding(10),
            self.edid.to_iced(game),
            Container::new(text("Full Name").size(30)).padding(10),
            self.objects.to_iced(game),
        ]
        .width(Length::Fill)
        .padding(10)
        .into()
    }
}
