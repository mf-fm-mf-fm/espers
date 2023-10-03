use crate::widgets::ToIced;
use crate::Args;
use espers::game::Game;
use espers::plugin::Plugin;
use espers::records::{Group, Record};
use espers::string_table::TableType;
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
    display_strings: Option<TableType>,
}

impl VespersApp {
    fn get_active_plugin(&self) -> &Plugin {
        &self.game.plugins()[&self.plugin_names[self.selected_plugin]]
    }

    fn selected(&self) -> Option<&Result<Record, espers::error::Error>> {
        let plugin = self.get_active_plugin();
        let mut selected: Option<&Result<Record, espers::error::Error>> = None;

        for i in &self.state {
            selected = match selected {
                Some(Ok(Record::Group(g))) => Some(&g.records[*i]),
                Some(Err(_)) => None,
                Some(_) => unreachable!("This should not happen!"),
                None => Some(&plugin.records[*i]),
            };

            if selected.is_none() {
                break;
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
                    Ok(Record::Group(g)) => Some(g),
                    _ => break,
                },
                None => match &plugin.records[*i] {
                    Ok(Record::Group(g)) => Some(g),
                    _ => break,
                },
            }
        }
        selected
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    LeftPaneScroll(scrollable::Viewport),
    RightPaneScroll(scrollable::Viewport),
    Click(usize),
    Back,
    ToggleTheme,
    ToggleDebugMode,
    TabSelected(usize),
    SetDisplayStrings(Option<TableType>),
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
                display_strings: None,
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
                    Some(Ok(Record::Group(_))) | None => self.state.push(i),
                    Some(_) => *self.state.last_mut().unwrap() = i,
                }
                self.right_scroll_offset = scrollable::RelativeOffset::START;
                scrollable::snap_to(SCROLLABLE_RIGHT.clone(), self.right_scroll_offset)
            }
            Message::LeftPaneScroll(viewport) => {
                self.left_scroll_offset = viewport.relative_offset();
                Command::none()
            }
            Message::RightPaneScroll(viewport) => {
                self.right_scroll_offset = viewport.relative_offset();
                Command::none()
            }
            Message::Back => {
                match self.selected() {
                    Some(Ok(Record::Group(_))) | None => {}
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
            Message::SetDisplayStrings(val) => {
                self.display_strings = val;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let plugin = self.get_active_plugin();

        let items: Vec<Element<Message>> = match self.display_strings {
            Some(table_type) => {
                let plugin_name = &self.plugin_names[self.selected_plugin];
                let strings = self
                    .game
                    .string_tables()
                    .list_strings(plugin_name.into(), table_type);

                match strings {
                    Some(i) => i
                        .iter()
                        .map(|s| button(text(s)).width(Length::Fill).into())
                        .collect(),
                    None => [].into(),
                }
            }
            None => {
                let selected = match self.selected_group() {
                    Some(g) => &g.records,
                    None => &plugin.records,
                };

                selected
                    .iter()
                    .enumerate()
                    .map(|(i, x)| {
                        let t = match x {
                            Ok(Record::Group(g)) => {
                                format!(
                                    "Group - {} items ({})",
                                    g.records.len(),
                                    g.magics().join(", ")
                                )
                            }
                            Ok(other) => {
                                format!("{}", other)
                            }
                            Err(err) => {
                                format!("ERR: {}", err)
                            }
                        };
                        button(text(t))
                            .on_press(Message::Click(i))
                            .width(Length::Fill)
                            .into()
                    })
                    .collect()
            }
        };

        let displayed: Element<Message> = match self.selected() {
            Some(ref rec) => rec.to_iced(&self.game).into(),
            None => text("Select an item")
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Center)
                .into(),
        };

        let mut tab_bar: Vec<_> = self.game.plugins().keys().collect();
        tab_bar.sort();
        let tab_bar = tab_bar
            .into_iter()
            .fold(TabBar::new(Message::TabSelected), |tab_bar, tab_label| {
                let idx = tab_bar.size();
                tab_bar.push(idx, TabLabel::Text(tab_label.clone()))
            })
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

        let assets_row = row![
            button(text("Main Assets")).on_press(Message::SetDisplayStrings(None)),
            button(text("Strings")).on_press(Message::SetDisplayStrings(Some(TableType::STRINGS))),
            button(text("DL Strings"))
                .on_press(Message::SetDisplayStrings(Some(TableType::DLSTRINGS))),
            button(text("IL Strings"))
                .on_press(Message::SetDisplayStrings(Some(TableType::ILSTRINGS)))
        ]
        .spacing(20)
        .padding(10)
        .align_items(Alignment::Start)
        .width(Length::Fill);

        let display_row = row![
            scrollable(
                Column::with_children(items)
                    .align_items(Alignment::Start)
                    .spacing(8),
            )
            .height(Length::Fill)
            .width(Length::Fill)
            .direction(scrollable::Direction::Vertical(Properties::new()))
            .id(SCROLLABLE_LEFT.clone())
            .on_scroll(Message::LeftPaneScroll),
            scrollable(
                column![displayed]
                    .width(Length::Fill)
                    .align_items(Alignment::Start),
            )
            .height(Length::Fill)
            .width(Length::Fill)
            .direction(scrollable::Direction::Vertical(Properties::new()))
            .id(SCROLLABLE_RIGHT.clone())
            .on_scroll(Message::RightPaneScroll),
        ]
        .spacing(20);

        let ui: Element<Message> = container(
            Column::with_children(vec![
                tab_bar.into(),
                container(assets_row)
                    .style(ContainerTheme::Custom(Box::new(ContainerSS)))
                    .into(),
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
