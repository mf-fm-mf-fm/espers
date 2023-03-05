use crate::Args;
use espers::plugin::Plugin;
use espers::records::{Group, Record};
use iced::{
    executor,
    widget::{button, container, scrollable, scrollable::Properties, text, Column, Row},
    Alignment, Application, Command, Element, Length, Theme,
};
use once_cell::sync::Lazy;
use ron::ser::to_string_pretty;

static SCROLLABLE_ID: Lazy<scrollable::Id> = Lazy::new(scrollable::Id::unique);

pub struct VespersApp {
    current_scroll_offset: scrollable::RelativeOffset,
    plugin: Plugin,
    args: Args,
    state: Vec<usize>,
}

impl VespersApp {
    fn selected(&self) -> Option<&Record> {
        let mut selected: Option<&Record> = None;

        for i in &self.state {
            selected = match selected {
                Some(Record::Group(g)) => Some(&g.records[*i]),
                Some(_) => unreachable!("This should not happen!"),
                None => Some(&self.plugin.records[*i]),
            }
        }
        selected
    }

    fn selected_group(&self) -> Option<&Group> {
        let mut selected: Option<&Group> = None;

        for i in &self.state {
            selected = match selected {
                Some(g) => match &g.records[*i] {
                    Record::Group(g) => Some(g),
                    _ => break,
                },
                None => match &self.plugin.records[*i] {
                    Record::Group(g) => Some(g),
                    _ => break,
                },
            }
        }
        selected
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Scrolled(scrollable::RelativeOffset),
    Click(usize),
    Back,
}

impl Application for VespersApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = (Plugin, Args);

    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        (
            VespersApp {
                current_scroll_offset: scrollable::RelativeOffset::START,
                plugin: flags.0,
                args: flags.1,
                state: Vec::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Vespers")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Click(i) => {
                match self.selected() {
                    Some(Record::Group(_)) | None => self.state.push(i),
                    Some(_) => *self.state.last_mut().unwrap() = i,
                }
                Command::none()
            }
            Message::Scrolled(offset) => {
                self.current_scroll_offset = offset;
                Command::none()
            }
            Message::Back => {
                match self.selected() {
                    Some(Record::Group(_)) | None => {}
                    Some(_) => {
                        self.state.pop();
                    }
                }
                self.state.pop();
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let selected = match self.selected_group() {
            Some(g) => &g.records,
            None => &self.plugin.records,
        };

        let items = selected
            .iter()
            .enumerate()
            .map(|(i, x)| {
                button(text(format!("{}", x)))
                    .on_press(Message::Click(i))
                    .width(Length::Fill)
                    .into()
            })
            .collect();

        let displayed = text(match &self.selected() {
            Some(Record::Group(_)) | None => "Select an item".into(),
            Some(r) => to_string_pretty(r, Default::default()).unwrap(),
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into();

        container(Row::with_children(vec![
            Column::with_children(vec![
                Row::with_children(vec![
                    button(text("Back")).on_press(Message::Back).into(),
                    text(&self.args.path).into(),
                ])
                .spacing(40)
                .align_items(Alignment::Start)
                .width(Length::Fill)
                .into(),
                scrollable(
                    Column::with_children(items)
                        .width(Length::Fill)
                        .align_items(Alignment::Start)
                        .spacing(8),
                )
                .height(Length::Fill)
                .vertical_scroll(Properties::new().scroller_width(10))
                .id(SCROLLABLE_ID.clone())
                .on_scroll(Message::Scrolled)
                .into(),
            ])
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Alignment::Center)
            .spacing(10)
            .into(),
            scrollable(
                Column::with_children(vec![displayed])
                    .width(Length::Fill)
                    .align_items(Alignment::Center)
                    .spacing(10),
            )
            .height(Length::Fill)
            .vertical_scroll(Properties::new().scroller_width(10))
            .into(),
        ]))
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(10)
        .center_x()
        .center_y()
        .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}
