use dioxus::prelude::*;

#[component]
pub fn Dialog(show: bool, onclose: Option<EventHandler<MouseEvent>>, children: Element) -> Element {
    if !show {
        return VNode::empty();
    }
    rsx! {
        div {
            class: "dialog-overlay",
            onclick: move |e| {
                if let Some(cb) = &onclose {
                    cb.call(e);
                }
            },
            {children}
        }
    }
}

#[component]
pub fn DialogContent(class: Option<&'static str>, children: Element) -> Element {
    let base = class
        .map(|c| format!("dialog-content {}", c))
        .unwrap_or_else(|| "dialog-content".to_string());
    rsx! {
        div {
            class: "{base}",
            onclick: move |e| e.stop_propagation(),
            {children}
        }
    }
}
