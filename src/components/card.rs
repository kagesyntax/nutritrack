use dioxus::prelude::*;

#[component]
pub fn Card(children: Element, class: Option<&'static str>) -> Element {
    let base = "bg-card rounded-xl shadow-sm border border-border";
    let cls = match class {
        Some(c) => format!("{} {}", base, c),
        None => base.to_string(),
    };
    rsx! {
        div { class: "{cls}", {children} }
    }
}

#[component]
pub fn CardHeader(children: Element, class: Option<&'static str>) -> Element {
    let base = "px-6 py-4";
    let cls = match class {
        Some(c) => format!("{} {}", base, c),
        None => base.to_string(),
    };
    rsx! {
        div { class: "{cls}", {children} }
    }
}

#[component]
pub fn CardTitle(children: Element, class: Option<&'static str>) -> Element {
    let base = "text-lg font-semibold";
    let cls = match class {
        Some(c) => format!("{} {}", base, c),
        None => base.to_string(),
    };
    rsx! {
        h3 { class: "{cls}", {children} }
    }
}

#[component]
pub fn CardContent(children: Element, class: Option<&'static str>) -> Element {
    let base = "px-6 py-4";
    let cls = match class {
        Some(c) => format!("{} {}", base, c),
        None => base.to_string(),
    };
    rsx! {
        div { class: "{cls}", {children} }
    }
}
