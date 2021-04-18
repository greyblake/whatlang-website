//  - it's useful when you want to check your code with `cargo make verify`
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
use whatlang::dev::{
    detect, raw_detect, Info, RawAlphabetsInfo, RawCombinedInfo, RawInfo, RawLangInfo,
    RawScriptInfo, RawTrigramsInfo,
};

mod icon;
use icon::Icon;

mod demo;
use demo::Show;

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.stream(streams::interval(100, || Msg::OnTick));

    let mut show = Show::gen_random();
    let text = show.next().unwrap_or("".to_string());
    Model {
        mode: Mode::Auto(0),
        info: detect(&text),
        raw_info: raw_detect(&text),
        text,
        tab: Tab::Language,
        show,
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    mode: Mode,
    text: String,
    info: Option<Info>,
    raw_info: RawInfo,
    tab: Tab,
    show: Show,
}

enum Mode {
    // Demo mode, when text is typed itself
    Auto(usize),

    // When user started interacting
    Manual,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Tab {
    Language,
    Debug,
}

impl Tab {
    fn name_and_icon(self) -> (&'static str, Icon) {
        match self {
            Tab::Language => ("Language", Icon::Language),
            Tab::Debug => ("Debug", Icon::DraftingCompass),
        }
    }
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Debug, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    UpdateText(String),
    OnTick,
    ChangeTab(Tab),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::UpdateText(text) => {
            match model.mode {
                Mode::Auto(_) => {
                    let manually_entered_text = text
                        .chars()
                        .last()
                        .map(|c| c.to_string())
                        .unwrap_or_else(|| "".to_string());
                    model.text = manually_entered_text;
                }
                Mode::Manual => {
                    model.text = text;
                }
            }
            model.mode = Mode::Manual;
            model.info = detect(&model.text);
            model.raw_info = raw_detect(&model.text);
        }
        Msg::OnTick => match model.mode {
            Mode::Auto(pos) => {
                let text = model.show.next().unwrap_or_else(|| "".to_string());
                model.info = detect(&text);
                model.raw_info = raw_detect(&text);
                model.text = text;
                model.mode = Mode::Auto(pos + 1);
            }
            Mode::Manual => (),
        },
        Msg::ChangeTab(tab) => {
            model.tab = tab;
        }
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        div![
            C!["container"],
            textarea![
                C!["textarea"],
                attrs! {
                    At::Value => model.text
                },
                input_ev(Ev::Input, Msg::UpdateText),
            ],
        ],
        br![],
        div![C!["container"], view_tabs(model), view_ouput(model),]
    ]
}

fn view_tabs(model: &Model) -> Node<Msg> {
    div![
        C!["tabs"],
        ul![
            view_tab_li(model, Tab::Language),
            view_tab_li(model, Tab::Debug),
        ],
    ]
}

fn view_tab_li(model: &Model, tab: Tab) -> Node<Msg> {
    let (name, icon) = tab.name_and_icon();

    li![
        input_ev(Ev::Click, move |_| Msg::ChangeTab(tab)),
        C![IF!(tab == model.tab => "is-active")],
        a![view_icon(icon), span![name],],
    ]
}

fn view_icon(icon: Icon) -> Node<Msg> {
    span![
        C!["icon is-small"],
        i![
            C![format!("fas {}", icon.to_class())],
            attrs! { "aria-hidden" => true }
        ],
    ]
}

fn view_ouput(model: &Model) -> Node<Msg> {
    match model.tab {
        Tab::Language => view_info(&model.info),
        Tab::Debug => view_debug_info(&model.raw_info),
    }
}

fn view_debug_info(raw_info: &RawInfo) -> Node<Msg> {
    let second_phase_columns = match raw_info.lang_info {
        None => vec![text![]],
        Some(ref lang_info) => view_debug_second_phase(lang_info),
    };

    div![
        C!["columns"],
        div![
            C!["column"],
            h5![C!["title is-5"], "Script Scores"],
            view_script_info_as_table(&raw_info.script_info),
        ],
        second_phase_columns
    ]
}

fn view_debug_second_phase(info: &RawLangInfo) -> Vec<Node<Msg>> {
    match info {
        RawLangInfo::OneScript(lang) | RawLangInfo::Mandarin(lang) => {
            vec![div![C!["column"], "Language", pre![lang.eng_name()]]]
        }
        RawLangInfo::MultiScript(combined) => {
            vec![
                div![
                    C!["column"],
                    h5![C!["title is-5"], "Alphabet Scores"],
                    view_alphabets_info_as_table(&combined.alphabet_raw_outcome),
                ],
                div![
                    C!["column"],
                    h5![C!["title is-5"], "Trigram Distances"],
                    view_trigrams_info_as_table(&combined.trigram_raw_outcome),
                ],
                div![
                    C!["column"],
                    h5![C!["title is-5"], "Combined Scores"],
                    view_combined_info_as_table(combined),
                ],
            ]
        }
    }
}

fn view_script_info_as_table(info: &RawScriptInfo) -> Node<Msg> {
    let rows = info
        .counters
        .iter()
        .map(|(script, count)| tr![td![script.name()], td![count]]);

    table![C!["table is-striped is-fullwidth"], tbody![rows],]
}

fn view_alphabets_info_as_table(info: &RawAlphabetsInfo) -> Node<Msg> {
    let rows = info
        .raw_scores
        .iter()
        .map(|(lang, count)| tr![td![lang.eng_name()], td![count]]);

    table![C!["table is-striped is-fullwidth"], tbody![rows],]
}

fn view_trigrams_info_as_table(info: &RawTrigramsInfo) -> Node<Msg> {
    let rows = info
        .raw_distances
        .iter()
        .map(|(lang, dist)| tr![td![lang.eng_name()], td![dist]]);

    table![C!["table is-striped is-fullwidth"], tbody![rows]]
}

fn view_combined_info_as_table(info: &RawCombinedInfo) -> Node<Msg> {
    let rows = info.scores.iter().map(|(lang, score)| {
        let rounded_score = format!("{:.4}", score);
        tr![td![lang.eng_name()], td![rounded_score]]
    });

    table![C!["table is-striped is-fullwidth"], tbody![rows]]
}

fn view_info(info: &Option<Info>) -> Node<Msg> {
    div![
        C!["columns"],
        div![
            C!["column"],
            h5![C!["title is-5"], "Human Output"],
            view_human_output(info)
        ],
        div![
            C!["column"],
            h5![C!["title is-5"], "Rust Output"],
            pre![format_rust_output(info)]
        ],
    ]
}

fn view_human_output(info: &Option<Info>) -> Node<Msg> {
    match info {
        Some(info) => {
            let is_reliable = if info.is_reliable() { "Yes" } else { "No" };
            let lang = info.lang();
            let lang_name = if lang.name() == lang.eng_name() {
                lang.name().to_string()
            } else {
                format!("{} ({})", lang.name(), lang.eng_name())
            };
            p![
                "Language: ",
                lang_name,
                br![],
                "Script: ",
                info.script().name(),
                br![],
                "Is reliable: ",
                is_reliable,
                br![],
                "Confidence: ",
                (info.confidence() * 100.0).round(),
                "%"
            ]
        }
        None => {
            p!["The input text is too scarce to detect anything. Try to type something meaningful."]
        }
    }
}

fn format_rust_output(info: &Option<Info>) -> String {
    format!("{:#?}", info)
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
