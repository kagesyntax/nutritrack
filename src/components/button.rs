use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ButtonVariant {
    Default,
    Ghost,
    Outline,
    Primary,
    Destructive,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ButtonSize {
    Sm,
    Md,
    Lg,
}

#[component]
pub fn Button(
    children: Element,
    variant: Option<ButtonVariant>,
    size: Option<ButtonSize>,
    class: Option<&'static str>,
    onclick: Option<EventHandler<MouseEvent>>,
    disabled: Option<bool>,
) -> Element {
    let variant = variant.unwrap_or(ButtonVariant::Default);
    let size = size.unwrap_or(ButtonSize::Md);

    let variant_cls = match variant {
        ButtonVariant::Default => "bg-primary text-white hover:bg-primary-dark",
        ButtonVariant::Ghost => "text-muted-foreground hover:bg-muted hover:text-foreground",
        ButtonVariant::Outline => "border border-border text-foreground hover:bg-muted",
        ButtonVariant::Primary => "bg-primary text-white hover:bg-primary-dark",
        ButtonVariant::Destructive => "bg-destructive text-white hover:bg-red-600",
    };

    let size_cls = match size {
        ButtonSize::Sm => "px-2.5 py-1.5 text-xs rounded-lg",
        ButtonSize::Md => "px-4 py-2 text-sm rounded-lg",
        ButtonSize::Lg => "px-6 py-3 text-base rounded-lg",
    };

    let base = "inline-flex items-center justify-center font-medium transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-primary/30 disabled:pointer-events-none disabled:opacity-50";
    let cls = match class {
        Some(c) => format!("{} {} {} {}", base, variant_cls, size_cls, c),
        None => format!("{} {} {}", base, variant_cls, size_cls),
    };

    rsx! {
        button {
            class: "{cls}",
            onclick: move |e| {
                if let Some(cb) = &onclick {
                    cb.call(e);
                }
            },
            disabled: disabled.unwrap_or(false),
            {children}
        }
    }
}
