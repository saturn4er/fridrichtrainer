use leptos::prelude::*;

use crate::{
    algs::Case,
    i18n::t,
    cube::Cube,
    render::{self, Style},
    stats::{self, format_ms, format_result},
    store::{Status, use_store},
};

#[component]
pub fn Diagram(cube: Cube, style: Style, #[prop(optional, into)] class: String) -> impl IntoView {
    let d = render::diagram(&cube, style);
    view! {
        <svg class=class viewBox=d.view_box xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
            {d.polys
                .into_iter()
                .map(|p| view! { <polygon points=p.points class=p.class></polygon> })
                .collect_view()}
            {d.arrows
                .into_iter()
                .map(|a| {
                    view! {
                        <line class="arrow" x1=a.x1 y1=a.y1 x2=a.x2 y2=a.y2></line>
                        {a.heads
                            .into_iter()
                            .map(|h| view! { <polygon class="arrow-head" points=h></polygon> })
                            .collect_view()}
                    }
                })
                .collect_view()}
        </svg>
    }
}

/// Shows `cube` as the tab icon while the calling component is mounted.
pub fn use_favicon(cube: impl Fn() -> Cube + 'static, style: Style) {
    let Ok(Some(link)) = document().query_selector("link[rel=icon]") else {
        return;
    };
    if !link.has_attribute("data-default") {
        let _ = link.set_attribute("data-default", &link.get_attribute("href").unwrap_or_default());
    }
    let shown = std::sync::Arc::new(std::sync::Mutex::new(String::new()));
    let set = shown.clone();
    Effect::new(move || {
        let svg = render::diagram(&cube(), style).to_svg();
        let href = format!("data:image/svg+xml,{}", js_sys::encode_uri_component(&svg));
        let _ = link.set_attribute("href", &href);
        *set.lock().unwrap() = href;
    });
    on_cleanup(move || {
        let Ok(Some(link)) = document().query_selector("link[rel=icon]") else {
            return;
        };
        if link.get_attribute("href").as_deref() == Some(shown.lock().unwrap().as_str())
            && let Some(default) = link.get_attribute("data-default")
        {
            let _ = link.set_attribute("href", &default);
        }
    });
}

#[component]
pub fn CaseDiagram(case: &'static Case, #[prop(optional, into)] class: String) -> impl IntoView {
    let cube = Cube::case_of(case.alg).expect("built-in algorithms parse");
    view! { <Diagram cube=cube style=Style::for_set(case.set) class=class /> }
}

#[component]
pub fn StatusPicker(case_id: String) -> impl IntoView {
    let data = use_store();
    let id = StoredValue::new(case_id);
    let current = move || id.with_value(|id| data.with(|d| d.status(id)));
    let s = t();
    view! {
        <div class="status" data-status=move || current().as_str()>
            {Status::ALL
                .into_iter()
                .map(|st| {
                    view! {
                        <button
                            type="button"
                            class=st.as_str()
                            aria-pressed=move || if current() == st { "true" } else { "false" }
                            on:click=move |_| id.with_value(|id| data.update(|d| d.set_status(id, st)))
                        >
                            {s.status(st)}
                        </button>
                    }
                })
                .collect_view()}
        </div>
    }
}

#[component]
pub fn Stat(label: &'static str, #[prop(into)] value: Signal<String>) -> impl IntoView {
    view! {
        <div class="stat">
            <span class="stat-label">{label}</span>
            <span class="stat-value">{value}</span>
        </div>
    }
}

pub fn dash(v: Option<i64>) -> String {
    v.map_or_else(|| "—".to_owned(), format_ms)
}

pub fn dash_avg(v: Option<Option<i64>>) -> String {
    v.map_or_else(|| "—".to_owned(), format_result)
}

/// Statistics of practice times given newest first.
#[component]
pub fn PracticeStats(#[prop(into)] times: Signal<Vec<i64>>) -> impl IntoView {
    let results = move || times.get().into_iter().map(Some).collect::<Vec<_>>();
    let s = t();
    view! {
        <div class="stats">
            <Stat label=s.attempts value=Signal::derive(move || times.with(Vec::len).to_string()) />
            <Stat label=s.best value=Signal::derive(move || dash(times.with(|t| t.iter().copied().min())))/>
            <Stat label=s.last value=Signal::derive(move || dash(times.with(|t| t.first().copied())))/>
            <Stat label="ao5" value=Signal::derive(move || dash_avg(stats::average_of(&results(), 5)))/>
            <Stat label="ao12" value=Signal::derive(move || dash_avg(stats::average_of(&results(), 12)))/>
        </div>
    }
}

const NOTATION: [&[&str]; 4] = [
    &["R", "L", "U", "D", "F", "B"],
    &["r", "l", "u", "d", "f", "b"],
    &["M", "E", "S"],
    &["x", "y", "z"],
];

#[component]
pub fn NotationPanel() -> impl IntoView {
    let s = t();
    let mut descriptions = s.moves.iter();
    view! {
        <button class="notation-toggle" type="button" popovertarget="notation" title=s.notation_button_title>
            {s.notation}
        </button>
        <aside id="notation" class="notation" popover="">
            <header>
                <h2>{s.notation}</h2>
                <button
                    type="button"
                    class="close"
                    popovertarget="notation"
                    popovertargetaction="hide"
                    aria-label=s.close
                >
                    "×"
                </button>
            </header>
            <p class="notation-rules">
                {s.notation_rules[0]} <b>"'"</b> {s.notation_rules[1]} <b>"2"</b> {s.notation_rules[2]}
            </p>
            {NOTATION
                .into_iter()
                .zip(s.notation_groups)
                .map(|(moves, title)| {
                    view! {
                        <h3>{title}</h3>
                        <ul class="moves">
                            {moves
                                .iter()
                                .map(|&name| {
                                    let text = descriptions.next().copied().unwrap_or_default();
                                    let mut cube = Cube::solved();
                                    cube.apply_alg(name).expect("notation moves parse");
                                    view! {
                                        <li>
                                            <Diagram cube=cube style=Style::Full class="move-cube" />
                                            <div>
                                                <code>{name}</code>
                                                <span>{text}</span>
                                            </div>
                                        </li>
                                    }
                                })
                                .collect_view()}
                        </ul>
                    }
                })
                .collect_view()}
        </aside>
    }
}
