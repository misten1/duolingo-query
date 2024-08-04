use reqwest::Client;
use std::collections::HashMap;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::prelude::duolingo_types::*;

// A struct containing information about a duolingo user.
pub struct Duolingo {
    user_profile: UserProfile,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserProfile {
    #[serde(rename = "globalAmbassadorStatus")]
    global_ambassador_status: HashMap<String, Value>,
    roles: Vec<String>,
    #[serde(rename = "totalXp")]
    total_xp: u32,
    id: u64,
    #[serde(rename = "hasPlus")]
    has_plus: bool,
    courses: Vec<Course>,
    #[serde(rename = "fromLanguage")]
    from_language: String,
    streak: u32,
    #[serde(rename = "creationDate")]
    creation_date: u64,
    #[serde(rename = "streakData")]
    streak_data: StreakData,
    #[serde(rename = "privacySettings")]
    privacy_settings: Vec<String>,
    picture: String,
    #[serde(rename = "learningLanguage")]
    learning_language: String,
    #[serde(rename = "subscriberLevel")]
    subscriber_level: String,
    name: Option<String>,
    username: String,
}


impl Duolingo {

    // Query information about a duolingo user
    pub async fn query(username: &str) -> anyhow::Result<Duolingo> {
        let uid = Self::get_user_uid(username).await?;
        Self::query_internal(uid).await
    }

    // Query information about a duolingo with uid
    pub async fn query_with_uid(id: u64) -> anyhow::Result<Duolingo> {
        Self::query_internal(id).await
    }

    async fn query_internal(id: u64) -> anyhow::Result<Duolingo> {
        let client = Client::new();
        let time = stamp();
        let tmp = "{currentStreak,previousStreak}";
        let data_endpoint = format!("https://www.duolingo.com/2017-06-30/users/{id}?fields=courses,creationDate,fromLanguage,gemsConfig,globalAmbassadorStatus,hasPlus,id,learningLanguage,location,name,picture,privacySettings,roles,streak,streakData{tmp},subscriberLevel,totalXp,username&_={time}");
        let request = client.get(data_endpoint).build()?;
        let response = client.execute(request).await?;
        let user_profile = response.json::<UserProfile>().await.expect("Error parsing userprofile, This could be a misspelling in the username or duolingo's API updated");
        Ok(Duolingo { user_profile })
    }

    // Get user id
    pub async fn get_user_uid(username: &str) -> anyhow::Result<u64> {
        let client = Client::new();
        let time = stamp();
        let uid_endpoint = format!("https://www.duolingo.com/2017-06-30/users?username={username}&_={time}");
        let request = client.get(uid_endpoint).build()?;
        let result = client.execute(request).await?;
        let user_response = result.json::<UsersResponse>().await?;
        Ok(user_response.users.first().unwrap().id)
    }

    pub fn get_roles(&self) -> &Vec<String> {
        &self.user_profile.roles
    }

    pub fn get_total_xp(&self) -> u32 {
        self.user_profile.total_xp
    }

    pub fn get_id(&self) -> u64 {
        self.user_profile.id
    }

    pub fn has_plus(&self) -> bool {
        self.user_profile.has_plus
    }

    pub fn get_courses(&self) -> &Vec<Course> {
        &self.user_profile.courses
    }

    pub fn get_from_language(&self) -> &str {
        &self.user_profile.from_language
    }

    pub fn get_streak(&self) -> u32 {
        self.user_profile.streak
    }

    pub fn get_creation_date(&self) -> u64 {
        self.user_profile.creation_date
    }

    pub fn get_streak_data(&self) -> &StreakData {
        &self.user_profile.streak_data
    }

    pub fn get_privacy_settings(&self) -> &Vec<String> {
        &self.user_profile.privacy_settings
    }

    pub fn get_picture(&self) -> &str {
        &self.user_profile.picture
    }

    pub fn get_learning_language(&self) -> &str {
        &self.user_profile.learning_language
    }

    pub fn get_subscriber_level(&self) -> &str {
        &self.user_profile.subscriber_level
    }

    pub fn get_name(&self) -> &Option<String> {
        &self.user_profile.name
    }

    pub fn get_username(&self) -> &str {
        &self.user_profile.username
    }

    pub fn get_global_ambassador_status(&self) -> &HashMap<String, Value> {
        &self.user_profile.global_ambassador_status
    }
}

fn stamp() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Unable to get UNIX timestamp")
        .as_millis()
}