use std::time::Duration;

use leptos::{ev, prelude::*};
use web_sys::KeyboardEvent;

use crate::{i18n::t, stats::format_ms};

const HOLD: Duration = Duration::from_millis(300);
const TICK: Duration = Duration::from_millis(16);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase {
    Idle,
    Holding,
    Ready,
    Running,
    Stopped,
}

impl Phase {
    fn as_str(self) -> &'static str {
        match self {
            Phase::Idle => "idle",
            Phase::Holding => "holding",
            Phase::Ready => "ready",
            Phase::Running => "running",
            Phase::Stopped => "stopped",
        }
    }
}

fn now() -> f64 {
    window().performance().map_or_else(js_sys::Date::now, |p| p.now())
}

fn typing(e: &KeyboardEvent) -> bool {
    use wasm_bindgen::JsCast;
    e.target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
        .is_some_and(|el| {
            el.is_content_editable() || matches!(el.tag_name().as_str(), "INPUT" | "TEXTAREA" | "SELECT")
        })
}

/// Hold-and-release speedcubing timer. Space (or a touch on the readout)
/// arms it, releasing starts it, any key stops it and Escape cancels.
/// While `has_next` is true, Space and Enter call `on_next` instead.
#[component]
pub fn Timer(
    #[prop(into)] on_done: Callback<i64>,
    #[prop(optional, into)] has_next: Signal<bool>,
    #[prop(optional, into)] on_next: Option<Callback<()>>,
) -> impl IntoView {
    let phase = RwSignal::new(Phase::Idle);
    let shown = RwSignal::new(0i64);
    let started = StoredValue::new(0.0f64);
    let hold = StoredValue::new(None::<TimeoutHandle>);
    let ticker = StoredValue::new(None::<IntervalHandle>);

    Effect::new(move |_| {
        let timing = matches!(phase.get(), Phase::Ready | Phase::Running);
        if let Some(body) = document().body() {
            let _ = body.class_list().toggle_with_force("timing", timing);
        }
    });

    let stop_ticker = move || {
        if let Some(h) = ticker.get_value() {
            h.clear();
            ticker.set_value(None);
        }
    };

    let press = move || match phase.get_untracked() {
        Phase::Running => {
            stop_ticker();
            let ms = (now() - started.get_value()).round() as i64;
            shown.set(ms);
            phase.set(Phase::Stopped);
            on_done.run(ms);
        }
        Phase::Idle => {
            phase.set(Phase::Holding);
            let handle = set_timeout_with_handle(
                move || {
                    if phase.get_untracked() == Phase::Holding {
                        shown.set(0);
                        phase.set(Phase::Ready);
                    }
                },
                HOLD,
            );
            hold.set_value(handle.ok());
        }
        _ => {}
    };

    let release = move || match phase.get_untracked() {
        Phase::Holding => {
            if let Some(h) = hold.get_value() {
                h.clear();
            }
            phase.set(Phase::Idle);
        }
        Phase::Ready => {
            started.set_value(now());
            phase.set(Phase::Running);
            let handle = set_interval_with_handle(
                move || shown.set((now() - started.get_value()) as i64),
                TICK,
            );
            ticker.set_value(handle.ok());
        }
        Phase::Stopped => phase.set(Phase::Idle),
        _ => {}
    };

    let s = t();
    let keydown = window_event_listener(ev::keydown, move |e: KeyboardEvent| {
        if typing(&e) || e.meta_key() || e.ctrl_key() || e.alt_key() {
            return;
        }
        let space = e.code() == "Space";
        match phase.get_untracked() {
            Phase::Running => {
                e.prevent_default();
                if e.key() == "Escape" {
                    stop_ticker();
                    shown.set(0);
                    phase.set(Phase::Idle);
                } else {
                    press();
                }
            }
            Phase::Idle if has_next.get_untracked() && (space || e.key() == "Enter") => {
                e.prevent_default();
                if let Some(next) = on_next {
                    next.run(());
                }
            }
            _ if space => {
                e.prevent_default();
                if !e.repeat() {
                    press();
                }
            }
            _ => {}
        }
    });
    let keyup = window_event_listener(ev::keyup, move |e: KeyboardEvent| {
        if e.code() == "Space" || phase.get_untracked() == Phase::Stopped {
            release();
        }
    });
    on_cleanup(move || {
        keydown.remove();
        keyup.remove();
        stop_ticker();
        if let Some(body) = document().body() {
            let _ = body.class_list().remove_1("timing");
        }
    });

    view! {
        <section
            class="timer"
            data-state=move || phase.get().as_str()
            on:touchstart=move |e| {
                e.prevent_default();
                press();
            }
            on:touchend=move |e| {
                e.prevent_default();
                release();
            }
        >
            <div class="timer-display">{move || format_ms(shown.get())}</div>
            <p class="timer-hint">
                {s.hint_hold} <kbd>{s.hint_space}</kbd> {s.hint_rest} <kbd>"Esc"</kbd> {s.hint_cancel}
            </p>
        </section>
    }
}
