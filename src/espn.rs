use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

pub type Root = Vec<Root2>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root2 {
    #[serde(rename = "starters_points")]
    pub starters_points: Vec<i64>,
    pub starters: Vec<String>,
    #[serde(rename = "roster_id")]
    pub roster_id: i64,
    pub points: i64,
    #[serde(rename = "players_points")]
    pub players_points: Value,
    pub players: Vec<String>,
    #[serde(rename = "matchup_id")]
    pub matchup_id: i64,
    #[serde(rename = "custom_points")]
    pub custom_points: Value,
}
