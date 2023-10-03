pub mod addn;
pub mod aspc;
pub mod book;
pub mod flst;
pub mod gmst;
pub mod kywd;
pub mod otft;

use crate::app::Message;

use espers::common::{FormID, LocalizedString, WString32};
use espers::fields::{
    dest::{DestructionDataHeader, StageData},
    dmds::{DestructionTexture, DestructionTextures},
    dstd::StageDataHeader,
    model::AlternateTexture,
    AlternateTextures, DestructionData, Model, ModelTextures, ObjectBounds, Property, Script,
    ScriptList, Textures, Unknown4,
};
use espers::game::Game;
use espers::records::Record;
use iced::{
    alignment::Horizontal,
    widget::{text, Column, Container},
    Element, Length,
};
use iced_aw::Grid;
use rgb::RGBA8;
use ron::ser::to_string_pretty;
use std::fmt::Debug;

pub trait ToIced {
    fn to_iced(&self, _: &Game) -> Element<Message>;
}

impl ToIced for bool {
    fn to_iced(&self, _: &Game) -> Element<Message> {
        Container::new(text(self))
            .width(Length::Fill)
            .padding(10)
            .into()
    }
}

impl ToIced for u8 {
    fn to_iced(&self, _: &Game) -> Element<Message> {
        Container::new(text(self))
            .width(Length::Fill)
            .padding(10)
            .into()
    }
}

impl ToIced for u16 {
    fn to_iced(&self, _: &Game) -> Element<Message> {
        Container::new(text(self))
            .width(Length::Fill)
            .padding(10)
            .into()
    }
}

impl ToIced for i16 {
    fn to_iced(&self, _: &Game) -> Element<Message> {
        Container::new(text(self))
            .width(Length::Fill)
            .padding(10)
            .into()
    }
}

impl ToIced for f32 {
    fn to_iced(&self, _: &Game) -> Element<Message> {
        Container::new(text(self))
            .width(Length::Fill)
            .padding(10)
            .into()
    }
}

impl ToIced for u32 {
    fn to_iced(&self, _: &Game) -> Element<Message> {
        Container::new(text(format!("0x{:08X}", self)))
            .width(Length::Fill)
            .padding(10)
            .into()
    }
}

impl ToIced for i32 {
    fn to_iced(&self, _: &Game) -> Element<Message> {
        Container::new(text(format!("0x{:08X}", self)))
            .width(Length::Fill)
            .padding(10)
            .into()
    }
}

impl ToIced for String {
    fn to_iced(&self, _: &Game) -> Element<Message> {
        Container::new(text(self))
            .width(Length::Fill)
            .padding(10)
            .into()
    }
}

impl ToIced for RGBA8 {
    fn to_iced(&self, _: &Game) -> Element<Message> {
        Container::new(
            text("███")
                .width(Length::Fixed(20.))
                .height(Length::Fixed(20.))
                .style(iced::Color::from_rgb8(self.r, self.g, self.b)),
        )
        .width(Length::Fill)
        .padding(10)
        .into()
    }
}

impl<T: ToIced> ToIced for Vec<T> {
    fn to_iced(&self, game: &Game) -> Element<Message> {
        Column::with_children(self.iter().map(|x| x.to_iced(game)).collect())
            .width(Length::Fill)
            .padding(10)
            .into()
    }
}

impl<T: ToIced, const N: usize> ToIced for [T; N] {
    fn to_iced(&self, game: &Game) -> Element<Message> {
        Column::with_children(self.iter().map(|x| x.to_iced(game)).collect())
            .width(Length::Fill)
            .padding(10)
            .into()
    }
}

impl<T: ToIced> ToIced for Option<T> {
    fn to_iced(&self, game: &Game) -> Element<Message> {
        match self {
            Some(s) => s.to_iced(game),
            None => Container::new(text("<not set>"))
                .width(Length::Fill)
                .padding(10)
                .into(),
        }
    }
}

impl<T, E> ToIced for Result<T, E>
where
    T: ToIced,
    E: Debug,
{
    fn to_iced(&self, game: &Game) -> Element<Message> {
        match self {
            Ok(s) => s.to_iced(game),
            Err(err) => Container::new(text(format!("ERR: {:?}", err)))
                .width(Length::Fill)
                .padding(10)
                .into(),
        }
    }
}

