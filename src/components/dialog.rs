use dioxus::prelude::*;

#[component]
pub fn Dialog(
    show: bool,
    onclose: EventHandler<()>,
    children: Element,
) -> Element {
    if !show {
        return VNode::empty();
    }

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center",
            div {
                class: "fixed inset-0 bg-black/40",
                onclick: move |_| onclose.call(()),
            }
            div {
                class: "relative bg-card rounded-xl shadow-lg max-w-lg w-full mx-4 max-h-[85vh] overflow-y-auto",
                {children}
            }
        }
    }
}

#[component]
pub fn DialogContent(children: Element, class: Option<&'static str>) -> Element {
    let base = "";
    let cls = match class {
        Some(c) => format!("{} {}", base, c),
        None => base.to_string(),
    };
    rsx! {
        div { class: "{cls}", {children} }
    }
}
