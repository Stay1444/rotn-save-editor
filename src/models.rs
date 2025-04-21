use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SaveGame {
    pub save_name: String,
    pub game_data_version: u64,
    pub save_data_version: u64,
    pub times_booted: u64,
    #[serde(rename = "SaveID")]
    pub save_id: u64,
    #[serde(rename = "PlayerID")]
    pub player_id: String,
    pub selected_language: String,
    pub framerate_limit: u64,
    #[serde(rename = "LevelDatas")]
    pub level_data: Vec<LevelData>,
    #[serde(rename = "StorylineDatas")]
    pub storyline_data: Vec<StorylineData>,
    pub active_cosmetic_pin: String,
    pub active_gameplay_pin: String,
    pub should_display_dialogue_debug: bool,
    pub should_unlock_all_levels: bool,
    pub has_input_dragon_dance: bool,
    pub selected_story_difficulty: u64,
    pub selected_arcade_difficulty: u64,
    pub selected_track_sorting_order: u64,
    pub selected_custom_music_sorting_order: u64,
    pub is_remix_mode_active: bool,
    pub should_play_all_story_content_in_order: bool,
    pub enemy_kill_counts_by_id: Vec<EnemyKillCount>,
    pub total_rhythm_rifts_cleared: u64,
    pub has_seen_splash_screens: bool,
    pub has_opened_story_mode: bool,
    pub has_agreed_to_no_streaming: bool,
    pub total_diamonds: u64,
    pub total_vibe_power_uses: u64,
    pub max_enemies_killed_while_vibing: u64,
    #[serde(rename = "BBTotalAttacks")]
    pub bb_total_attacks: u64,
    #[serde(rename = "BBTotalDodges")]
    pub bb_total_dodges: u64,
    #[serde(rename = "BBTotalBlockedHits")]
    pub bb_total_blocked_hits: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LevelData {
    pub level_id: String,
    pub stage_type: u64,
    pub was_completed_in_story_mode: bool,
    pub was_attempted_in_story_mode: bool,
    pub was_skipped_in_story_mode: bool,
    pub awarded_diamonds: u64,
    pub awarded_diamonds_remix: u64,
    #[serde(rename = "DifficultyHighScoreDatas")]
    pub difficulty_data: Vec<DifficultyData>,
    #[serde(rename = "RemixDifficultyHighScoreDatas")]
    pub remix_difficulty_data: Vec<DifficultyData>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DifficultyData {
    pub difficulty: u64,
    pub high_score: u64,
    pub letter_grade: String,
    pub max_combo_count: u64,
    pub num_attempts: u64,
    pub num_clears: u64,
    pub num_retries: u64,
    pub num_game_overs: u64,
    pub has_all_perfects: bool,
    pub has_full_combo_rhythm_rift: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct StorylineData {
    #[serde(rename = "storylineCharacters")]
    pub storyline_characters: u64,
    pub has_unlocked_storyline: bool,
    pub has_completed_storyline: bool,
    #[serde(rename = "StoryBeatDatas")]
    pub story_beat_data: Vec<StoryBeatData>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct StoryBeatData {
    pub level_id: String,
    pub times_played: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct EnemyKillCount {
    pub enemy_id: u64,
    pub number_of_kills: u64,
    pub number_of_deaths: u64,
}
