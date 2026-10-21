//! Hash-based routes, so the app works from any static host path without
//! server-side fallbacks.

use crate::{algs::Set, store::Status};

/// Status filter of a list: `None` shows every status.
pub type StatusFilter = Option<Status>;

#[derive(Clone, Debug, PartialEq)]
pub enum Route {
    Timer,
    Algs { set: Set, status: StatusFilter },
    Case(String),
    Train { set: Option<Set>, status: StatusFilter, case: Option<String> },
    Data,
}

impl Route {
    pub fn parse(hash: &str) -> Route {
        let hash = hash.trim_start_matches('#');
        let (path, query) = hash.split_once('?').unwrap_or((hash, ""));
        let param = |name: &str| {
            query
                .split('&')
                .filter_map(|kv| kv.split_once('='))
                .find(|(k, _)| *k == name)
                .map(|(_, v)| v.to_owned())
        };
        let status = |default: StatusFilter| match param("status").as_deref() {
            Some("all") => None,
            Some(s) => Status::parse(s).or(default),
            None => default,
        };
        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        match segments.as_slice() {
            ["algs"] => Route::Algs {
                set: param("set").as_deref().and_then(Set::from_slug).unwrap_or(Set::Oll),
                status: status(None),
            },
            ["algs", id] => Route::Case((*id).to_owned()),
            ["train"] => Route::Train {
                set: param("set").as_deref().and_then(Set::from_slug),
                status: status(Some(Status::Learning)),
                case: param("case"),
            },
            ["data"] => Route::Data,
            _ => Route::Timer,
        }
    }

    pub fn href(&self) -> String {
        let status = |s: StatusFilter| s.map_or("all", Status::as_str);
        match self {
            Route::Timer => "#/".to_owned(),
            Route::Algs { set, status: None } => format!("#/algs?set={}", set.slug()),
            Route::Algs { set, status: Some(s) } => {
                format!("#/algs?set={}&status={}", set.slug(), s.as_str())
            }
            Route::Case(id) => format!("#/algs/{id}"),
            Route::Train { case: Some(id), .. } => format!("#/train?case={id}"),
            Route::Train { set, status: st, case: None } => match set {
                Some(set) => format!("#/train?set={}&status={}", set.slug(), status(*st)),
                None => format!("#/train?status={}", status(*st)),
            },
            Route::Data => "#/data".to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips() {
        let routes = [
            Route::Timer,
            Route::Algs { set: Set::Pll, status: None },
            Route::Algs { set: Set::F2l, status: Some(Status::Learned) },
            Route::Case("oll-27".to_owned()),
            Route::Train { set: None, status: Some(Status::Learning), case: None },
            Route::Train { set: Some(Set::Oll), status: None, case: None },
            Route::Train { set: None, status: Some(Status::Learning), case: Some("pll-t".to_owned()) },
            Route::Data,
        ];
        for route in routes {
            assert_eq!(Route::parse(&route.href()), route, "{}", route.href());
        }
    }
}
