use dioxus::prelude::*;

#[component]
pub fn CalorieRing(current: f64, target: f64) -> Element {
    let ratio = if target > 0.0 { (current / target).min(1.0) } else { 0.0 };
    let remaining = (target - current).max(0.0);

    rsx! {
        div { class: "flex flex-col items-center gap-2",
            div { class: "relative w-36 h-36",
                svg {
                    class: "transform -rotate-90 w-full h-full",
                    view_box: "0 0 120 120",
                    circle {
                        cx: "60", cy: "60", r: "52",
                        fill: "none",
                        stroke: "currentColor",
                        class: "text-surface-tertiary",
                        stroke_width: "8",
                    }
                    circle {
                        cx: "60", cy: "60", r: "52",
                        fill: "none",
                        stroke: "currentColor",
                        class: "text-primary",
                        stroke_width: "8",
                        stroke_linecap: "round",
                        stroke_dasharray: "326.73",
                        stroke_dashoffset: "{326.73 - (326.73 * ratio)}",
                    }
                }
                div { class: "absolute inset-0 flex flex-col items-center justify-center",
                    span { class: "text-3xl font-bold tabular-nums text-text-primary", "{current:.0}" }
                    span { class: "text-xs text-text-tertiary", "of {target:.0} kcal" }
                }
            }
            span { class: "text-xs text-text-secondary", "{remaining:.0} kcal remaining" }
        }
    }
}

#[component]
pub fn MacroBar(
    label: &'static str,
    current: f64,
    target: f64,
    color_class: &'static str,
    unit: &'static str,
) -> Element {
    let ratio = if target > 0.0 { (current / target).min(1.0) * 100.0 } else { 0.0 };

    rsx! {
        div { class: "flex items-center gap-3",
            span { class: "w-12 text-xs font-medium text-text-secondary shrink-0", "{label}" }
            div { class: "flex-1 h-2 rounded-full bg-surface-tertiary overflow-hidden",
                div {
                    class: "h-full rounded-full transition-all duration-300 {color_class}",
                    style: "width: {ratio:.0}%",
                }
            }
            span { class: "w-24 text-right text-sm tabular-nums text-text-primary",
                "{current:.1} / {target:.0}{unit}"
            }
        }
    }
}
