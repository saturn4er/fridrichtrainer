use leptos::prelude::{Set as _, *};

use crate::{
    algs::{self, Case, Set},
    i18n::t,
    pages::timer::random_index,
    route::{Route, StatusFilter},
    stats::format_ms,
    store::{Status, use_store},
    timer::Timer,
    ui::{CaseDiagram, PracticeStats},
};

#[component]
pub fn TrainPage(set: Option<Set>, status: StatusFilter, case: Option<String>) -> impl IntoView {
    let data = use_store();
    let s = t();
    let fixed = case.as_deref().and_then(algs::find);

    let pool = move || -> Vec<&'static Case> {
        if let Some(case) = fixed {
            return vec![case];
        }
        data.with_untracked(|d| {
            algs::CASES
                .iter()
                .filter(|c| set.is_none_or(|s| c.set == s))
                .filter(|c| status.is_none_or(|st| d.status(&c.id) == st))
                .collect()
        })
    };
    let pick = move || {
        let pool = pool();
        (!pool.is_empty()).then(|| pool[random_index(pool.len())])
    };
    let current = RwSignal::new(pick());
    let result = RwSignal::new(None::<i64>);
    let next = move || {
        result.set(None);
        current.set(pick());
    };
    let url = move |set: Option<Set>, status: StatusFilter| Route::Train { set, status, case: None }.href();

    view! {
        <div class="train-filters">
            <nav class="tabs">
                <a href=url(None, status) aria-current=set.is_none().then_some("page")>
                    {s.all}
                </a>
                {Set::ALL
                    .into_iter()
                    .map(|s| {
                        view! {
                            <a href=url(Some(s), status) aria-current=(set == Some(s)).then_some("page")>
                                {s.title()}
                            </a>
                        }
                    })
                    .collect_view()}
            </nav>
            <nav class="filters">
                {Status::ALL
                    .into_iter()
                    .map(|st| {
                        view! {
                            <a href=url(set, Some(st)) aria-current=(status == Some(st)).then_some("page")>
                                {s.status(st)}
                            </a>
                        }
                    })
                    .collect_view()}
                <a href=url(set, None) aria-current=status.is_none().then_some("page")>
                    {s.any_status}
                </a>
            </nav>
            <p class="muted">{s.in_pool} {pool().len()}</p>
        </div>

        {move || match current.get() {
            None => {
                view! {
                    <p class="empty">
                        {s.no_cases}
                        <a href=Route::Algs { set: Set::Oll, status: None }.href()>{s.no_cases_link}</a>
                        {s.no_cases_end}
                    </p>
                }
                    .into_any()
            }
            Some(case) => view! { <Training case=case result=result on_next=next /> }.into_any(),
        }}
    }
}

#[component]
fn Training(case: &'static Case, result: RwSignal<Option<i64>>, #[prop(into)] on_next: Callback<()>) -> impl IntoView {
    let data = use_store();
    let s = t();
    let alg = move || data.with(|d| d.custom_alg(&case.id)).unwrap_or_else(|| case.alg.to_owned());
    let times = Signal::derive(move || data.with(|d| d.times(&case.id)));

    view! {
        <div class="train">
            <div class="train-case">
                <CaseDiagram case=case class="case-cube big" />
                <p class="muted">{s.set_up}</p>
                <code class="alg">{case.setup()}</code>
            </div>
            <div class="train-run">
                <Timer
                    on_done=move |ms| {
                        data.update(|d| d.add_case_time(&case.id, ms));
                        result.set(Some(ms));
                    }
                    has_next=Signal::derive(move || result.get().is_some())
                    on_next=on_next
                />
                {move || match result.get() {
                    None => {
                        view! {
                            <details class="reveal">
                                <summary>{s.peek}</summary>
                                <code class="alg big">{alg}</code>
                            </details>
                        }
                            .into_any()
                    }
                    Some(ms) => {
                        view! {
                            <div class="result">
                                <h2>
                                    <a href=Route::Case(case.id.clone()).href()>{case.name.clone()}</a>
                                    <span class="result-time">{format_ms(ms)}</span>
                                </h2>
                                <code class="alg big">{alg}</code>
                                <PracticeStats times=times />
                                <button class="button next" type="button" on:click=move |_| on_next.run(())>
                                    {s.next_case}
                                </button>
                                <p class="muted">{s.next_hint}</p>
                            </div>
                        }
                            .into_any()
                    }
                }}
            </div>
        </div>
    }
}
