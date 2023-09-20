use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

pub type Root = Vec<Root2>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root2 {
    pub taxi: Value,
    pub starters: Vec<String>,
    pub settings: Settings,
    #[serde(rename = "roster_id")]
    pub roster_id: i64,
    #[serde(default)]
    pub reserve: Vec<String>,
    pub players: Vec<String>,
    #[serde(rename = "player_map")]
    pub player_map: Value,
    #[serde(rename = "owner_id")]
    pub owner_id: String,
    pub metadata: Value,
    #[serde(rename = "league_id")]
    pub league_id: String,
    pub keepers: Value,
    #[serde(rename = "co_owners")]
    pub co_owners: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub wins: i64,
    #[serde(rename = "waiver_position")]
    pub waiver_position: i64,
    #[serde(rename = "waiver_budget_used")]
    pub waiver_budget_used: i64,
    #[serde(rename = "total_moves")]
    pub total_moves: i64,
    pub ties: i64,
    #[serde(rename = "ppts_decimal")]
    pub ppts_decimal: i64,
    pub ppts: i64,
    pub losses: i64,
    #[serde(rename = "fpts_decimal")]
    pub fpts_decimal: i64,
    #[serde(rename = "fpts_against_decimal")]
    pub fpts_against_decimal: i64,
    #[serde(rename = "fpts_against")]
    pub fpts_against: i64,
    pub fpts: i64,
}
