use crate::widgets::ToIced;
use crate::Args;
use espers::game::Game;
use espers::plugin::Plugin;
use espers::records::{Group, Record};
use iced::{
    alignment::Horizontal,
    event, executor, keyboard,
    keyboard::KeyCode,
    subscription,
    theme::Container as ContainerTheme,
    widget::{button, column, container, row, scrollable, scrollable::Properties, text, Column},
    Alignment, Application, Background, Color, Command, Element, Event, Length, Subscription,
    Theme,
};
use iced_aw::{TabBar, TabLabel};
use once_cell::sync::Lazy;

static SCROLLABLE_LEFT: Lazy<scrollable::Id> = Lazy::new(scrollable::Id::unique);
static SCROLLABLE_RIGHT: Lazy<scrollable::Id> = Lazy::new(scrollable::Id::unique);

pub struct ContainerSS;

impl container::StyleSheet for ContainerSS {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::new(0.5, 0.5, 0.5, 0.3))),
            ..style.appearance(&ContainerTheme::Box)
        }
    }
}

pub struct VespersApp {
    left_scroll_offset: scrollable::RelativeOffset,
    right_scroll_offset: scrollable::RelativeOffset,
    game: Game,
    args: Args,
    state: Vec<usize>,
    theme: Theme,
    debug_mode: bool,
    plugin_names: Vec<String>,
    selected_plugin: usize,
}

impl VespersApp {
    fn get_active_plugin(&self) -> &Plugin {
        &self.game.plugins()[&self.plugin_names[self.selected_plugin]]
    }

    fn selected(&self) -> Option<&Record> {
        let plugin = self.get_active_plugin();
        let mut selected: Option<&Record> = None;

        for i in &self.state {
            selected = match selected {
                Some(Record::Group(g)) => Some(&g.records[*i]),
                Some(_) => unreachable!("This should not happen!"),
                None => Some(&plugin.records[*i]),
            }
        }
        selected
    }

    fn selected_group(&self) -> Option<&Group> {
        let plugin = self.get_active_plugin();
        let mut selected: Option<&Group> = None;

        for i in &self.state {
            selected = match selected {
                Some(g) => match &g.records[*i] {
                    Record::Group(g) => Some(g),
                    _ => break,
                },
                None => match &plugin.records[*i] {
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
    LeftPaneScroll(scrollable::RelativeOffset),
    RightPaneScroll(scrollable::RelativeOffset),
    Click(usize),
    Back,
    ToggleTheme,
    ToggleDebugMode,
    TabSelected(usize),
}

impl Application for VespersApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = (Game, Args);

    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        let mut plugin_names: Vec<_> = flags.0.plugins().keys().cloned().collect();
        plugin_names.sort();
        (
            VespersApp {
                left_scroll_offset: scrollable::RelativeOffset::START,
                right_scroll_offset: scrollable::RelativeOffset::START,
                game: flags.0,
                args: flags.1,
                state: Vec::new(),
                theme: Theme::Dark,
                debug_mode: false,
                plugin_names,
                selected_plugin: 0,
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
                self.right_scroll_offset = scrollable::RelativeOffset::START;
                scrollable::snap_to(SCROLLABLE_RIGHT.clone(), self.right_scroll_offset)
            }
            Message::LeftPaneScroll(offset) => {
                self.left_scroll_offset = offset;
                Command::none()
            }
            Message::RightPaneScroll(offset) => {
                self.right_scroll_offset = offset;
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
                self.left_scroll_offset = scrollable::RelativeOffset::START;
                scrollable::snap_to(SCROLLABLE_LEFT.clone(), self.left_scroll_offset)
            }
            Message::ToggleTheme => {
                self.theme = match self.theme {
                    Theme::Light => Theme::Dark,
                    Theme::Dark => Theme::Light,
                    _ => Theme::Dark,
                };
                Command::none()
            }
            Message::ToggleDebugMode => {
                self.debug_mode = !self.debug_mode;
                Command::none()
            }
            Message::TabSelected(idx) => {
                self.selected_plugin = idx;
                self.state = Vec::new();
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let plugin = self.get_active_plugin();
        let selected = match self.selected_group() {
            Some(g) => &g.records,
            None => &plugin.records,
        };

        let items: Vec<Element<Message>> = selected
            .iter()
            .enumerate()
            .map(|(i, x)| {
                button(text(if let Record::Group(g) = x {
                    format!(
                        "Group - {} items ({})",
                        g.records.len(),
                        g.magics().join(", ")
                    )
                } else {
                    format!("{}", x)
                }))
                .on_press(Message::Click(i))
                .width(Length::Fill)
                .into()
            })
            .collect();

        let displayed: Element<Message> = match self.selected() {
            Some(ref rec) => rec.to_iced(&self.game).into(),
            None => text("Select an item")
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Center)
                .into(),
        };

        let tab_bar = self
            .game
            .plugins()
            .keys()
            .fold(
                TabBar::new(self.selected_plugin, |x| Message::TabSelected(x)),
                |tab_bar, name| tab_bar.push(TabLabel::Text(name.clone())),
            )
            .tab_width(Length::Shrink)
            .spacing(8.0)
            .padding(8.0)
            .text_size(24.0);

        let title_row = row![
            button(text("Back")).on_press(Message::Back),
            text(&self.args.paths.join(", ")).width(Length::Fill),
            text(&format!(
                "Author: {}",
                plugin.header.author.as_deref().unwrap_or("<not set>")
            ))
            .width(Length::Fill),
            text(&format!(
                "Description: {}",
                plugin.header.description.as_deref().unwrap_or("<not set>")
            ))
            .width(Length::Fill),
        ]
        .spacing(20)
        .padding(10)
        .align_items(Alignment::Start)
        .width(Length::Fill);

        let display_row = row![
            scrollable(
                Column::with_children(items)
                    .width(Length::Fill)
                    .align_items(Alignment::Start)
                    .spacing(8),
            )
            .height(Length::Fill)
            .vertical_scroll(Properties::new().scroller_width(10))
            .id(SCROLLABLE_LEFT.clone())
            .on_scroll(Message::LeftPaneScroll),
            scrollable(
                column![displayed]
                    .width(Length::Fill)
                    .align_items(Alignment::Start),
            )
            .height(Length::Fill)
            .vertical_scroll(Properties::new().scroller_width(10))
            .id(SCROLLABLE_RIGHT.clone())
            .on_scroll(Message::RightPaneScroll),
        ]
        .spacing(20);

        let ui: Element<Message> = container(
            Column::with_children(vec![
                tab_bar.into(),
                container(title_row)
                    .style(ContainerTheme::Custom(Box::new(ContainerSS)))
                    .into(),
                display_row.into(),
            ])
            .spacing(20),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(10)
        .center_x()
        .center_y()
        .into();

        if self.debug_mode {
            ui.explain(self.theme.palette().primary)
        } else {
            ui
        }
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events_with(|event, status| {
            if let event::Status::Captured = status {
                return None;
            }

            match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key_code: KeyCode::T,
                    ..
                }) => Some(Message::ToggleTheme),
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key_code: KeyCode::D,
                    ..
                }) => Some(Message::ToggleDebugMode),
                _ => None,
            }
        })
    }
}
