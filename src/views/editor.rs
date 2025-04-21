use std::{
    borrow::Cow,
    path::PathBuf,
    str::FromStr,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use iced::{
    Element, Length, Task,
    alignment::Vertical,
    color,
    widget::{button, checkbox, column, container, responsive, row, scrollable, text, text_input},
};

use crate::{
    Message,
    modals::numeric_field_editor::NumericFieldEditorInit,
    models::{DifficultyData, LevelData, SaveGame},
};

pub struct EditorState {
    data: SaveGame,
    original: SaveGame,
    path: PathBuf,
}

impl EditorState {
    pub fn new(save: SaveGame, path: PathBuf) -> Self {
        EditorState {
            data: save.clone(),
            original: save,
            path,
        }
    }
}

#[derive(Clone, Debug)]
pub enum EditorMessage {
    Save,
    MarkAllFullCombo,
    EditSaveName(String),
    EditPlayerID(String),
    EditSelectedLanguage(String),
    EditShouldDisplayDialogueDebug(bool),
    EditShouldUnlockAllLevels(bool),
    EditHasInputDragonDance(bool),
    EditIsRemixModeActive(bool),
    EditShouldPlayAllStoryContentInOrder(bool),
    EditGameDataVersion(u64),
    EditSaveDataVersion(u64),
    EditTimesBooted(u64),
    EditSaveId(u64),
    EditFramerateLimit(u64),
    EditSelectedStoryDifficulty(u64),
    EditSelectedArcadeDifficulty(u64),
    EditTotalRhythmRiftsCleared(u64),
    EditTotalDiamonds(u64),
    EditVibePowerUses(u64),
    EditMaxEnemiesKilledWhileVibing(u64),
    EditBBTotalAttacks(u64),
    EditBBTotalDodges(u64),
    EditBBTotalBlockedHits(u64),
    LevelEdit {
        index: usize,
        message: LevelEditMessage,
    },
}

impl EditorState {
    pub fn update(&mut self, message: EditorMessage) -> Task<Message> {
        match message {
            EditorMessage::EditSaveName(value) => self.data.save_name = value,
            EditorMessage::EditPlayerID(value) => self.data.player_id = value,
            EditorMessage::EditSelectedLanguage(value) => self.data.selected_language = value,
            EditorMessage::EditShouldDisplayDialogueDebug(value) => {
                self.data.should_display_dialogue_debug = value
            }
            EditorMessage::EditShouldUnlockAllLevels(value) => {
                self.data.should_unlock_all_levels = value;
            }
            EditorMessage::EditHasInputDragonDance(value) => {
                self.data.has_input_dragon_dance = value;
            }
            EditorMessage::EditIsRemixModeActive(value) => {
                self.data.is_remix_mode_active = value;
            }
            EditorMessage::EditShouldPlayAllStoryContentInOrder(value) => {
                self.data.should_play_all_story_content_in_order = value;
            }
            EditorMessage::EditGameDataVersion(value) => self.data.game_data_version = value,
            EditorMessage::EditSaveDataVersion(value) => self.data.save_data_version = value,
            EditorMessage::EditTimesBooted(value) => self.data.times_booted = value,
            EditorMessage::EditSaveId(value) => self.data.save_id = value,
            EditorMessage::EditFramerateLimit(value) => self.data.framerate_limit = value,
            EditorMessage::EditSelectedStoryDifficulty(value) => {
                self.data.selected_story_difficulty = value
            }
            EditorMessage::EditSelectedArcadeDifficulty(value) => {
                self.data.selected_arcade_difficulty = value
            }
            EditorMessage::EditTotalRhythmRiftsCleared(value) => {
                self.data.total_rhythm_rifts_cleared = value
            }
            EditorMessage::EditTotalDiamonds(value) => self.data.total_diamonds = value,
            EditorMessage::EditVibePowerUses(value) => self.data.total_vibe_power_uses = value,
            EditorMessage::EditMaxEnemiesKilledWhileVibing(value) => {
                self.data.max_enemies_killed_while_vibing = value
            }
            EditorMessage::EditBBTotalAttacks(value) => self.data.bb_total_attacks = value,
            EditorMessage::EditBBTotalDodges(value) => self.data.bb_total_dodges = value,
            EditorMessage::EditBBTotalBlockedHits(value) => self.data.bb_total_blocked_hits = value,
            EditorMessage::LevelEdit { index, message } => {
                let Some(level) = self.data.level_data.get_mut(index) else {
                    return Task::none();
                };

                match message {
                    LevelEditMessage::EditLevelId(value) => level.level_id = value,
                    LevelEditMessage::EditStageType(value) => level.stage_type = value,
                    LevelEditMessage::EditWasCompletedInStoryMode(value) => {
                        level.was_completed_in_story_mode = value
                    }
                    LevelEditMessage::EditWasAttemptedInStoryMode(value) => {
                        level.was_attempted_in_story_mode = value
                    }
                    LevelEditMessage::EditWasSkippedInStoryMode(value) => {
                        level.was_skipped_in_story_mode = value
                    }
                    LevelEditMessage::EditAwardedDiamonds(value) => level.awarded_diamonds = value,
                    LevelEditMessage::EditAwardedDiamondsRemix(value) => {
                        level.awarded_diamonds_remix = value
                    }
                    LevelEditMessage::EditHighScoreData { index, message } => {
                        let Some(highscore) = level.difficulty_data.get_mut(index) else {
                            return Task::none();
                        };

                        match message {
                            LevelEditHighScoreDataMessage::EditDifficulty(value) => {
                                highscore.difficulty = value
                            }
                            LevelEditHighScoreDataMessage::EditHighScore(value) => {
                                highscore.high_score = value
                            }
                            LevelEditHighScoreDataMessage::EditLetterGrade(value) => {
                                highscore.letter_grade = value
                            }
                            LevelEditHighScoreDataMessage::EditMaxComboCount(value) => {
                                highscore.max_combo_count = value
                            }
                            LevelEditHighScoreDataMessage::EditNumAttempts(value) => {
                                highscore.num_attempts = value
                            }
                            LevelEditHighScoreDataMessage::EditNumClears(value) => {
                                highscore.num_clears = value
                            }
                            LevelEditHighScoreDataMessage::EditNumRetries(value) => {
                                highscore.num_retries = value
                            }
                            LevelEditHighScoreDataMessage::EditNumGameOvers(value) => {
                                highscore.num_game_overs = value
                            }
                            LevelEditHighScoreDataMessage::EditHasAllPerfects(value) => {
                                highscore.has_all_perfects = value
                            }
                            LevelEditHighScoreDataMessage::EditHasFullComboRhythmRift(value) => {
                                highscore.has_full_combo_rhythm_rift = value
                            }
                        }
                    }
                }
            }
            EditorMessage::Save => {
                let target = PathBuf::from_str(&format!(
                    "{}.editor.{}.bak",
                    self.path.clone().to_string_lossy(),
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards")
                        .as_millis()
                ));

                _ = std::fs::copy(&self.path, &target.unwrap());

                let content = serde_json::to_string(&self.data).expect("Serialization error");

                self.original = self.data.clone();

                std::fs::write(&self.path, content).expect("Failed to write file");
            }
            EditorMessage::MarkAllFullCombo => {
                for level in &mut self.data.level_data {
                    level.was_attempted_in_story_mode = true;
                    level.was_completed_in_story_mode = true;
                    for difficulty in &mut level.difficulty_data {
                        difficulty.num_attempts = difficulty.num_attempts.min(1);
                        difficulty.num_retries = difficulty.num_retries.min(1);
                        difficulty.has_all_perfects = true;
                        difficulty.has_full_combo_rhythm_rift = true;
                    }
                }
            }
        }
        Task::none()
    }
}

pub fn view(state: &EditorState) -> Element<Message> {
    container(
        row![
            scrollable(
                column![
                    text("General").size(22.0),
                    str_field(
                        "SaveName".into(),
                        &state.data.save_name,
                        &state.original.save_name,
                        EditorMessage::EditSaveName
                    ),
                    str_field(
                        "PlayerID".into(),
                        &state.data.player_id,
                        &state.original.player_id,
                        EditorMessage::EditPlayerID
                    ),
                    str_field(
                        "Selected Language".into(),
                        &state.data.selected_language,
                        &state.original.selected_language,
                        EditorMessage::EditSelectedLanguage
                    ),
                    bool_field(
                        "Should Display Dialogue Debug".into(),
                        state.data.should_display_dialogue_debug,
                        state.original.should_display_dialogue_debug,
                        EditorMessage::EditShouldDisplayDialogueDebug
                    ),
                    bool_field(
                        "Should Unlock All Levels".into(),
                        state.data.should_unlock_all_levels,
                        state.original.should_unlock_all_levels,
                        EditorMessage::EditShouldUnlockAllLevels
                    ),
                    bool_field(
                        "Has Input Dragon Dance".into(),
                        state.data.has_input_dragon_dance,
                        state.original.has_input_dragon_dance,
                        EditorMessage::EditHasInputDragonDance
                    ),
                    bool_field(
                        "Is Remix Mode Active".into(),
                        state.data.is_remix_mode_active,
                        state.original.is_remix_mode_active,
                        EditorMessage::EditIsRemixModeActive
                    ),
                    bool_field(
                        "Should Play All Story Content In Order".into(),
                        state.data.should_play_all_story_content_in_order,
                        state.original.should_play_all_story_content_in_order,
                        EditorMessage::EditShouldPlayAllStoryContentInOrder
                    ),
                    num_field(
                        "Game Data Version".into(),
                        state.data.game_data_version,
                        state.original.game_data_version,
                        EditorMessage::EditGameDataVersion
                    ),
                    num_field(
                        "Save Data Version".into(),
                        state.data.save_data_version,
                        state.original.save_data_version,
                        EditorMessage::EditSaveDataVersion
                    ),
                    num_field(
                        "Times Booted".into(),
                        state.data.times_booted,
                        state.original.times_booted,
                        EditorMessage::EditTimesBooted
                    ),
                    num_field(
                        "Save ID".into(),
                        state.data.save_id,
                        state.original.save_id,
                        EditorMessage::EditSaveId
                    ),
                    num_field(
                        "Framerate Limit".into(),
                        state.data.framerate_limit,
                        state.original.framerate_limit,
                        EditorMessage::EditFramerateLimit
                    ),
                    num_field(
                        "Selected Story Difficulty".into(),
                        state.data.selected_story_difficulty,
                        state.original.selected_story_difficulty,
                        EditorMessage::EditSelectedStoryDifficulty
                    ),
                    num_field(
                        "Selected Arcade Difficulty".into(),
                        state.data.selected_arcade_difficulty,
                        state.original.selected_arcade_difficulty,
                        EditorMessage::EditSelectedArcadeDifficulty
                    ),
                    num_field(
                        "Total Rythm Rifts Cleared".into(),
                        state.data.total_rhythm_rifts_cleared,
                        state.original.total_rhythm_rifts_cleared,
                        EditorMessage::EditTotalRhythmRiftsCleared
                    ),
                    num_field(
                        "Total Diamonds".into(),
                        state.data.total_diamonds,
                        state.original.total_diamonds,
                        EditorMessage::EditTotalDiamonds
                    ),
                    num_field(
                        "Total Vibe Power Uses".into(),
                        state.data.total_vibe_power_uses,
                        state.original.total_vibe_power_uses,
                        EditorMessage::EditVibePowerUses
                    ),
                    num_field(
                        "Max Enemies Killed While Vibing".into(),
                        state.data.max_enemies_killed_while_vibing,
                        state.original.max_enemies_killed_while_vibing,
                        EditorMessage::EditMaxEnemiesKilledWhileVibing
                    ),
                    num_field(
                        "BB Total Attacks".into(),
                        state.data.bb_total_attacks,
                        state.original.bb_total_attacks,
                        EditorMessage::EditBBTotalAttacks
                    ),
                    num_field(
                        "BB Total Dodges".into(),
                        state.data.bb_total_dodges,
                        state.original.bb_total_dodges,
                        EditorMessage::EditBBTotalDodges
                    ),
                    num_field(
                        "BB Total Blocked Hits".into(),
                        state.data.bb_total_blocked_hits,
                        state.original.bb_total_blocked_hits,
                        EditorMessage::EditBBTotalBlockedHits
                    ),
                    text("Levels").size(22.0),
                    column(
                        state
                            .data
                            .level_data
                            .iter()
                            .enumerate()
                            .map(|(id, level)| level_edit(
                                id,
                                level,
                                state.original.level_data.get(id).unwrap()
                            ))
                    )
                    .spacing(8.0)
                ]
                .spacing(8.0),
            )
            .spacing(4.0)
            .width(Length::Fill)
            .height(Length::Fill),
            column![
                text("Actions").size(22.0),
                scrollable(
                    column![
                        button("Mark all as Full Combo")
                            .width(Length::Fill)
                            .on_press(EditorMessage::MarkAllFullCombo.into())
                    ]
                    .padding(2.0)
                )
                .spacing(4.0)
                .height(Length::Fill)
                .width(Length::Fill),
                container(responsive(|size| button("Save")
                    .style(button::success)
                    .width(Length::Fixed(size.width))
                    .height(Length::Shrink)
                    .on_press(EditorMessage::Save.into())
                    .into()))
                .width(Length::Fill)
                .height(Length::Fixed(45.0)),
                row![
                    text("Saving in"),
                    text(state.path.display().to_string()).color(color!(1, 1, 1, 0.8))
                ]
                .spacing(2.0),
            ]
            .height(Length::Fill)
            .width(Length::Shrink)
            .max_width(350.0)
        ]
        .height(Length::Fill)
        .spacing(4.0),
    )
    .padding(12.0)
    .into()
}

impl Into<Message> for EditorMessage {
    fn into(self) -> Message {
        Message::Editor(self)
    }
}

pub fn str_field<'a>(
    name: Cow<'a, str>,
    value: impl Into<String>,
    placeholder: impl Into<String>,
    change: impl Fn(String) -> EditorMessage + 'a,
) -> Element<'a, Message> {
    let current = value.into();
    row![
        text(name.clone()),
        text_input(&placeholder.into(), &current).on_input(move |x| change(x).into())
    ]
    .align_y(Vertical::Center)
    .spacing(8.0)
    .into()
}

