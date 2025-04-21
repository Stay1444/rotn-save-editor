use iced::{
    Background, Border, Element, Length, Task, color,
    widget::{button, column, container, mouse_area, opaque, row, text, text_input},
};

use crate::Message;

pub struct NumericFieldEditorInit {
    pub name: String,
    pub value: u64,
    pub original: u64,
    pub on_save: Box<dyn Fn(u64) -> Message + Send + Sync + 'static>,
}

impl std::fmt::Debug for NumericFieldEditorInit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NumericFieldEditorInit")
    }
}

pub struct NumericFieldEditorState {
    pub name: String,
    pub value: u64,
    pub original: u64,
    pub on_save: Box<dyn Fn(u64) -> Message + Send + Sync + 'static>,
    pub input: String,
    pub error: Option<String>,
}

#[derive(Clone, Debug)]
pub enum NumericFieldEditorMessage {
    EditInput(String),
    Save,
}

impl NumericFieldEditorState {
    pub fn new(init: NumericFieldEditorInit) -> Self {
        Self {
            name: init.name,
            value: init.value,
            original: init.original,
            on_save: init.on_save,
            input: init.value.to_string(),
            error: None,
        }
    }

    pub fn update(&mut self, message: NumericFieldEditorMessage) -> Task<Message> {
        match message {
            NumericFieldEditorMessage::EditInput(value) => match value.parse::<u64>() {
                Ok(valid) => {
                    self.value = valid;
                    self.error = None;
                    self.input = value;
                    Task::none()
                }
                Err(err) => {
                    self.error = Some(format!("Invalid number: {}", err.to_string()));
                    self.input = value;
                    Task::none()
                }
            },
            NumericFieldEditorMessage::Save => {
                if self.error.is_some() {
                    return Task::none();
                }

                let message = (self.on_save)(self.value);

                Task::batch([Task::done(message), Task::done(Message::CloseModal)])
            }
        }
    }
}

pub fn view(state: &NumericFieldEditorState) -> Element<Message> {
    opaque(mouse_area(
        container(
            container(
                column![
                    text(format!("Edit {}", state.name.clone())),
                    row![
                        text_input(&state.original.to_string(), &state.input.clone())
                            .on_input(|x| NumericFieldEditorMessage::EditInput(x).into())
                    ]
                    .spacing(4.0),
                    container(if let Some(error) = state.error.clone() {
                        row![text(error).color(color!(0xFF3333))]
                    } else {
                        row![]
                    }),
                    row![
                        button("Cancel")
                            .on_press(Message::CloseModal)
                            .style(button::danger),
                        button("Save")
                            .on_press_maybe(if state.error.is_none() {
                                Some(NumericFieldEditorMessage::Save.into())
                            } else {
                                None
                            })
                            .style(button::success)
                    ]
                    .spacing(4.0)
                ]
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
            background: Some(Background::Color(color!(33, 33, 33, 0.3))),
            ..Default::default()
        })
        .center(Length::Fill)
        .width(Length::Fill)
        .height(Length::Fill),
    ))
    .into()
}

impl Into<Message> for NumericFieldEditorMessage {
    fn into(self) -> Message {
        Message::NumericEditor(self)
    }
}
