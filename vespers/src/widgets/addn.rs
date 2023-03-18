use crate::app::Message;
use crate::widgets::ToIced;
use espers::plugin::Plugin;
use espers::records::{AddonNode, AddonNodeFlags};
use iced::{
    widget::{column, text, Container},
    Element, Length,
};

impl ToIced for AddonNodeFlags {
    fn to_iced(&self, _: &Plugin) -> Element<Message> {
        Container::new(text(format!("{:?}", self)))
            .width(Length::Fill)
            .padding(10)
            .into()
    }
}

impl ToIced for AddonNode {
    fn to_iced(&self, plugin: &Plugin) -> Element<Message> {
        column![
            Container::new(text("EDID").size(30)).padding(10),
            self.edid.to_iced(plugin),
            Container::new(text("Object Bounds").size(30)).padding(10),
            self.bounds.to_iced(plugin),
            Container::new(text("Model Filename").size(30)).padding(10),
            self.model_filename.to_iced(plugin),
            Container::new(text("Model Textures").size(30)).padding(10),
            self.model_textures.to_iced(plugin),
            Container::new(text("Addon Node Index").size(30)).padding(10),
            self.addon_node_index.to_iced(plugin),
            Container::new(text("Ambient Sound").size(30)).padding(10),
            self.ambient_sound.to_iced(plugin),
            Container::new(text("Particle System Cap").size(30)).padding(10),
            self.particle_system_cap.to_iced(plugin),
            Container::new(text("Flags").size(30)).padding(10),
            self.flags.to_iced(plugin),
        ]
        .width(Length::Fill)
        .padding(10)
        .into()
    }
}
