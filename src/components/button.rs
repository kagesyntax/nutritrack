use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Default,
    Ghost,
    Outline,
    Primary,
    Destructive,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonSize {
    Sm,
    Md,
    Lg,
}

#[component]
pub fn Button(
    variant: Option<ButtonVariant>,
    size: Option<ButtonSize>,
    class: Option<&'static str>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let variant_class = match variant.unwrap_or(ButtonVariant::Default) {
        ButtonVariant::Default => "btn-default",
        ButtonVariant::Ghost => "btn-ghost",
        ButtonVariant::Outline => "btn-outline",
        ButtonVariant::Primary => "btn-primary",
        ButtonVariant::Destructive => "btn-destructive",
    };
    let size_class = match size.unwrap_or(ButtonSize::Md) {
        ButtonSize::Sm => "btn-sm",
        ButtonSize::Md => "btn-md",
        ButtonSize::Lg => "btn-lg",
    };
    let extra = class.unwrap_or("");
    rsx! {
        button {
            class: "btn {variant_class} {size_class} {extra}",
            onclick: move |e| {
                if let Some(cb) = &onclick {
                    cb.call(e);
                }
            },
            {children}
        }
    }
}
