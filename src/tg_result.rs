use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct TgExportResult {
    pub name: String,
    pub id: u64,
    pub messages: Vec<TgExportMessage>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct TgExportMessage {
    pub id: u64,
    pub date_unixtime: String,

    // pub from: String,
    #[serde(default)]
    pub from_id: Option<String>,
    #[serde(default)]
    pub from: Option<String>,

    #[serde(default)]
    pub actor_id: Option<String>,
    #[serde(default)]
    pub actor: Option<String>,
    // pub text: String,

    #[serde(rename = "type")]
    pub message_type: String,
}


impl Default for TgExportMessage {
    fn default() -> Self {
        Self {
            id: 0,
            date_unixtime: "".to_string(),
            from_id: None,
            from: None,
            actor_id: None,
            actor: None,
            message_type: "".to_string(),
        }
    }
}

impl TgExportMessage {
    pub(crate) fn get_date(&self) -> DateTime<Utc> {
        DateTime::from_timestamp(self.date_unixtime.parse::<i64>().unwrap(), 0).unwrap()
    }

    pub(crate) fn get_actor_id(&self) -> u64 {
        let id = {
            if self.actor_id.is_some() { self.actor_id.clone().unwrap() } else { self.from_id.clone().unwrap() }
        };

        id.replacen("user", "", 1).parse().unwrap()
    }

    pub(crate) fn get_actor(&self) -> String {
        if self.actor.is_some() {self.actor.clone().unwrap()} else {self.from.clone().unwrap()}
    }
}