impl ToIced for ObjectBounds {
    fn to_iced(&self, _: &Game) -> Element<Message> {
        Container::new(text(format!(
            "[{}, {}, {}] -> [{}, {}, {}]",
            self.x1, self.y1, self.z1, self.x2, self.y2, self.z2
        )))
        .width(Length::Fill)
        .padding(10)
        .into()
    }
}

impl ToIced for LocalizedString {
    fn to_iced(&self, game: &Game) -> Element<Message> {
        let st = game.string_tables();
        let val = match self {
            LocalizedString::Localized(l) => match st.get_string(&l) {
                Some(s) => text(s.clone()),
                None => text("<not set>"),
            },
            LocalizedString::ZString(z) => text(z.clone()),
        };

        Container::new(val).width(Length::Fill).padding(10).into()
    }
}

impl ToIced for WString32 {
    fn to_iced(&self, _: &Game) -> Element<Message> {
        Container::new(text(self.to_string()))
            .width(Length::Fill)
            .padding(10)
            .into()
    }
}

impl ToIced for FormID {
    fn to_iced(&self, game: &Game) -> Element<Message> {
        let x = match game.get_record_by_form_id(self) {
            Some(Ok(r)) => format!("{}", r),
            Some(Err(err)) => format!("ERR: {:?}", err),
            None => "<not set>".into(),
        };
        Container::new(text(x))
            .width(Length::Fill)
            .padding(10)
            .into()
    }
}

impl ToIced for Property {
    fn to_iced(&self, game: &Game) -> Element<Message> {
        let mut g = Grid::with_columns(2);

        match self {
            Property::Bool {
                name,
                status,
                value,
            } => {
                g.insert(Container::new(text("Name")).padding(10));
                g.insert(name.to_iced(game));
                g.insert(Container::new(text("Status")).padding(10));
                g.insert(status.to_iced(game));
                g.insert(Container::new(text("Value")).padding(10));
                g.insert(value.to_iced(game));
            }
            Property::ObjectV1 {
                name,
                status,
                form_id,
                alias,
                unused,
            } => {
                g.insert(Container::new(text("Name")).padding(10));
                g.insert(name.to_iced(game));
                g.insert(Container::new(text("Status")).padding(10));
                g.insert(status.to_iced(game));
                g.insert(Container::new(text("Form ID")).padding(10));
                g.insert(form_id.to_iced(game));
                g.insert(Container::new(text("Alias")).padding(10));
                g.insert(alias.to_iced(game));
                g.insert(Container::new(text("Unused")).padding(10));
                g.insert(unused.to_iced(game));
            }
            Property::ObjectV2 {
                name,
                status,
                unused,
                alias,
                form_id,
            } => {
                g.insert(Container::new(text("Name")).padding(10));
                g.insert(name.to_iced(game));
                g.insert(Container::new(text("Status")).padding(10));
                g.insert(status.to_iced(game));
                g.insert(Container::new(text("Unused")).padding(10));
                g.insert(unused.to_iced(game));
                g.insert(Container::new(text("Alias")).padding(10));
                g.insert(alias.to_iced(game));
                g.insert(Container::new(text("Form ID")).padding(10));
                g.insert(form_id.to_iced(game));
            }
            Property::String {
                name,
                status,
                value,
            } => {
                g.insert(Container::new(text("Name")).padding(10));
                g.insert(name.to_iced(game));
                g.insert(Container::new(text("Status")).padding(10));
                g.insert(status.to_iced(game));
                g.insert(Container::new(text("Value")).padding(10));
                g.insert(value.to_iced(game));
            }
            Property::Int {
                name,
                status,
                value,
            } => {
                g.insert(Container::new(text("Name")).padding(10));
                g.insert(name.to_iced(game));
                g.insert(Container::new(text("Status")).padding(10));
                g.insert(status.to_iced(game));
                g.insert(Container::new(text("Value")).padding(10));
                g.insert(value.to_iced(game));
            }
            Property::Float {
                name,
                status,
                value,
            } => {
                g.insert(Container::new(text("Name")).padding(10));
                g.insert(name.to_iced(game));
                g.insert(Container::new(text("Status")).padding(10));
                g.insert(status.to_iced(game));
                g.insert(Container::new(text("Value")).padding(10));
                g.insert(value.to_iced(game));
            }
            other => g.insert(
                Container::new(text(to_string_pretty(&other, Default::default()).unwrap()))
                    .padding(10),
            ),
        }

        Container::new(g).width(Length::Fill).padding(10).into()
    }
}

