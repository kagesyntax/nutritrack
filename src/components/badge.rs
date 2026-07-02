use dioxus::prelude::*;

#[component]
pub fn Badge(children: Element, class: Option<&'static str>) -> Element {
    let base = "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium";
    let cls = match class {
        Some(c) => format!("{} {}", base, c),
        None => base.to_string(),
    };
    rsx! {
        span { class: "{cls}", {children} }
    }
}
