use leptos::prelude::*;

use crate::{
    algs::{self, Case, Group, Set},
    i18n::t,
    route::{Route, StatusFilter},
    stats::format_ms,
    store::{Status, use_store},
    ui::{CaseDiagram, StatusPicker},
};

#[component]
pub fn AlgsPage(set: Set, status: StatusFilter) -> impl IntoView {
    let data = use_store();
    let s = t();

    let groups = Memo::new(move |_| {
        let mut groups: Vec<(Group, Vec<&'static Case>)> = Vec::new();
        data.with(|d| {
            for case in algs::of_set(set) {
                if status.is_some_and(|s| s != d.status(&case.id)) {
                    continue;
                }
                match groups.iter_mut().find(|(g, _)| *g == case.group) {
                    Some((_, cases)) => cases.push(case),
                    None => groups.push((case.group, vec![case])),
                }
            }
        });
        groups
    });

    let total = algs::of_set(set).count();
    let count = move |st: Status| {
        data.with(|d| algs::of_set(set).filter(|c| d.status(&c.id) == st).count())
    };
    let pct = move |n: usize| format!("width: {:.2}%", n as f64 * 100.0 / total as f64);
    let filter = move |s: StatusFilter| Route::Algs { set, status: s }.href();

    view! {
        <div class="algs-head">
            <nav class="tabs">
                {Set::ALL
                    .into_iter()
                    .map(|s| {
                        view! {
                            <a
                                href=Route::Algs { set: s, status: None }.href()
                                aria-current=(s == set).then_some("page")
                            >
                                {s.title()}
                            </a>
                        }
                    })
                    .collect_view()}
            </nav>
            <div class="progress">
                <div class="bar">
                    <span class="learned" style=move || pct(count(Status::Learned))></span>
                    <span class="learning" style=move || pct(count(Status::Learning))></span>
                </div>
                <p>
                    {move || (s.learned_summary)(count(Status::Learned), total, count(Status::Learning))}
                </p>
            </div>
        </div>

        <nav class="filters">
            <a href=filter(None) aria-current=status.is_none().then_some("page")>
                {s.all}
            </a>
            {Status::ALL
                .into_iter()
                .map(|st| {
                    view! {
                        <a href=filter(Some(st)) aria-current=(status == Some(st)).then_some("page")>
                            {s.status(st)}
                        </a>
                    }
                })
                .collect_view()}
            <a
                class="train-link"
                href=Route::Train {
                    set: Some(set),
                    status: Some(Status::Learning),
                    case: None,
                }
                    .href()
            >
                {s.train_learning}
            </a>
        </nav>

        <Show when=move || groups.with(Vec::is_empty)>
            <p class="empty">{s.empty_filter}</p>
        </Show>

        <For
            each=move || groups.get()
            key=|(group, cases)| (*group, cases.iter().map(|c| c.id.clone()).collect::<Vec<_>>())
            children=move |(group, cases)| {
                view! {
                    <section class="group">
                        <h2>{s.group(group)} <span>{cases.len()}</span></h2>
                        <div class="cards">
                            {cases.into_iter().map(|case| view! { <CaseCard case=case /> }).collect_view()}
                        </div>
                    </section>
                }
            }
        />
    }
}

#[component]
fn CaseCard(case: &'static Case) -> impl IntoView {
    let data = use_store();
    let s = t();
    let alg = move || data.with(|d| d.custom_alg(&case.id)).unwrap_or_else(|| case.alg.to_owned());
    let best = move || {
        data.with(|d| {
            d.case_times
                .get(&case.id)
                .filter(|t| !t.is_empty())
                .map(|t| (t.iter().copied().min().unwrap_or_default(), t.len()))
        })
    };
    view! {
        <article class="card">
            <a class="card-link" href=Route::Case(case.id.clone()).href()>
                <CaseDiagram case=case class="case-cube" />
                <div class="card-title">
                    <b>{case.name.clone()}</b>
                    {move || {
                        best()
                            .map(|(best, n)| {
                                view! {
                                    <small title=s.best_and_attempts>
                                        {format_ms(best)} " · " {n}
                                    </small>
                                }
                            })
                    }}
                </div>
                <code class="alg">{alg}</code>
            </a>
            <StatusPicker case_id=case.id.clone() />
        </article>
    }
}
