use crate::app::Message;
use crate::widgets::ToIced;
use espers::plugin::Plugin;
use espers::records::Outfit;
use iced::{
    widget::{column, text, Container},
    Element, Length,
};

impl ToIced for Outfit {
    fn to_iced(&self, plugin: &Plugin) -> Element<Message> {
        column![
            Container::new(text("EDID").size(30)).padding(10),
            self.edid.to_iced(plugin),
            Container::new(text("Inventory").size(30)).padding(10),
            self.inventory.to_iced(plugin),
        ]
        .width(Length::Fill)
        .padding(10)
        .into()
    }
}