impl ToIced for Script {
    fn to_iced(&self, game: &Game) -> Element<Message> {
        let g = Grid::with_columns(2)
            .push(Container::new(text("Name")).padding(10))
            .push(self.name.to_iced(game))
            .push(Container::new(text("Status")).padding(10))
            .push(self.status.to_iced(game))
            .push(Container::new(text("Properties")).padding(10))
            .push(self.properties.to_iced(game));

        Container::new(g).width(Length::Fill).padding(10).into()
    }
}

impl ToIced for ScriptList {
    fn to_iced(&self, game: &Game) -> Element<Message> {
        let g = Grid::with_columns(2)
            .push(Container::new(text("Version")).padding(10))
            .push(self.version.to_iced(game))
            .push(Container::new(text("Object Format")).padding(10))
            .push(self.object_format.to_iced(game))
            .push(Container::new(text("Scripts")).padding(10))
            .push(self.scripts.to_iced(game));

        Container::new(g).width(Length::Fill).padding(10).into()
    }
}
impl ToIced for Unknown4 {
    fn to_iced(&self, game: &Game) -> Element<Message> {
        let g = Grid::with_columns(2)
            .push(Container::new(text("Unknown 1")).padding(10))
            .push(self.unknown1.to_iced(game))
            .push(Container::new(text("Unknown 2")).padding(10))
            .push(self.unknown2.to_iced(game))
            .push(Container::new(text("Unknown 3")).padding(10))
            .push(self.unknown3.to_iced(game));

        Container::new(g).width(Length::Fill).padding(10).into()
    }
}

impl ToIced for ModelTextures {
    fn to_iced(&self, game: &Game) -> Element<Message> {
        let g = Grid::with_columns(2)
            .push(Container::new(text("Count")).padding(10))
            .push(self.count.to_iced(game))
            .push(Container::new(text("Unknown4 Count")).padding(10))
            .push(self.unknown4_count.to_iced(game))
            .push(Container::new(text("Unknown2")).padding(10))
            .push(self.unknown5.to_iced(game))
            .push(Container::new(text("Unknown4s")).padding(10))
            .push(self.unknown4s.to_iced(game));

        Container::new(g).width(Length::Fill).padding(10).into()
    }
}

impl ToIced for Textures {
    fn to_iced(&self, game: &Game) -> Element<Message> {
        match self {
            Textures::Header(x) => x.to_iced(game),
            Textures::NoHeader(x) => x.to_iced(game),
        }
    }
}

impl ToIced for AlternateTexture {
    fn to_iced(&self, game: &Game) -> Element<Message> {
        let g = Grid::with_columns(2)
            .push(Container::new(text("Object Name")).padding(10))
            .push(self.object_name.to_iced(game))
            .push(Container::new(text("Texture Set")).padding(10))
            .push(self.texture_set.to_iced(game))
            .push(Container::new(text("3D Index")).padding(10))
            .push(self.threed_index.to_iced(game));

        Container::new(g).width(Length::Fill).padding(10).into()
    }
}

impl ToIced for AlternateTextures {
    fn to_iced(&self, game: &Game) -> Element<Message> {
        let g = Grid::with_columns(2)
            .push(Container::new(text("Count")).padding(10))
            .push(self.count.to_iced(game))
            .push(Container::new(text("Textures")).padding(10))
            .push(self.textures.to_iced(game));

        Container::new(g).width(Length::Fill).padding(10).into()
    }
}

impl ToIced for Model {
    fn to_iced(&self, game: &Game) -> Element<Message> {
        let g = Grid::with_columns(2)
            .push(Container::new(text("Model")).padding(10))
            .push(self.model.to_iced(game))
            .push(Container::new(text("Textures")).padding(10))
            .push(self.textures.to_iced(game))
            .push(Container::new(text("Alternate Textures")).padding(10))
            .push(self.alternate_textures.to_iced(game));
        Container::new(g).width(Length::Fill).padding(10).into()
    }
}

impl ToIced for DestructionTexture {
    fn to_iced(&self, game: &Game) -> Element<Message> {
        let g = Grid::with_columns(2)
            .push(Container::new(text("Name")).padding(10))
            .push(self.name.to_iced(game))
            .push(Container::new(text("Texture ID")).padding(10))
            .push(self.texture_id.to_iced(game))
            .push(Container::new(text("Unknown 1")).padding(10))
            .push(self.unknown1.to_iced(game));
        Container::new(g).width(Length::Fill).padding(10).into()
    }
}

