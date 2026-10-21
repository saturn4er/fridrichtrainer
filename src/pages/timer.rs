use leptos::prelude::*;

use crate::{
    cube::Cube,
    i18n::t,
    render::Style,
    stats::{self, format_ms, format_result},
    store::{Penalty, use_store},
    timer::Timer,
    ui::{Diagram, Stat, dash, dash_avg},
};

pub fn random_index(n: usize) -> usize {
    ((js_sys::Math::random() * n as f64) as usize).min(n - 1)
}

#[component]
pub fn TimerPage() -> impl IntoView {
    let data = use_store();
    let s = t();
    let scramble = RwSignal::new(stats::scramble(random_index));
    let new_scramble = move || scramble.set(stats::scramble(random_index));

    let results = Memo::new(move |_| data.with(|d| d.results()));
    let valid = move || results.with(|r| r.iter().flatten().copied().collect::<Vec<_>>());
    let stat = |f: fn(&[Option<i64>]) -> String| Signal::derive(move || results.with(|r| f(r)));

    view! {
        <Timer on_done=move |ms| {
            data.update(|d| d.add_solve(ms, scramble.get_untracked()));
            new_scramble();
        } />
        <div id="session">
            <section class="scramble">
                <div>
                    <div class="scramble-head">
                        <span class="label">{s.scramble}</span>
                        <button class="link" type="button" on:click=move |_| new_scramble()>
                            {s.new_scramble}
                        </button>
                    </div>
                    <code class="scramble-text">{move || scramble.get()}</code>
                    <p class="hold">
                        {s.hold[0]} " " <i class="swatch st-u"></i> " " {s.hold[1]} " "
                        <i class="swatch st-f"></i> " " {s.hold[2]}
                    </p>
                </div>
                {move || {
                    let mut cube = Cube::solved();
                    cube.apply_alg(&scramble.get()).expect("scrambles parse");
                    view! { <Diagram cube=cube style=Style::Full class="scramble-cube" /> }
                }}
            </section>

            <section class="stats">
                <Stat label=s.solves value=stat(|r| r.len().to_string()) />
                <Stat label=s.best value=Signal::derive(move || dash(valid().into_iter().min())) />
                <Stat label=s.mean value=Signal::derive(move || dash(stats::mean(&valid()))) />
                <Stat label="ao5" value=stat(|r| dash_avg(stats::average_of(r, 5))) />
                <Stat label="ao12" value=stat(|r| dash_avg(stats::average_of(r, 12))) />
                <Stat label=s.best_ao5 value=stat(|r| dash(stats::best_average_of(r, 5))) />
                <Stat label=s.best_ao12 value=stat(|r| dash(stats::best_average_of(r, 12))) />
            </section>

            <Show
                when=move || data.with(|d| !d.solves.is_empty())
                fallback=move || view! { <p class="empty">{s.no_solves}</p> }
            >
                <table class="solves">
                    <thead>
                        <tr>
                            <th>"#"</th>
                            <th>{s.time}</th>
                            <th>{s.scramble}</th>
                            <th></th>
                        </tr>
                    </thead>
                    <tbody>
                        <For
                            each=move || {
                                data.with(|d| {
                                    d.solves
                                        .iter()
                                        .enumerate()
                                        .rev()
                                        .take(100)
                                        .map(|(i, s)| (i + 1, s.clone()))
                                        .collect::<Vec<_>>()
                                })
                            }
                            key=|(num, s)| (*num, s.id, s.penalty)
                            children=move |(num, solve)| {
                                let id = solve.id;
                                let penalty = solve.penalty;
                                let toggle = move |p: Penalty| {
                                    let next = if penalty == p { Penalty::Ok } else { p };
                                    data.update(|d| d.set_penalty(id, next));
                                };
                                view! {
                                    <tr>
                                        <td class="num">{num}</td>
                                        <td class="time">
                                            {format_result(solve.result())}
                                            {(penalty == Penalty::Plus2)
                                                .then(|| view! { <small>" (" {format_ms(solve.ms)} "+2)"</small> })}
                                        </td>
                                        <td class="scr">{solve.scramble.clone()}</td>
                                        <td class="actions">
                                            <button
                                                aria-pressed=(penalty == Penalty::Plus2).then_some("true")
                                                on:click=move |_| toggle(Penalty::Plus2)
                                            >
                                                "+2"
                                            </button>
                                            <button
                                                aria-pressed=(penalty == Penalty::Dnf).then_some("true")
                                                on:click=move |_| toggle(Penalty::Dnf)
                                            >
                                                "DNF"
                                            </button>
                                            <button
                                                class="danger"
                                                title=s.delete
                                                on:click=move |_| data.update(|d| d.delete_solve(id))
                                            >
                                                "×"
                                            </button>
                                        </td>
                                    </tr>
                                }
                            }
                        />
                    </tbody>
                </table>
            </Show>
        </div>
    }
}
