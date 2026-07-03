use dioxus::prelude::*;

#[component]
pub fn Badge(class: Option<&'static str>, children: Element) -> Element {
    let base = class
        .map(|c| format!("badge {}", c))
        .unwrap_or_else(|| "badge".to_string());
    rsx! {
        span { class: "{base}", {children} }
    }
}