pub fn num_field<'a>(
    name: Cow<'a, str>,
    value: u64,
    original: u64,
    change: impl Fn(u64) -> EditorMessage + Sync + Send + 'static,
) -> Element<'a, Message> {
    row![
        text(name.clone()),
        button(text(value.to_string()))
            .style(button::secondary)
            .on_press(Message::OpenNumericEditor(Arc::new(
                NumericFieldEditorInit {
                    name: name.to_string(),
                    value,
                    original,
                    on_save: Box::new(move |x| { change(x).into() })
                }
            ))),
        text(original.to_string()).color(color!(0x999999))
    ]
    .align_y(Vertical::Center)
    .spacing(8.0)
    .into()
}

pub fn bool_field<'a>(
    name: Cow<'a, str>,
    value: bool,
    original: bool,
    change: impl Fn(bool) -> EditorMessage + 'a,
) -> Element<'a, Message> {
    row![
        text(name),
        checkbox("", value).on_toggle(move |x| change(x).into()),
        checkbox("", original),
    ]
    .align_y(Vertical::Center)
    .spacing(8.0)
    .into()
}

#[derive(Clone, Debug)]
pub enum LevelEditMessage {
    EditLevelId(String),
    EditStageType(u64),
    EditWasCompletedInStoryMode(bool),
    EditWasAttemptedInStoryMode(bool),
    EditWasSkippedInStoryMode(bool),
    EditAwardedDiamonds(u64),
    EditAwardedDiamondsRemix(u64),
    EditHighScoreData {
        index: usize,
        message: LevelEditHighScoreDataMessage,
    },
}

