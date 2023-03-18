use crate::app::Message;
use crate::widgets::ToIced;
use espers::plugin::Plugin;
use espers::records::{Book, BookData, BookFlags};
use iced::{
    widget::{column, text, Container},
    Element, Length,
};

impl ToIced for BookFlags {
    fn to_iced(&self, _: &Plugin) -> Element<Message> {
        Container::new(text(format!("{:?}", self)))
            .width(Length::Fill)
            .padding(10)
            .into()
    }
}

impl ToIced for BookData {
    fn to_iced(&self, plugin: &Plugin) -> Element<Message> {
        column![
            Container::new(text("Flags")).padding(10),
            self.flags.to_iced(plugin),
            Container::new(text("Type")).padding(10),
            self.kind.to_iced(plugin),
            Container::new(text("Unknown")).padding(10),
            self.unknown.to_iced(plugin),
            Container::new(text("Teaches")).padding(10),
            self.teaches.to_iced(plugin),
            Container::new(text("Value")).padding(10),
            self.value.to_iced(plugin),
            Container::new(text("Weight")).padding(10),
            self.weight.to_iced(plugin)
        ]
        .width(Length::Fill)
        .padding(10)
        .into()
    }
}

impl ToIced for Book {
    fn to_iced(&self, plugin: &Plugin) -> Element<Message> {
        column![
            Container::new(text("EDID").size(30)).padding(10),
            self.edid.to_iced(plugin),
            Container::new(text("Full Name").size(30)).padding(10),
            self.full_name.to_iced(plugin),
            Container::new(text("Description").size(30)).padding(10),
            self.description.to_iced(plugin),
            Container::new(text("Text").size(30)).padding(10),
            self.text.to_iced(plugin),
            Container::new(text("Scripts").size(30)).padding(10),
            self.scripts.to_iced(plugin),
            Container::new(text("Object Bounds").size(30)).padding(10),
            self.bounds.to_iced(plugin),
            Container::new(text("Model Filename").size(30)).padding(10),
            self.model_filename.to_iced(plugin),
            Container::new(text("Model Textures").size(30)).padding(10),
            self.model_textures.to_iced(plugin),
            Container::new(text("Icon").size(30)).padding(10),
            self.icon.to_iced(plugin),
            Container::new(text("Message Icon").size(30)).padding(10),
            self.message_icon.to_iced(plugin),
            Container::new(text("Pickup Sound").size(30)).padding(10),
            self.pickup_sound.to_iced(plugin),
            Container::new(text("Drop Sound").size(30)).padding(10),
            self.drop_sound.to_iced(plugin),
            Container::new(text("Keywords").size(30)).padding(10),
            self.keywords.to_iced(plugin),
            Container::new(text("Data").size(30)).padding(10),
            self.data.to_iced(plugin),
            Container::new(text("Inventory Art").size(30)).padding(10),
            self.inventory_art.to_iced(plugin),
        ]
        .width(Length::Fill)
        .padding(10)
        .into()
    }
}
