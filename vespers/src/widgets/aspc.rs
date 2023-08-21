use crate::app::Message;
use crate::widgets::ToIced;
use espers::game::Game;
use espers::records::AcousticSpace;
use iced::{
    widget::{column, text, Container},
    Element, Length,
};

impl ToIced for AcousticSpace {
    fn to_iced(&self, game: &Game) -> Element<Message> {
        column![
            Container::new(text("EDID").size(30)).padding(10),
            self.edid.to_iced(game),
            Container::new(text("Object Bounds").size(30)).padding(10),
            self.bounds.to_iced(game),
            Container::new(text("Ambient").size(30)).padding(10),
            self.ambient.to_iced(game),
            Container::new(text("Region Data").size(30)).padding(10),
            self.region_data.to_iced(game),
            Container::new(text("Reverb").size(30)).padding(10),
            self.reverb.to_iced(game),
        ]
        .width(Length::Fill)
        .padding(10)
        .into()
    }
}