#[derive(Clone, Debug)]
pub enum LevelEditHighScoreDataMessage {
    EditDifficulty(u64),
    EditHighScore(u64),
    EditLetterGrade(String),
    EditMaxComboCount(u64),
    EditNumAttempts(u64),
    EditNumClears(u64),
    EditNumRetries(u64),
    EditNumGameOvers(u64),
    EditHasAllPerfects(bool),
    EditHasFullComboRhythmRift(bool),
}

pub fn level_edit<'a>(
    index: usize,
    level: &'a LevelData,
    original: &'a LevelData,
) -> Element<'a, Message> {
    container(column![
        str_field(
            "Level Id".into(),
            level.level_id.clone(),
            original.level_id.clone(),
            move |x| EditorMessage::LevelEdit {
                index,
                message: LevelEditMessage::EditLevelId(x)
            }
        ),
        num_field(
            "Stage Type".into(),
            level.stage_type,
            original.stage_type,
            move |x| EditorMessage::LevelEdit {
                index,
                message: LevelEditMessage::EditStageType(x)
            }
        ),
        bool_field(
            "Was Completed In Story Mode".into(),
            level.was_completed_in_story_mode,
            original.was_completed_in_story_mode,
            move |x| EditorMessage::LevelEdit {
                index: index,
                message: LevelEditMessage::EditWasCompletedInStoryMode(x)
            }
        ),
        bool_field(
            "Was Attempted In Story Mode".into(),
            level.was_attempted_in_story_mode,
            original.was_attempted_in_story_mode,
            move |x| EditorMessage::LevelEdit {
                index: index,
                message: LevelEditMessage::EditWasAttemptedInStoryMode(x)
            }
        ),
        bool_field(
            "Was Skipped In Story Mode".into(),
            level.was_skipped_in_story_mode,
            original.was_skipped_in_story_mode,
            move |x| EditorMessage::LevelEdit {
                index: index,
                message: LevelEditMessage::EditWasSkippedInStoryMode(x)
            }
        ),
        num_field(
            "Awarded Diamonds".into(),
            level.awarded_diamonds,
            original.awarded_diamonds,
            move |x| EditorMessage::LevelEdit {
                index,
                message: LevelEditMessage::EditAwardedDiamonds(x)
            }
        ),
        num_field(
            "Awarded Diamonds Remix".into(),
            level.awarded_diamonds_remix,
            original.awarded_diamonds_remix,
            move |x| EditorMessage::LevelEdit {
                index,
                message: LevelEditMessage::EditAwardedDiamondsRemix(x)
            }
        ),
        text("High Score Data"),
        column(
            level
                .difficulty_data
                .iter()
                .enumerate()
                .map(|(id, level)| high_score_data_edit(
                    index,
                    id,
                    level,
                    original.difficulty_data.get(id).unwrap()
                ))
        )
        .spacing(8.0)
    ])
    .padding(8.0)
    .style(|_| container::Style {
        border: iced::Border {
            color: color!(0x222222),
            width: 1.0.into(),
            radius: 8.0.into(),
        },
        ..Default::default()
    })
    .into()
}

