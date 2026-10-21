use leptos::prelude::*;

use crate::{
    algs::{CheckError, Group},
    store::Status,
};

const KEY: &str = "fridrich-trainer.lang";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Lang {
    #[default]
    En,
    Uk,
}

impl Lang {
    pub const ALL: [Lang; 2] = [Lang::En, Lang::Uk];

    pub fn code(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Uk => "uk",
        }
    }

    pub fn strings(self) -> &'static Strings {
        match self {
            Lang::En => &EN,
            Lang::Uk => &UK,
        }
    }
}

pub struct Strings {
    pub nav_timer: &'static str,
    pub nav_algs: &'static str,
    pub nav_train: &'static str,
    pub nav_data: &'static str,

    pub scramble: &'static str,
    pub new_scramble: &'static str,
    pub solves: &'static str,
    pub best: &'static str,
    pub mean: &'static str,
    pub best_ao5: &'static str,
    pub best_ao12: &'static str,
    pub no_solves: &'static str,
    pub time: &'static str,
    pub delete: &'static str,
    pub hint_hold: &'static str,
    pub hint_space: &'static str,
    pub hint_rest: &'static str,
    pub hint_cancel: &'static str,

    pub status_new: &'static str,
    pub status_learning: &'static str,
    pub status_learned: &'static str,
    pub all: &'static str,
    pub train_learning: &'static str,
    pub empty_filter: &'static str,
    pub best_and_attempts: &'static str,
    pub learned_summary: fn(usize, usize, usize) -> String,

    pub no_such_case: &'static str,
    pub to_algs: &'static str,
    pub algorithm: &'static str,
    pub my_version: &'static str,
    pub set_up_case: &'static str,
    pub set_up_hint: &'static str,
    pub custom_alg: &'static str,
    pub custom_placeholder: &'static str,
    pub save: &'static str,
    pub custom_hint: &'static str,
    pub practice: &'static str,
    pub train_this: &'static str,
    pub attempts: &'static str,
    pub last: &'static str,

    pub any_status: &'static str,
    pub in_pool: &'static str,
    pub no_cases: &'static str,
    pub no_cases_link: &'static str,
    pub no_cases_end: &'static str,
    pub set_up: &'static str,
    pub peek: &'static str,
    pub next_case: &'static str,
    pub next_hint: &'static str,

    pub data_title: &'static str,
    pub data_info: &'static str,
    pub export: &'static str,
    pub import: &'static str,
    pub reset_all: &'static str,
    pub download_failed: &'static str,
    pub confirm_import: &'static str,
    pub imported: &'static str,
    pub bad_file: &'static str,
    pub confirm_reset: &'static str,
    pub reset_done: &'static str,
    pub data_summary: fn(usize, usize, usize) -> String,

