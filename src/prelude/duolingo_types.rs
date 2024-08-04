use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct UsersResponse {
    pub(crate) users: Vec<IDEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IDEntry {
    pub(crate) id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Course {
    preload: bool,
    #[serde(rename = "placementTestAvailable")]
    placement_test_available: bool,
    #[serde(rename = "authorId")]
    author_id: String,
    title: String,
    #[serde(rename = "learningLanguage")]
    learning_language: String,
    xp: u32,
    #[serde(rename = "healthEnabled")]
    health_enabled: bool,
    #[serde(rename = "fromLanguage")]
    from_language: String,
    id: String,
    crowns: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StreakData {
    #[serde(rename = "currentStreak")]
    current_streak: Option<Value>,
    #[serde(rename = "previousStreak")]
    previous_streak: Option<PreviousStreak>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviousStreak {
    #[serde(rename = "endDate")]
    end_date: String,
    length: u32,
    #[serde(rename = "lastExtendedDate")]
    last_extended_date: String,
    #[serde(rename = "startDate")]
    start_date: String,
}

impl Course {
    pub fn get_placement_test_available(&self) -> bool {
        self.placement_test_available
    }

    pub fn get_author_id(&self) -> &str {
        &self.author_id
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_learning_language(&self) -> &str {
        &self.learning_language
    }

    pub fn get_xp(&self) -> u32 {
        self.xp
    }

    pub fn get_health_enabled(&self) -> bool {
        self.health_enabled
    }

    pub fn get_from_language(&self) -> &str {
        &self.from_language
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_crowns(&self) -> u32 {
        self.crowns
    }
}

impl PreviousStreak {
    pub fn get_end_date(&self) -> &str {
        &self.end_date
    }

    pub fn get_length(&self) -> u32 {
        self.length
    }

    pub fn get_last_extended_date(&self) -> &str {
        &self.last_extended_date
    }

    pub fn get_start_date(&self) -> &str {
        &self.start_date
    }
}

impl StreakData {
    pub fn current_streak(&self) -> &Option<Value> {
        &self.current_streak
    }

    pub fn previous_streak(&self) -> &Option<PreviousStreak>  {
        &self.previous_streak
    }
}