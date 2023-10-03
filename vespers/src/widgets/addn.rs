use crate::app::Message;
use crate::widgets::ToIced;
use espers::game::Game;
use espers::records::{AddonNode, AddonNodeFlags};
use iced::{
    widget::{column, text, Container},
    Element, Length,
};

impl ToIced for AddonNodeFlags {
    fn to_iced(&self, _: &Game) -> Element<Message> {
        Container::new(text(format!("{:?}", self)))
            .width(Length::Fill)
            .padding(10)
            .into()
    }
}

impl ToIced for AddonNode {
    fn to_iced(&self, game: &Game) -> Element<Message> {
        column![
            Container::new(text("EDID").size(30)).padding(10),
            self.edid.to_iced(game),
            Container::new(text("Object Bounds").size(30)).padding(10),
            self.bounds.to_iced(game),
            Container::new(text("Model").size(30)).padding(10),
            self.model.to_iced(game),
            Container::new(text("Addon Node Index").size(30)).padding(10),
            self.addon_node_index.to_iced(game),
            Container::new(text("Ambient Sound").size(30)).padding(10),
            self.ambient_sound.to_iced(game),
            Container::new(text("Particle System Cap").size(30)).padding(10),
            self.particle_system_cap.to_iced(game),
            Container::new(text("Flags").size(30)).padding(10),
            self.flags.to_iced(game),
        ]
        .width(Length::Fill)
        .padding(10)
        .into()
    }
}
