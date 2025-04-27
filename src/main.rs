use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut state = use_signal(|| String::from("123"));
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
    tracing::info!("Child rendered");
    use_effect(move || {
        let current = state.cloned();
        tracing::info!("Child state updated: {}", current);
    });
    use_drop(|| tracing::info!("Child dropped"));

    let num: u32 = state().parse::<u32>()?;
    rsx! {
      p { "Number parsed as {num}" }
    }
}
