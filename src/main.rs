use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut state = use_signal(|| String::new());
    rsx! {
      input {
        r#type: "text",
        id: "my-input",
        value: state(),
        oninput: move |e| state.set(e.value().clone())
      }
      label {
        for: "my-input",
        "{state}"
      }
      ErrorBoundary {
        Child { state: state }
      }
    }
}

#[component]
fn Child(state: Signal<String>) -> Element {
    let num: u32 = state().parse::<u32>()?;
    rsx! {
      p { "Number parsed as {num}" }
    }
}
