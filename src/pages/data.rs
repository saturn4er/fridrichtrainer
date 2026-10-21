use leptos::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, HtmlInputElement, Url};

use crate::{
    i18n::t,
    store::{Data, use_store},
};

fn download(name: &str, json: &str) -> Result<(), JsValue> {
    let parts = js_sys::Array::of1(&JsValue::from_str(json));
    let options = BlobPropertyBag::new();
    options.set_type("application/json");
    let blob = Blob::new_with_str_sequence_and_options(&parts, &options)?;
    let url = Url::create_object_url_with_blob(&blob)?;
    let a: HtmlAnchorElement = document().create_element("a")?.unchecked_into();
    a.set_href(&url);
    a.set_download(name);
    a.click();
    Url::revoke_object_url(&url)
}

fn today() -> String {
    let d = js_sys::Date::new_0();
    format!("{}-{:02}-{:02}", d.get_full_year(), d.get_month() + 1, d.get_date())
}

#[component]
pub fn DataPage() -> impl IntoView {
    let data = use_store();
    let s = t();
    let message = RwSignal::new(None::<&str>);

    let export = move |_| {
        let json = data.with_untracked(Data::to_json);
        if download(&format!("fridrich-trainer-{}.json", today()), &json).is_err() {
            message.set(Some(s.download_failed));
        }
    };

    let import = move |ev: leptos::ev::Event| {
        let input: HtmlInputElement = event_target(&ev);
        let Some(file) = input.files().and_then(|f| f.get(0)) else {
            return;
        };
        input.set_value("");
        wasm_bindgen_futures::spawn_local(async move {
            let text = wasm_bindgen_futures::JsFuture::from(file.text()).await;
            let parsed = text.ok().and_then(|t| t.as_string()).map(|t| Data::from_json(&t));
            match parsed {
                Some(Ok(imported)) => {
                    let confirmed = window()
                        .confirm_with_message(s.confirm_import)
                        .unwrap_or(false);
                    if confirmed {
                        data.set(imported);
                        message.set(Some(s.imported));
                    }
                }
                _ => message.set(Some(s.bad_file)),
            }
        });
    };

    let reset = move |_| {
        let confirmed = window()
            .confirm_with_message(s.confirm_reset)
            .unwrap_or(false);
        if confirmed {
            data.set(Data::default());
            message.set(Some(s.reset_done));
        }
    };

    let summary = move || {
        data.with(|d| {
            let times: usize = d.case_times.values().map(Vec::len).sum();
            let marked = d.progress.values().filter(|p| p.status != Default::default()).count();
            (s.data_summary)(d.solves.len(), marked, times)
        })
    };

    view! {
        <div class="data-page">
            <h1>{s.data_title}</h1>
            <p>{s.data_info}</p>
            <p class="muted">{summary}</p>
            <div class="data-actions">
                <button class="button" type="button" on:click=export>
                    {s.export}
                </button>
                <label class="button secondary">
                    {s.import}
                    <input type="file" accept="application/json,.json" hidden on:change=import />
                </label>
                <button class="button danger" type="button" on:click=reset>
                    {s.reset_all}
                </button>
            </div>
            {move || message.get().map(|m| view! { <p class="message">{m}</p> })}
        </div>
    }
}
