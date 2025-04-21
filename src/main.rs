use std::{path::PathBuf, sync::Arc};

use iced::{Element, Task, widget::stack};
use modals::{
    Modal,
    numeric_field_editor::{
        NumericFieldEditorInit, NumericFieldEditorMessage, NumericFieldEditorState,
    },
};
use models::SaveGame;
use views::{
    View,
    editor::{EditorMessage, EditorState},
    pick_file::PickFileMessage,
};

include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

pub mod modals;
pub mod models;
mod views;

#[derive(Default)]
struct Application {
    view: View,
    modal: Option<Modal>,
}

impl Application {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::None => Task::none(),
            Message::Init => Task::none(),
            Message::PickFile(message) => match &mut self.view {
                View::PickFile(state) => state.update(message),
                _ => unreachable!(),
            },
            Message::Editor(message) => match &mut self.view {
                View::Editor(state) => state.update(message),
                _ => unreachable!(),
            },
            Message::Loaded(save_game, path) => {
                self.view = View::Editor(EditorState::new(save_game, path));
                Task::none()
            }
            Message::NumericEditor(message) => match &mut self.modal {
                Some(Modal::EditNumericField(state)) => state.update(message),
                _ => unreachable!(),
            },
            Message::OpenNumericEditor(init) => {
                let init = match Arc::try_unwrap(init) {
                    Ok(x) => x,
                    Err(_) => return Task::none(),
                };

                self.modal = Some(Modal::EditNumericField(NumericFieldEditorState::new(init)));

                Task::none()
            }
            Message::CloseModal => {
                self.modal = None;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let view = match &self.view {
            View::PickFile(state) => views::pick_file::view(state),
            View::Editor(state) => views::editor::view(state),
        };

        let modal = match self.modal.as_ref() {
            Some(Modal::EditNumericField(state)) => {
                Some(modals::numeric_field_editor::view(&state))
            }
            None => None,
        };

        stack([view]).push_maybe(modal).into()
    }

    pub fn init() -> (Application, Task<Message>) {
        (Default::default(), Task::done(Message::Init))
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    None,
    Init,
    Loaded(SaveGame, PathBuf),
    PickFile(PickFileMessage),
    Editor(EditorMessage),
    NumericEditor(NumericFieldEditorMessage),
    OpenNumericEditor(Arc<NumericFieldEditorInit>),
    CloseModal,
}

fn main() {
    println!(
        r#"HASH = "{}",
BUILD_DATE = "{}",
TARGET_OS = "{}",
RUSTC_VERSION = "{}",
PROFILE = "{}""#,
        BuildInfo::GIT_HASH,
        BuildInfo::BUILD_DATE,
        BuildInfo::TARGET_OS,
        BuildInfo::RUSTC_VERSION,
        BuildInfo::PROFILE
    );

    tracing_subscriber::fmt::init();

    tracing::info!("Starting");

    if let Err(err) = iced::application(Application::init, Application::update, Application::view)
        .antialiasing(true)
        .title("Rift Of The Necrodancer | Save Editor")
        .run()
    {
        tracing::error!("Error running application: {}", err);
    }
}
