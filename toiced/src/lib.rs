pub use iced;
use iced::{
    widget::{text, Column, Container},
    Element, Length,
};

pub trait ToIced<'a, Message> {
    fn to_iced(&self) -> Element<'a, Message>;
}

impl<'a, Message: 'a> ToIced<'a, Message> for f32 {
    fn to_iced(&self) -> Element<'a, Message> {
        Container::new(text(self))
            .width(Length::Fill)
            .padding(10)
            .into()
    }
}

impl<'a, Message: 'a> ToIced<'a, Message> for u8 {
    fn to_iced(&self) -> Element<'a, Message> {
        Container::new(text(self))
            .width(Length::Fill)
            .padding(10)
            .into()
    }
}

impl<'a, Message: 'a> ToIced<'a, Message> for u16 {
    fn to_iced(&self) -> Element<'a, Message> {
        Container::new(text(self))
            .width(Length::Fill)
            .padding(10)
            .into()
    }
}

impl<'a, Message: 'a> ToIced<'a, Message> for i16 {
    fn to_iced(&self) -> Element<'a, Message> {
        Container::new(text(self))
            .width(Length::Fill)
            .padding(10)
            .into()
    }
}

impl<'a, Message: 'a> ToIced<'a, Message> for u32 {
    fn to_iced(&self) -> Element<'a, Message> {
        Container::new(text(format!("0x{:08X}", self)))
            .width(Length::Fill)
            .padding(10)
            .into()
    }
}

impl<'a, Message: 'a> ToIced<'a, Message> for String {
    fn to_iced(&self) -> Element<'a, Message> {
        Container::new(text(self))
            .width(Length::Fill)
            .padding(10)
            .into()
    }
}

// impl ToIced for FormID {
//     fn to_iced(&self, plugin: &Plugin) -> Element<Message> {
//         let x = match plugin.get_record_by_form_id(self) {
//             Some(r) => format!("{}", r),
//             None => "<not set>".into(),
//         };
//         Container::new(text(x))
//             .width(Length::Fill)
//             .padding(10)
//             .into()
//     }
// }

// pl ToIced for ScriptList {
//     fn to_iced(&self, _: &Plugin) -> Element<Message> {
//         Container::new(text(format!("{:?}", self)))
//             .width(Length::Fill)
//             .padding(10)
//             .into()
//     }
// }

// impl ToIced for BookFlags {
//     fn to_iced(&self, _: &Plugin) -> Element<Message> {
//         Container::new(text(format!("{:?}", self)))
//             .width(Length::Fill)
//             .padding(10)
//             .into()
//     }
// }

// impl ToIced for BookData {
//     fn to_iced(&self, plugin: &Plugin) -> Element<Message> {
//         let g = Grid::with_columns(2)
//             .push(Container::new(text("Flags")).padding(10))
//             .push(self.flags.to_iced(plugin))
//             .push(Container::new(text("Type")).padding(10))
//             .push(self.kind.to_iced(plugin))
//             .push(Container::new(text("Unknown")).padding(10))
//             .push(self.unknown.to_iced(plugin))
//             .push(Container::new(text("Teaches")).padding(10))
//             .push(self.teaches.to_iced(plugin))
//             .push(Container::new(text("Value")).padding(10))
//             .push(self.value.to_iced(plugin))
//             .push(Container::new(text("Weight")).padding(10))
//             .push(self.weight.to_iced(plugin));

//         Container::new(g).width(Length::Fill).padding(10).into()
//     }
// }

// impl ToIced for Unknown4 {
//     fn to_iced(&self, plugin: &Plugin) -> Element<Message> {
//         let g = Grid::with_columns(2)
//             .push(Container::new(text("Unknown 1")).padding(10))
//             .push(self.unknown1.to_iced(plugin))
//             .push(Container::new(text("Unknown 2")).padding(10))
//             .push(self.unknown2.to_iced(plugin))
//             .push(Container::new(text("Unknown 3")).padding(10))
//             .push(self.unknown3.to_iced(plugin));

//         Container::new(g).width(Length::Fill).padding(10).into()
//     }
// }

// impl ToIced for ModelTextures {
//     fn to_iced(&self, plugin: &Plugin) -> Element<Message> {
//         let g = Grid::with_columns(2)
//             .push(Container::new(text("Count")).padding(10))
//             .push(self.count.to_iced(plugin))
//             .push(Container::new(text("Unknown4 Count")).padding(10))
//             .push(self.unknown4_count.to_iced(plugin))
//             .push(Container::new(text("Unknown2")).padding(10))
//             .push(self.unknown2.to_iced(plugin))
//             .push(Container::new(text("Unknown4s")).padding(10))
//             .push(self.unknown4s.to_iced(plugin));

//         Container::new(g).width(Length::Fill).padding(10).into()
//     }
// }
impl<'a, Message: 'a, T> ToIced<'a, Message> for Vec<T>
where
    T: ToIced<'a, Message>,
{
    fn to_iced(&self) -> Element<'a, Message> {
        Column::with_children(self.iter().map(|x| x.to_iced()).collect())
            .width(Length::Fill)
            .padding(10)
            .into()
    }
}

impl<'a, Message: 'a, T, const N: usize> ToIced<'a, Message> for [T; N]
where
    T: ToIced<'a, Message>,
{
    fn to_iced(&self) -> Element<'a, Message> {
        Column::with_children(self.iter().map(|x| x.to_iced()).collect())
            .width(Length::Fill)
            .padding(10)
            .into()
    }
}
impl<'a, Message: 'a, T> ToIced<'a, Message> for Option<T>
where
    T: ToIced<'a, Message>,
{
    fn to_iced(&self) -> Element<'a, Message> {
        match self {
            Some(s) => s.to_iced(),
            None => Container::new(text("<not set>"))
                .width(Length::Fill)
                .padding(10)
                .into(),
        }
    }
}
