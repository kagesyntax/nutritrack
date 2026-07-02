use dioxus::prelude::*;

#[component]
pub fn Card(class: Option<&'static str>, children: Element) -> Element {
    let base = class.map(|c| format!("card {}", c)).unwrap_or_else(|| "card".to_string());
    rsx! {
        div { class: "{base}", {children} }
    }
}

#[component]
pub fn CardHeader(class: Option<&'static str>, children: Element) -> Element {
    let base = class.map(|c| format!("card-header {}", c)).unwrap_or_else(|| "card-header".to_string());
    rsx! {
        div { class: "{base}", {children} }
    }
}

#[component]
pub fn CardContent(class: Option<&'static str>, children: Element) -> Element {
    let base = class.map(|c| format!("card-content {}", c)).unwrap_or_else(|| "card-content".to_string());
    rsx! {
        div { class: "{base}", {children} }
    }
}

#[component]
pub fn CardTitle(class: Option<&'static str>, children: Element) -> Element {
    let base = class.map(|c| format!("card-title {}", c)).unwrap_or_else(|| "card-title".to_string());
    rsx! {
        h3 { class: "{base}", {children} }
    }
}
