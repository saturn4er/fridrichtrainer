use std::collections::BTreeMap;

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

const KEY: &str = "fridrich-trainer";
const BROKEN_KEY: &str = "fridrich-trainer.broken";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Penalty {
    #[default]
    Ok,
    Plus2,
    Dnf,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Solve {
    pub id: u64,
    pub ms: i64,
    #[serde(default)]
    pub penalty: Penalty,
    pub scramble: String,
    /// Unix time in milliseconds.
    pub at: f64,
}

impl Solve {
    /// Time with penalties applied; `None` for a DNF.
    pub fn result(&self) -> Option<i64> {
        match self.penalty {
            Penalty::Ok => Some(self.ms),
            Penalty::Plus2 => Some(self.ms + 2000),
            Penalty::Dnf => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    #[default]
    New,
    Learning,
    Learned,
}

impl Status {
    pub const ALL: [Status; 3] = [Status::New, Status::Learning, Status::Learned];

    pub fn as_str(self) -> &'static str {
        match self {
            Status::New => "new",
            Status::Learning => "learning",
            Status::Learned => "learned",
        }
    }

    pub fn parse(s: &str) -> Option<Status> {
        Status::ALL.into_iter().find(|st| st.as_str() == s)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Progress {
    #[serde(default)]
    pub status: Status,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_alg: Option<String>,
}

/// Everything the app remembers. Lists are kept oldest first.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Data {
    #[serde(default)]
    pub solves: Vec<Solve>,
    #[serde(default)]
    pub progress: BTreeMap<String, Progress>,
    #[serde(default)]
    pub case_times: BTreeMap<String, Vec<i64>>,
    #[serde(default)]
    pub next_id: u64,
}

impl Data {
    pub fn status(&self, case_id: &str) -> Status {
        self.progress.get(case_id).map(|p| p.status).unwrap_or_default()
    }

    pub fn custom_alg(&self, case_id: &str) -> Option<String> {
        self.progress.get(case_id).and_then(|p| p.custom_alg.clone())
    }

    /// Penalised results, newest first.
    pub fn results(&self) -> Vec<Option<i64>> {
        self.solves.iter().rev().map(Solve::result).collect()
    }

    /// Practice times of one case, newest first.
    pub fn times(&self, case_id: &str) -> Vec<i64> {
        self.case_times
            .get(case_id)
            .map(|t| t.iter().rev().copied().collect())
            .unwrap_or_default()
    }

    pub fn add_solve(&mut self, ms: i64, scramble: String) {
        self.next_id += 1;
        self.solves.push(Solve {
            id: self.next_id,
            ms,
            penalty: Penalty::Ok,
            scramble,
            at: js_sys::Date::now(),
        });
    }

    pub fn set_penalty(&mut self, id: u64, penalty: Penalty) {
        if let Some(s) = self.solves.iter_mut().find(|s| s.id == id) {
            s.penalty = penalty;
        }
    }

    pub fn delete_solve(&mut self, id: u64) {
        self.solves.retain(|s| s.id != id);
    }

    pub fn set_status(&mut self, case_id: &str, status: Status) {
        self.progress.entry(case_id.to_owned()).or_default().status = status;
    }

    pub fn set_custom_alg(&mut self, case_id: &str, alg: Option<String>) {
        self.progress.entry(case_id.to_owned()).or_default().custom_alg = alg;
    }

    pub fn add_case_time(&mut self, case_id: &str, ms: i64) {
        self.case_times.entry(case_id.to_owned()).or_default().push(ms);
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("data serializes")
    }

    pub fn from_json(json: &str) -> Result<Data, serde_json::Error> {
        serde_json::from_str(json)
    }
}

fn storage() -> Option<web_sys::Storage> {
    window().local_storage().ok().flatten()
}

/// Reads saved data. Unreadable data is copied aside so a later save does
/// not destroy it.
fn load() -> Data {
    let Some(storage) = storage() else {
        return Data::default();
    };
    let Ok(Some(json)) = storage.get_item(KEY) else {
        return Data::default();
    };
    Data::from_json(&json).unwrap_or_else(|_| {
        let _ = storage.set_item(BROKEN_KEY, &json);
        Data::default()
    })
}

#[derive(Clone, Copy)]
pub struct Store(pub RwSignal<Data>);

impl Store {
    /// Loads saved data, keeps it in sync with localStorage and makes it
    /// available through [`use_store`].
    pub fn provide() {
        let data = RwSignal::new(load());
        Effect::new(move |_| {
            let json = data.with(|d| serde_json::to_string(d).expect("data serializes"));
            if let Some(storage) = storage() {
                let _ = storage.set_item(KEY, &json);
            }
        });
        provide_context(Store(data));
    }
}

pub fn use_store() -> RwSignal<Data> {
    expect_context::<Store>().0
}
