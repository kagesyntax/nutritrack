use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RingAnim {
    Off,
    On,
}

#[component]
pub fn CalorieRing(current: f64, target: f64, anim: RingAnim) -> Element {
    let ratio = if target > 0.0 {
        (current / target).min(1.0)
    } else {
        0.0
    };
    let anim_class = match anim {
        RingAnim::On => "calorie-ring-animated",
        RingAnim::Off => "",
    };

    rsx! {
        div { class: "calorie-ring {anim_class}",
            svg {
                class: "calorie-ring-svg",
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
                    class: "text-primary calorie-ring-fill",
                    stroke_width: "8",
                    stroke_linecap: "round",
                    stroke_dasharray: "326.73",
                    stroke_dashoffset: "{326.73 - (326.73 * ratio)}",
                }
            }
            div { class: "calorie-ring-label",
                span { class: "calorie-ring-value", "{current:.0}" }
            }
        }
    }
}

#[component]
pub fn MacroBar(
    label: &'static str,
    current: f64,
    target: f64,
    color: &'static str,
    unit: &'static str,
) -> Element {
    let ratio = if target > 0.0 {
        (current / target).min(1.0) * 100.0
    } else {
        0.0
    };

    rsx! {
        div { class: "macro-bar",
            span { class: "macro-bar-label", "{label}" }
            div { class: "macro-bar-track",
                div {
                    class: "macro-bar-fill {color}",
                    style: "width: {ratio:.0}%",
                }
            }
            span { class: "macro-bar-value",
                "{current:.1} / {target:.0}{unit}"
            }
        }
    }
}