fn high_score_data_edit<'a>(
    level_index: usize,
    index: usize,
    data: &DifficultyData,
    original: &DifficultyData,
) -> Element<'a, Message> {
    container(column![
        num_field(
            "Difficulty".into(),
            data.difficulty,
            original.difficulty,
            move |x| EditorMessage::LevelEdit {
                index: level_index,
                message: LevelEditMessage::EditHighScoreData {
                    index,
                    message: LevelEditHighScoreDataMessage::EditDifficulty(x)
                }
            }
        ),
        num_field(
            "High Score".into(),
            data.high_score,
            original.high_score,
            move |x| EditorMessage::LevelEdit {
                index: level_index,
                message: LevelEditMessage::EditHighScoreData {
                    index,
                    message: LevelEditHighScoreDataMessage::EditHighScore(x)
                }
            }
        ),
        str_field(
            "Letter Grade".into(),
            data.letter_grade.clone(),
            original.letter_grade.clone(),
            move |x| EditorMessage::LevelEdit {
                index: level_index,
                message: LevelEditMessage::EditHighScoreData {
                    index,
                    message: LevelEditHighScoreDataMessage::EditLetterGrade(x)
                }
            }
        ),
        num_field(
            "Max Combo Count".into(),
            data.max_combo_count,
            original.max_combo_count,
            move |x| EditorMessage::LevelEdit {
                index: level_index,
                message: LevelEditMessage::EditHighScoreData {
                    index,
                    message: LevelEditHighScoreDataMessage::EditMaxComboCount(x)
                }
            }
        ),
        num_field(
            "Attempts".into(),
            data.num_attempts,
            original.num_attempts,
            move |x| EditorMessage::LevelEdit {
                index: level_index,
                message: LevelEditMessage::EditHighScoreData {
                    index,
                    message: LevelEditHighScoreDataMessage::EditNumAttempts(x)
                }
            }
        ),
        num_field(
            "Clears".into(),
            data.num_clears,
            original.num_clears,
            move |x| EditorMessage::LevelEdit {
                index: level_index,
                message: LevelEditMessage::EditHighScoreData {
                    index,
                    message: LevelEditHighScoreDataMessage::EditNumClears(x)
                }
            }
        ),
        num_field(
            "Retries".into(),
            data.num_retries,
            original.num_retries,
            move |x| EditorMessage::LevelEdit {
                index: level_index,
                message: LevelEditMessage::EditHighScoreData {
                    index,
                    message: LevelEditHighScoreDataMessage::EditNumRetries(x)
                }
            }
        ),
        num_field(
            "Game Overs".into(),
            data.num_game_overs,
            original.num_game_overs,
            move |x| EditorMessage::LevelEdit {
                index: level_index,
                message: LevelEditMessage::EditHighScoreData {
                    index,
                    message: LevelEditHighScoreDataMessage::EditNumGameOvers(x)
                }
            }
        ),
        bool_field(
            "Has All Perfects".into(),
            data.has_all_perfects,
            original.has_all_perfects,
            move |x| EditorMessage::LevelEdit {
                index: level_index,
                message: LevelEditMessage::EditHighScoreData {
                    index,
                    message: LevelEditHighScoreDataMessage::EditHasAllPerfects(x)
                }
            }
        ),
        bool_field(
            "Has Full Combo Rhythm Shift".into(),
            data.has_full_combo_rhythm_rift,
            original.has_full_combo_rhythm_rift,
            move |x| EditorMessage::LevelEdit {
                index: level_index,
                message: LevelEditMessage::EditHighScoreData {
                    index,
                    message: LevelEditHighScoreDataMessage::EditHasFullComboRhythmRift(x)
                }
            }
        ),
    ])
    .padding(8.0)
    .width(Length::Fill)
    .style(|_| container::Style {
        border: iced::Border {
            color: color!(0x222222),
            width: 1.0.into(),
            radius: 8.0.into(),
        },
        ..Default::default()
    })
    .into()
}
