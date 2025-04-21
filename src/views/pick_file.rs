use std::{path::PathBuf, str::FromStr};

use iced::{
    Background, Border, Element, Length, Task, color,
    widget::{button, column, container, row, text},
};
use rfd::AsyncFileDialog;

use crate::{Message, models::SaveGame};

#[derive(Default)]
pub struct PickFileState {
    path: String,
    valid: bool,
    error: Option<String>,
}

#[derive(Clone, Debug)]
pub enum PickFileMessage {
    OpenDialog,
    UserChangedPath(String),
    Submit,
}

impl PickFileState {
    pub fn update(&mut self, message: PickFileMessage) -> Task<Message> {
        match message {
            PickFileMessage::OpenDialog => Task::future(async {
                let file = AsyncFileDialog::new().pick_file().await;
                let Some(file) = file else {
                    return Message::None;
                };

                let path = file.path().to_string_lossy().to_string();
                PickFileMessage::UserChangedPath(path).into()
            }),
            PickFileMessage::UserChangedPath(path) => {
                self.path = path;

                match PathBuf::from_str(&self.path) {
                    Ok(path) => self.valid = path.exists() && path.is_file(),
                    Err(_) => self.valid = false,
                }

                Task::none()
            }
            PickFileMessage::Submit => {
                let Ok(path) = PathBuf::from_str(&self.path);

                let content = match std::fs::read_to_string(&path) {
                    Ok(x) => x,
                    Err(err) => {
                        self.error = Some(err.to_string());
                        return Task::none();
                    }
                };

                let data: SaveGame = match serde_json::from_str(&content) {
                    Ok(x) => x,
                    Err(err) => {
                        self.error = Some(format!("Error parsing json: {}", err.to_string()));
                        return Task::none();
                    }
                };

                Task::done(Message::Loaded(data, path))
            }
        }
    }
}

pub fn view(state: &PickFileState) -> Element<Message> {
    container(
        container(
            column![
                text("Pick SaveGame file location"),
                row![
                    iced::widget::text_input("Path", state.path.as_str())
                        .on_input(|t| PickFileMessage::UserChangedPath(t).into())
                        .on_submit(PickFileMessage::Submit.into()),
                    button("Pick")
                        .style(button::secondary)
                        .on_press(PickFileMessage::OpenDialog.into())
                ]
                .spacing(4.0),
                button("Open").on_press_maybe(if state.valid {
                    Some(PickFileMessage::Submit.into())
                } else {
                    None
                })
            ]
            .push_maybe(if let Some(error) = state.error.clone() {
                text(error).color(color!(0xFF0000)).into()
            } else {
                None
            })
            .spacing(8.0),
        )
        .style(|_| container::Style {
            shadow: iced::Shadow {
                color: color!(0x333333),
                offset: iced::Vector { x: 4.0, y: 4.0 },
                blur_radius: 8.0,
            },
            border: Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            background: Some(Background::Color(color!(0xFFFFFF))),
            ..Default::default()
        })
        .max_width(600.0)
        .padding(8.0),
    )
    .style(|_| container::Style {
        background: Some(Background::Color(color!(0xBBBBBB))),
        ..Default::default()
    })
    .center(Length::Fill)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

impl Into<Message> for PickFileMessage {
    fn into(self) -> Message {
        Message::PickFile(self)
    }
}
