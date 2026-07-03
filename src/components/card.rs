use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum CardSize {
    Sm,
}

#[component]
pub fn Card(
    onclick: Option<EventHandler<MouseEvent>>,
    class: Option<&'static str>,
    size: Option<CardSize>,
    children: Element,
) -> Element {
    let is_clickable = onclick.is_some();
    let hover =
        "cursor-pointer group transition-all duration-200 hover-shadow-md hover--translate-y-0_5";
    let mut classes = vec!["card"];
    if let Some(c) = class {
        classes.push(c);
    }
    if let Some(s) = size {
        match s {
            CardSize::Sm => classes.push("card-sm"),
        }
    }
    if is_clickable {
        classes.push(hover);
    }
    let base = classes.join(" ");
    rsx! {
        div {
            class: "{base}",
            onclick: move |e| {
                if let Some(oc) = &onclick {
                    oc.call(e);
                }
            },
            {children}
        }
    }
}

#[component]
pub fn CardHeader(class: Option<&'static str>, children: Element) -> Element {
    let base = class
        .map(|c| format!("card-header {}", c))
        .unwrap_or_else(|| "card-header".to_string());
    rsx! {
        div { class: "{base}", {children} }
    }
}

#[component]
pub fn CardContent(class: Option<&'static str>, children: Element) -> Element {
    let base = class
        .map(|c| format!("card-content {}", c))
        .unwrap_or_else(|| "card-content".to_string());
    rsx! {
        div { class: "{base}", {children} }
    }
}

#[component]
pub fn CardTitle(class: Option<&'static str>, children: Element) -> Element {
    let base = class
        .map(|c| format!("card-title {}", c))
        .unwrap_or_else(|| "card-title".to_string());
    rsx! {
        h3 { class: "{base}", {children} }
    }
}
