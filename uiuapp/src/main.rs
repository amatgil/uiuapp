#![allow(non_snake_case)]

use crate::document::*;
use base64::engine::general_purpose;
use base64::Engine;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use uiuapp::ScrollbackItem as SBI;
use uiuapp::*;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

#[component]
fn App() -> Element {
    static CSS: Asset = asset!("/assets/uiuapp.css");
    static _UIUA386: Asset = asset!("/assets/Uiua386.ttf");

    // the text that's been input and evaluated
    // populated for testing
    // TODO(release): depopulate
    let mut buffer_contents = use_signal(|| {
        let code = "˙⊞=⇡3";
        let output = SBI::Output(vec![run_uiua(code).unwrap()[0].clone()]);
        let c = SBI::Input(highlight_code(code));

        vec![
            SBI::Input(highlight_code("+ 1 1")),
            SBI::Output(vec![ScrollbackOutput::Text("2".to_string())]),
            c.clone(),
            output.clone(),
            c.clone(),
            output.clone(),
            c.clone(),
            output.clone(),
        ]
    });
    // Has been input but not yet evaluated
    let mut input_contents = use_signal(String::new);
    let touch_info: Signal<Option<LastTouchContext>> = use_signal(|| None);
    let rad_info: Signal<RadialInfo> = use_signal(RadialInfo::new);
    let mut settings: Signal<Settings> = use_signal(Settings::default);

    rsx! {
        Meta { charset: "UTF-8" }
        Meta {
            content: "width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, viewport-fit=cover",
            name: "viewport",
        }
        Title { "cas/uiuapp" }
        Stylesheet { href: CSS }

        div { class: "app",
            div { class: "top-bar",
                button {
                    onclick: move |e| {
                        info!("Settings button pressed (unimplemented as of yet)");
                        // Settings button toggles this specific one, for testing
                        // TODO: implement an actual settings menu
                        let b = settings.read().clean_input_on_run;
                        settings.write().clean_input_on_run = !b;
                    },
                    "Settings"
                }
            }
            div { class: "code-view-zone",
                for item in buffer_contents.read().clone() {
                    {
                        match item {
                            SBI::Input(input) => {
                                rsx! {
                                    p { class: "user-input",
                                    onclick: move |_e| {
                                        if input_contents().is_empty() {
                                            *input_contents.write() = match input {
                                                Ok(ref v) => v.iter().map(|uhs| match uhs {
                                                    // Bunch of cloning, this should be benchmarked
                                                    UiuappHistorySpan::UnstyledCode { text } => text.clone(),
                                                    UiuappHistorySpan::StyledCode { text, .. } => text.clone(),
                                                    UiuappHistorySpan::Whitspace(text) => text.clone(),
                                                }).collect::<Vec<String>>().join(""),
                                                Err(ref s) => s.to_string()
                                            };
                                        }
                                    },
                                        match input {
                                            Ok(ref v) => {
                                                rsx! {
                                                    for uhs in v {
                                                        match uhs {
                                                            UiuappHistorySpan::UnstyledCode { text } => rsx! { span { "{text}" } },
                                                            UiuappHistorySpan::StyledCode { class: c, text } => rsx! { span { class: "{c}", "{text}"} },
                                                            UiuappHistorySpan::Whitspace(text) => rsx! { span { "{text}" } },
                                                        }
                                                    }
                                                }
                                            }
                                            Err(ref s) => rsx! { span { "{s}" } }
                                        }

                                    }
                                }
                            },
                            SBI::Output(outputs) => {
                                let outputs = match settings.read().stack_ordering {
                                    StackOrdering::TopAtTop => outputs,
                                    StackOrdering::BottomAtTop => outputs.into_iter().rev().collect(),
                                };
                                rsx! {
                                    for output in outputs {
                                        match output {
                                            ScrollbackOutput::Text(text) => {
                                                info!("TEXT");
                                                rsx! {
                                                    p { class: "user-result", "{text}" }
                                                }
                                            },
                                            ScrollbackOutput::Image(bytes) => {
                                                let data = general_purpose::STANDARD.encode(&bytes);
                                                rsx! {
                                                    img { class: "user-result", src: "data:image/png;base64,{data}" }
                                                }
                                            },
                                            ScrollbackOutput::Audio(bytes) => {
                                                let data = general_purpose::STANDARD.encode(&bytes);
                                                rsx! {
                                                    audio { class: "user-result", controls: true,
                                                            autoplay: settings.read().autoplay_audio,
                                                            src: "data:audio/wav;base64,{data}" }
                                                }
                                            },
                                            ScrollbackOutput::Gif(bytes) => {
                                                let data = general_purpose::STANDARD.encode(&bytes);
                                                rsx! {
                                                    img { class: "user-result", src: "data:image/gif;base64,{data}" }
                                                }
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
              div { class: "input-zone",
                    RadialSelector { input_contents, rad_info }
                    div { class: "input-bar",
                    // This textarea should bring up the native keyboard for
                    // ascii-and-related typing
                          textarea { class: "text-box", rows: 2,
                                     onkeydown: move |e| {
                                         if let Key::Enter = e.key() {
                                             info!("Return gotten");
                                             if e.modifiers().contains(Modifiers::CONTROL) {
                                                 e.prevent_default();
                                                 info!("Running from shortcut");
                                                 handle_running_code(input_contents, buffer_contents, settings);
                                             }
                                         }
                                     },
                                     onchange: move |e| {
                                         *input_contents.write() = e.value();
                                     },
                                     value: input_contents }
                          button { class: "run-button",
                                   onclick: move |e| {
                                       handle_running_code(input_contents, buffer_contents, settings);
                                   },
                                   "Run" },
                    }
                    div { class: "special-buttons",
                          button { class: "special-button", onclick: move |_| {input_contents.write().push('\n');}, "Ret" }
                          button { class: "special-button", onclick: move |_| {*buffer_contents.write() = vec![];}, "Clear Past" }
                          button { class: "special-button", onclick: move |_| {*input_contents.write() = "".to_string();}, "Clear Curr" }
                          button { class: "special-button", onclick: move |_| {input_contents.write().push(';');}, ";" }
                          // TODO: Decide if these arrows should even exist
                          /*button { class: "special-button", "←" } // TODO: position cursor
                          button { class: "special-button", "↓" }
                          button { class: "special-button", "↑" }
                          button { class: "special-button", "→" }*/
                          button { class: "special-button", onclick: move |_| {input_contents.write().pop();}, "Bksp" }
                    }
                    div { class: "input-grid-buttons",
                           ButtonIcons { input_contents, rad_info }
                    }
              }
        }
    }
}
