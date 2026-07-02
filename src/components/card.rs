use dioxus::prelude::*;

#[component]
pub fn Card(
    onclick: Option<EventHandler<MouseEvent>>,
    class: Option<&'static str>,
    children: Element,
) -> Element {
    let is_clickable = onclick.is_some();
    let hover = "cursor-pointer group transition-all duration-200 hover:shadow-md hover:-translate-y-0_5";
    let base = class.map(|c| {
        if is_clickable {
            format!("card {} {}", c, hover)
        } else {
            format!("card {}", c)
        }
    }).unwrap_or_else(|| {
        if is_clickable {
            format!("card {}", hover)
        } else {
            "card".to_string()
        }
    });
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
