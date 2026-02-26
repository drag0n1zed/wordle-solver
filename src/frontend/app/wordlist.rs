use gloo_net::http::Request;
use leptos::{prelude::*, task};
use std::sync::Arc;
use wasm_bindgen_futures::JsFuture;

use crate::{
    backend::reqs::{Requirement, WordlistFilter},
    frontend::app::solutions::SolutionEntry,
};

pub(super) enum WordlistState {
    Loading,
    Loaded(Arc<String>),
    Error(String),
}

#[derive(Clone)]
pub(super) struct WordlistEntry {
    pub name: String,
    pub state: RwSignal<WordlistState>,
    pub removable: bool,
}

impl WordlistEntry {
    pub(super) fn filter(&self, reqs: &Requirement) -> SolutionEntry {
        let solutions = self.state.with(|state| match state {
            WordlistState::Loaded(source) => Some(source.lines().filter_wordlist(reqs).map(str::to_owned).collect()),
            _ => None,
        });
        SolutionEntry {
            id: 0,
            name: self.name.clone(),
            value: solutions,
        }
    }
}

pub(super) enum WordlistSource {
    // "/assets/enable1.txt"
    Url(&'static str),
    // <input type="file">
    File(web_sys::File),
}

pub(super) fn fetch(name: impl Into<String>, source: WordlistSource, removable: bool) -> WordlistEntry {
    let name = name.into();
    let state = RwSignal::new(WordlistState::Loading);
    match source {
        WordlistSource::Url(url) => task::spawn_local(async move {
            let result = async {
                let resp = Request::get(url).send().await.map_err(|e| e.to_string())?;
                if !resp.ok() {
                    return Err(format!("HTTP {}", resp.status()));
                }
                resp.text().await.map_err(|e| e.to_string())
            }
            .await;
            state.set(match result {
                Ok(text) => WordlistState::Loaded(Arc::new(text)),
                Err(e) => WordlistState::Error(e),
            })
        }),
        WordlistSource::File(file) => task::spawn_local(async move {
            let result = JsFuture::from(file.text())
                .await
                .map(|s| s.as_string().unwrap_or_default())
                .map_err(|e| format!("{e:?}"));
            state.set(match result {
                Ok(text) => WordlistState::Loaded(Arc::new(text)),
                Err(e) => WordlistState::Error(e),
            })
        }),
    }
    WordlistEntry { name, state, removable }
}