impl ToIced for DestructionTextures {
    fn to_iced(&self, game: &Game) -> Element<Message> {
        let g = Grid::with_columns(2)
            .push(Container::new(text("Count")).padding(10))
            .push(self.count.to_iced(game))
            .push(Container::new(text("Textures")).padding(10))
            .push(self.textures.to_iced(game));
        Container::new(g).width(Length::Fill).padding(10).into()
    }
}

impl ToIced for StageDataHeader {
    fn to_iced(&self, game: &Game) -> Element<Message> {
        let g = Grid::with_columns(2)
            .push(Container::new(text("Health Percent")).padding(10))
            .push(self.health_percent.to_iced(game))
            .push(Container::new(text("Index")).padding(10))
            .push(self.index.to_iced(game))
            .push(Container::new(text("Damage Stage")).padding(10))
            .push(self.damage_stage.to_iced(game))
            .push(Container::new(text("Flags")).padding(10))
            .push(self.flags.to_iced(game))
            .push(Container::new(text("Self Damage Rate")).padding(10))
            .push(self.self_damage_rate.to_iced(game))
            .push(Container::new(text("Explosion ID")).padding(10))
            .push(self.explosion_id.to_iced(game))
            .push(Container::new(text("Debris ID")).padding(10))
            .push(self.debris_id.to_iced(game))
            .push(Container::new(text("Debris Count")).padding(10))
            .push(self.debris_count.to_iced(game));
        Container::new(g).width(Length::Fill).padding(10).into()
    }
}

impl ToIced for StageData {
    fn to_iced(&self, game: &Game) -> Element<Message> {
        let g = Grid::with_columns(2)
            .push(Container::new(text("Header")).padding(10))
            .push(self.header.to_iced(game))
            .push(Container::new(text("Replacement Model")).padding(10))
            .push(self.replacement_model.to_iced(game))
            .push(Container::new(text("Unknown 1")).padding(10))
            .push(self.unknown1.to_iced(game))
            .push(Container::new(text("Textures")).padding(10))
            .push(self.destruction_textures.to_iced(game));
        Container::new(g).width(Length::Fill).padding(10).into()
    }
}

impl ToIced for DestructionDataHeader {
    fn to_iced(&self, game: &Game) -> Element<Message> {
        let g = Grid::with_columns(2)
            .push(Container::new(text("Health")).padding(10))
            .push(self.health.to_iced(game))
            .push(Container::new(text("Count")).padding(10))
            .push(self.count.to_iced(game))
            .push(Container::new(text("Flag")).padding(10))
            .push(self.flag.to_iced(game))
            .push(Container::new(text("Unknown 1")).padding(10))
            .push(self.unknown1.to_iced(game))
            .push(Container::new(text("Unknown 2")).padding(10))
            .push(self.unknown2.to_iced(game));
        Container::new(g).width(Length::Fill).padding(10).into()
    }
}

impl ToIced for DestructionData {
    fn to_iced(&self, game: &Game) -> Element<Message> {
        let g = Grid::with_columns(2)
            .push(Container::new(text("Data")).padding(10))
            .push(self.data.to_iced(game))
            .push(Container::new(text("Stage Data")).padding(10))
            .push(self.stage_data.to_iced(game));
        Container::new(g).width(Length::Fill).padding(10).into()
    }
}

impl ToIced for Record {
    fn to_iced(&self, game: &Game) -> Element<Message> {
        match self {
            Record::Group(g) => text(format!(
                "Group - {} items ({})",
                g.records.len(),
                g.magics().join(", ")
            ))
            .width(Length::Fill)
            .horizontal_alignment(Horizontal::Center)
            .into(),
            Record::Book(x) => x.to_iced(game),
            Record::AddonNode(x) => x.to_iced(game),
            Record::AcousticSpace(x) => x.to_iced(game),
            Record::Outfit(x) => x.to_iced(game),
            Record::FormList(x) => x.to_iced(game),
            Record::GameSetting(x) => x.to_iced(game),
            Record::Keyword(x) => x.to_iced(game),
            rec => text(format!("{:?}", rec)).into(),
        }
    }
}