    pub notation: &'static str,
    pub notation_button_title: &'static str,
    pub close: &'static str,
    pub notation_rules: [&'static str; 3],
    pub notation_groups: [&'static str; 4],
    /// Descriptions of R L U D F B, r l u d f b, M E S, x y z.
    pub moves: [&'static str; 18],

    pub breaks_solved_part: &'static str,
    pub wrong_case: &'static str,
    pub unknown_move: &'static str,

    pub groups: fn(Group) -> &'static str,
}

impl Strings {
    pub fn status(&self, status: Status) -> &'static str {
        match status {
            Status::New => self.status_new,
            Status::Learning => self.status_learning,
            Status::Learned => self.status_learned,
        }
    }

    pub fn check_error(&self, e: &CheckError) -> String {
        match e {
            CheckError::Parse(token) => format!("{} “{token}”", self.unknown_move),
            CheckError::BreaksSolvedPart => self.breaks_solved_part.to_owned(),
            CheckError::WrongCase => self.wrong_case.to_owned(),
        }
    }

    pub fn group(&self, group: Group) -> &'static str {
        (self.groups)(group)
    }
}

static EN: Strings = Strings {
    nav_timer: "Timer",
    nav_algs: "Algorithms",
    nav_train: "Practice",
    nav_data: "Data",

    scramble: "Scramble",
    new_scramble: "new",
    solves: "Solves",
    best: "Best",
    mean: "Mean",
    best_ao5: "Best ao5",
    best_ao12: "Best ao12",
    no_solves: "No solves yet. Scramble your cube and press space.",
    time: "Time",
    delete: "Delete",
    hint_hold: "Hold ",
    hint_space: "space",
    hint_rest: " (or touch the screen), release to start. Any key stops, ",
    hint_cancel: " cancels.",

    status_new: "Not started",
    status_learning: "Learning",
    status_learned: "Learned",
    all: "All",
    train_learning: "Practice “Learning” →",
    empty_filter: "Nothing here — try another filter.",
    best_and_attempts: "Best time · attempts",
    learned_summary: |learned, total, learning| {
        let mut s = format!("{learned} of {total} learned");
        if learning > 0 {
            s.push_str(&format!(", {learning} in progress"));
        }
        s
    },

    no_such_case: "No such case. ",
    to_algs: "Back to algorithms",
    algorithm: "Algorithm",
    my_version: "My version",
    set_up_case: "Set up the case",
    set_up_hint: "From a solved cube (yellow on top, green in front):",
    custom_alg: "Your own algorithm",
    custom_placeholder: "e.g. R U R' U R U2 R'",
    save: "Save",
    custom_hint: "Checked on a simulator: it has to solve this very case. Leave empty to use the standard one.",
    practice: "Practice",
    train_this: "Practice this case",
    attempts: "Attempts",
    last: "Last",

    any_status: "Any",
    in_pool: "In the pool: ",
    no_cases: "No cases match this filter. Mark algorithms as “Learning” on the ",
    no_cases_link: "Algorithms",
    no_cases_end: " page or pick “Any”.",
    set_up: "Set up:",
    peek: "Peek at the algorithm",
    next_case: "Next case →",
    next_hint: "Space or Enter — next.",

    data_title: "Data",
    data_info: "Everything is stored only in this browser (localStorage). Clearing site data, another browser or another device — and the progress is not there. Export from time to time.",
    export: "Export to JSON",
    import: "Import from JSON",
    reset_all: "Delete everything",
    download_failed: "Could not download the file.",
    confirm_import: "Replace the current data with the file contents?",
    imported: "Data loaded.",
    bad_file: "This file does not look like an export of this app.",
    confirm_reset: "Delete all solves, statuses and practice times? This cannot be undone.",
    reset_done: "Everything deleted.",
    data_summary: |solves, marked, times| {
        format!("{solves} solves, {marked} marked cases, {times} practice attempts")
    },

    notation: "Notation",
    notation_button_title: "Notation (press ?)",
    close: "Close",
    notation_rules: [
        "A letter is a 90° clockwise turn as seen looking straight at that face. ",
        " — counter-clockwise, ",
        " — 180°. Pictures show a solved cube after the move (yellow on top, green in front).",
    ],
    notation_groups: ["Faces", "Wide", "Slices", "Rotations"],
    moves: [
        "right face",
        "left face",
        "top face",
        "bottom face",
        "front face",
        "back face",
        "R together with the middle layer (Rw)",
        "L together with the middle layer (Lw)",
        "U together with the middle layer (Uw)",
        "D together with the middle layer (Dw)",
        "F together with the middle layer (Fw)",
        "B together with the middle layer (Bw)",
        "between L and R, like L",
        "between U and D, like D",
        "between F and B, like F",
        "whole cube like R",
        "whole cube like U",
        "whole cube like F",
    ],

    breaks_solved_part: "the algorithm breaks the solved part of the cube",
    wrong_case: "the algorithm solves a different case",
    unknown_move: "unknown move",

    groups: |g| match g {
        Group::Basic => "Basic inserts",
        Group::BothInTop => "Corner and edge on top",
        Group::EdgeInSlot => "Edge in slot",
        Group::CornerInSlot => "Corner in slot",
        Group::BothInSlot => "Corner and edge in slot",
        Group::Dot => "Dot",
        Group::Square => "Square",
        Group::Lightning => "Lightning bolt",
        Group::Fish => "Fish",
        Group::KnightMove => "Knight move",
        Group::Cross => "Cross (OCLL)",
        Group::CornersOriented => "Corners oriented",
        Group::Awkward => "Awkward",
        Group::P => "P shape",
        Group::T => "T shape",
        Group::C => "C shape",
        Group::W => "W shape",
        Group::L => "L shape",
        Group::I => "I shape",
        Group::CornersOnly => "Corners only",
        Group::EdgesOnly => "Edges only",
        Group::AdjacentSwap => "Adjacent corner swap",
        Group::DiagonalSwap => "Diagonal corner swap",
        Group::G => "G perms",
    },
};

static UK: Strings = Strings {
    nav_timer: "Таймер",
    nav_algs: "Алгоритми",
    nav_train: "Тренування",
    nav_data: "Дані",

    scramble: "Скрембл",
    new_scramble: "інший",
    solves: "Збірок",
    best: "Найкраща",
    mean: "Середнє",
    best_ao5: "Найкраще ao5",
    best_ao12: "Найкраще ao12",
    no_solves: "Ще немає збірок. Переміщай кубик за скремблом і тисни пробіл.",
    time: "Час",
    delete: "Видалити",
    hint_hold: "Затисни ",
    hint_space: "пробіл",
    hint_rest: " (або палець на екрані), відпусти — старт. Будь-яка клавіша — стоп, ",
    hint_cancel: " — скасувати.",

    status_new: "Не вчив",
    status_learning: "Вчу",
    status_learned: "Вивчив",
    all: "Усі",
    train_learning: "Тренувати «Вчу» →",
    empty_filter: "Тут порожньо — зміни фільтр.",
    best_and_attempts: "Найкращий час · спроб",
    learned_summary: |learned, total, learning| {
        let mut s = format!("{learned} з {total} вивчено");
        if learning > 0 {
            s.push_str(&format!(", {learning} в процесі"));
        }
        s
    },

    no_such_case: "Такого випадку немає. ",
    to_algs: "До алгоритмів",
    algorithm: "Алгоритм",
    my_version: "Мій варіант",
    set_up_case: "Виставити випадок",
    set_up_hint: "Із зібраного кубика (жовтий згори, зелений спереду):",
    custom_alg: "Свій алгоритм",
    custom_placeholder: "наприклад, R U R' U R U2 R'",
    save: "Зберегти",
    custom_hint: "Перевіряється на симуляторі: має розв'язувати саме цей випадок. Порожнє поле — повернути стандартний.",
    practice: "Тренування",
    train_this: "Тренувати цей випадок",
    attempts: "Спроб",
    last: "Остання",

    any_status: "Будь-які",
    in_pool: "У вибірці: ",
    no_cases: "Немає випадків під цей фільтр. Познач алгоритми як «Вчу» на сторінці ",
    no_cases_link: "Алгоритми",
    no_cases_end: " або обери «Будь-які».",
    set_up: "Виставити:",
    peek: "Підглянути алгоритм",
    next_case: "Наступний випадок →",
    next_hint: "Пробіл або Enter — наступний.",

    data_title: "Дані",
    data_info: "Усе зберігається лише в цьому браузері (localStorage). Очищення даних сайту, інший браузер чи пристрій — і прогресу там не буде. Час від часу роби експорт.",
    export: "Експорт у JSON",
    import: "Імпорт із JSON",
    reset_all: "Видалити все",
    download_failed: "Не вдалося завантажити файл.",
    confirm_import: "Замінити поточні дані вмістом файлу?",
    imported: "Дані завантажено.",
    bad_file: "Файл не схожий на експорт цього застосунку.",
    confirm_reset: "Видалити всі збірки, статуси й тренування? Це не скасувати.",
    reset_done: "Усе видалено.",
    data_summary: |solves, marked, times| {
        format!("Збірок: {solves}, позначених випадків: {marked}, тренувальних спроб: {times}")
    },

    notation: "Нотація",
    notation_button_title: "Нотація (клавіша ?)",
    close: "Закрити",
    notation_rules: [
        "Літера — поворот на 90° за годинниковою стрілкою, якщо дивитися прямо на цю грань. ",
        " — проти годинникової, ",
        " — на 180°. Картинка показує зібраний кубик після ходу (жовтий згори, зелений спереду).",
    ],
    notation_groups: ["Грані", "Широкі", "Середні шари", "Перехоплення"],
    moves: [
        "права грань",
        "ліва грань",
        "верхня грань",
        "нижня грань",
        "передня грань",
        "задня грань",
        "R разом із середнім шаром (Rw)",
        "L разом із середнім шаром (Lw)",
        "U разом із середнім шаром (Uw)",
        "D разом із середнім шаром (Dw)",
        "F разом із середнім шаром (Fw)",
        "B разом із середнім шаром (Bw)",
        "між L і R, у бік L",
        "між U і D, у бік D",
        "між F і B, у бік F",
        "увесь кубик як R",
        "увесь кубик як U",
        "увесь кубик як F",
    ],

    breaks_solved_part: "алгоритм ламає зібрану частину кубика",
    wrong_case: "алгоритм розв'язує інший випадок",
    unknown_move: "незрозумілий хід",

    groups: |g| match g {
        Group::Basic => "Базові",
        Group::BothInTop => "Кут і ребро згори",
        Group::EdgeInSlot => "Ребро в слоті",
        Group::CornerInSlot => "Кут у слоті",
        Group::BothInSlot => "Кут і ребро в слоті",
        Group::Dot => "Крапка",
        Group::Square => "Квадрат",
        Group::Lightning => "Блискавка",
        Group::Fish => "Риба",
        Group::KnightMove => "Хід конем",
        Group::Cross => "Хрест (OCLL)",
        Group::CornersOriented => "Кути орієнтовані",
        Group::Awkward => "Незручні",
        Group::P => "P-форма",
        Group::T => "T-форма",
        Group::C => "C-форма",
        Group::W => "W-форма",
        Group::L => "L-форма",
        Group::I => "I-форма",
        Group::CornersOnly => "Лише кути",
        Group::EdgesOnly => "Лише ребра",
        Group::AdjacentSwap => "Сусідні кути",
        Group::DiagonalSwap => "Діагональ",
        Group::G => "G-перми",
    },
};

#[derive(Clone, Copy)]
pub struct I18n(pub RwSignal<Lang>);

fn load() -> Lang {
    window()
        .local_storage()
        .ok()
        .flatten()
        .and_then(|s| s.get_item(KEY).ok().flatten())
        .and_then(|code| Lang::ALL.into_iter().find(|l| l.code() == code))
        .unwrap_or_default()
}

impl I18n {
    /// Restores the saved language, keeps it saved and mirrored in the
    /// document's `lang` attribute, and provides it through [`use_lang`].
    pub fn provide() -> RwSignal<Lang> {
        let lang = RwSignal::new(load());
        Effect::new(move |_| {
            let lang = lang.get();
            if let Some(storage) = window().local_storage().ok().flatten() {
                let _ = storage.set_item(KEY, lang.code());
            }
            if let Some(root) = document().document_element() {
                let _ = root.set_attribute("lang", lang.code());
            }
        });
        provide_context(I18n(lang));
        lang
    }
}

pub fn use_lang() -> RwSignal<Lang> {
    expect_context::<I18n>().0
}

/// Strings of the current language. Views re-render as a whole when the
/// language changes, so reading it untracked is enough.
pub fn t() -> &'static Strings {
    use_lang().get_untracked().strings()
}
