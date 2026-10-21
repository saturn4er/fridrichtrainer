mod algs;
mod cube;
mod i18n;
mod pages;
mod render;
mod route;
mod stats;
mod store;
mod timer;
mod ui;

use leptos::{ev, prelude::Set as _, prelude::*};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::KeyboardEvent;

use crate::{
    algs::Set,
    i18n::{I18n, Lang, t},
    pages::{algs::AlgsPage, case::CasePage, data::DataPage, timer::TimerPage, train::TrainPage},
    route::Route,
    store::{Status, Store},
    ui::NotationPanel,
};

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

fn current_route() -> Route {
    Route::parse(&window().location().hash().unwrap_or_default())
}

fn toggle_notation() {
    let Some(panel) = document().get_element_by_id("notation") else {
        return;
    };
    if let Ok(toggle) = js_sys::Reflect::get(&panel, &JsValue::from_str("togglePopover"))
        && let Some(f) = toggle.dyn_ref::<js_sys::Function>()
    {
        let _ = f.call0(&panel);
    }
}

#[component]
fn App() -> impl IntoView {
    Store::provide();
    let lang = I18n::provide();
    let route = RwSignal::new(current_route());
    let hash = window_event_listener(ev::hashchange, move |_| {
        route.set(current_route());
        window().scroll_to_with_x_and_y(0.0, 0.0);
    });
    let help = window_event_listener(ev::keydown, move |e: KeyboardEvent| {
        let in_field = e
            .target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
            .is_some_and(|el| matches!(el.tag_name().as_str(), "INPUT" | "TEXTAREA"));
        if e.key() == "?" && !in_field {
            toggle_notation();
        }
    });
    on_cleanup(move || {
        hash.remove();
        help.remove();
    });

    view! {
        {move || {
            lang.track();
            view! { <Shell route=route lang=lang /> }
        }}
    }
}

#[component]
fn Shell(route: RwSignal<Route>, lang: RwSignal<Lang>) -> impl IntoView {
    let s = t();
    let nav = [
        (Route::Timer, s.nav_timer),
        (Route::Algs { set: Set::Oll, status: None }, s.nav_algs),
        (Route::Train { set: None, status: Some(Status::Learning), case: None }, s.nav_train),
        (Route::Data, s.nav_data),
    ];
    let section = |r: &Route| std::mem::discriminant(&match r {
        Route::Case(_) => Route::Algs { set: Set::Oll, status: None },
        other => other.clone(),
    });

    view! {
        <nav class="top">
            <a class="brand" href="#/">
                "Fridrich" <span>" trainer"</span>
            </a>
            {nav
                .into_iter()
                .map(|(target, label)| {
                    let href = target.href();
                    let active = move || route.with(|r| section(r) == section(&target));
                    view! {
                        <a href=href aria-current=move || active().then_some("page")>
                            {label}
                        </a>
                    }
                })
                .collect_view()}
            <div class="lang">
                {Lang::ALL
                    .into_iter()
                    .map(|l| {
                        view! {
                            <button
                                type="button"
                                aria-pressed=(lang.get_untracked() == l).then_some("true")
                                on:click=move |_| lang.set(l)
                            >
                                {l.code().to_uppercase()}
                            </button>
                        }
                    })
                    .collect_view()}
            </div>
        </nav>
        <main>
            {move || match route.get() {
                Route::Timer => view! { <TimerPage /> }.into_any(),
                Route::Algs { set, status } => view! { <AlgsPage set=set status=status /> }.into_any(),
                Route::Case(id) => view! { <CasePage id=id /> }.into_any(),
                Route::Train { set, status, case } => {
                    view! { <TrainPage set=set status=status case=case /> }.into_any()
                }
                Route::Data => view! { <DataPage /> }.into_any(),
            }}
        </main>
        <NotationPanel />
    }
}
