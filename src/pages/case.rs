use leptos::prelude::{Set as _, *};

use crate::{
    algs::{self, Case},
    i18n::t,
    route::Route,
    stats::format_ms,
    store::{Status, use_store},
    algs::Set,
    cube::{Cube, turn_alg},
    render::{self, Style},
    ui::{CaseDiagram, Diagram, PracticeStats, StatusPicker, use_favicon},
};

/// The case met from each side of the cube, with the algorithm rewritten
/// for that grip. Sides that look the same are shown once.
#[component]
fn Sides(set: Set, alg: String) -> impl IntoView {
    let s = t();
    let style = Style::for_set(set);
    let mut seen = Vec::new();
    let mut sides = Vec::new();
    let mut current = alg;
    for _ in 0..4 {
        if let Ok(cube) = Cube::case_of(&current) {
            let sig = render::diagram(&cube, style).signature();
            if !seen.contains(&sig) {
                seen.push(sig);
                sides.push((cube, current.clone()));
            }
        }
        current = turn_alg(&current);
    }
    view! {
        <h3>{s.sides}</h3>
        <div class="sides">
            {sides
                .into_iter()
                .map(|(cube, alg)| {
                    view! {
                        <figure>
                            <Diagram cube=cube style=style class="case-cube" />
                            <figcaption>
                                <code class="alg">{alg}</code>
                            </figcaption>
                        </figure>
                    }
                })
                .collect_view()}
        </div>
    }
}

#[component]
pub fn CasePage(id: String) -> impl IntoView {
    match algs::find(&id) {
        Some(case) => view! { <CaseDetails case=case /> }.into_any(),
        None => view! {
            <p class="empty">
                {t().no_such_case}
                <a href=Route::Algs { set: algs::Set::Oll, status: None }.href()>{t().to_algs}</a>
            </p>
        }
        .into_any(),
    }
}

#[component]
fn CaseDetails(case: &'static Case) -> impl IntoView {
    let data = use_store();
    let s = t();
    let custom = move || data.with(|d| d.custom_alg(&case.id));
    let draft = RwSignal::new(data.with_untracked(|d| d.custom_alg(&case.id)).unwrap_or_default());
    let error = RwSignal::new(None::<String>);
    let times = Signal::derive(move || data.with(|d| d.times(&case.id)));
    use_favicon(move || Cube::case_of(case.alg).expect("built-in algorithms parse"), Style::for_set(case.set));

    let save = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let alg = draft.get_untracked().split_whitespace().collect::<Vec<_>>().join(" ");
        if alg.is_empty() {
            data.update(|d| d.set_custom_alg(&case.id, None));
            error.set(None);
            return;
        }
        match algs::matches_case(case, &alg) {
            Ok(()) => {
                draft.set(alg.clone());
                data.update(|d| d.set_custom_alg(&case.id, Some(alg)));
                error.set(None);
            }
            Err(e) => error.set(Some(s.check_error(&e))),
        }
    };

    view! {
        <a class="back" href=Route::Algs { set: case.set, status: None }.href()>
            "← " {case.set.title()}
        </a>
        <div class="case-page">
            <CaseDiagram case=case class="case-cube big" />
            <div class="case-info">
                <h1>{case.name.clone()} <small>{s.group(case.group)}</small></h1>
                <StatusPicker case_id=case.id.clone() />

                <h3>{s.algorithm}</h3>
                <code class="alg big">{case.alg}</code>
                {move || {
                    custom()
                        .map(|alg| {
                            view! {
                                <h3>{s.my_version}</h3>
                                <code class="alg big mine">{alg}</code>
                            }
                        })
                }}
                {(case.set != Set::F2l)
                    .then_some(move || {
                        let alg = custom().unwrap_or_else(|| case.alg.to_owned());
                        view! { <Sides set=case.set alg=alg /> }
                    })}
                <h3>{s.set_up_case}</h3>
                <p class="muted">{s.set_up_hint}</p>
                <code class="alg">{case.setup()}</code>

                <form class="custom" on:submit=save>
                    <label for="custom-alg">{s.custom_alg}</label>
                    <input
                        id="custom-alg"
                        prop:value=move || draft.get()
                        on:input=move |ev| draft.set(event_target_value(&ev))
                        placeholder=s.custom_placeholder
                        autocomplete="off"
                        spellcheck="false"
                    />
                    <button type="submit">{s.save}</button>
                    {move || error.get().map(|e| view! { <p class="error">{e}</p> })}
                    <p class="muted">{s.custom_hint}</p>
                </form>

                <h3>{s.practice}</h3>
                <PracticeStats times=times />
                <p class="times">
                    {move || {
                        times
                            .get()
                            .into_iter()
                            .take(30)
                            .map(|t| view! { <span>{format_ms(t)}</span> })
                            .collect_view()
                    }}
                </p>
                <a
                    class="button"
                    href=Route::Train {
                        set: None,
                        status: Some(Status::Learning),
                        case: Some(case.id.clone()),
                    }
                        .href()
                >
                    {s.train_this}
                </a>
            </div>
        </div>
    }
}
