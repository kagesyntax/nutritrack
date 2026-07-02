use dioxus::prelude::*;

use crate::components::button::{Button, ButtonVariant};
use crate::components::card::{Card, CardContent, CardHeader, CardTitle};

#[component]
pub fn Settings() -> Element {
    let mut settings = use_context::<Signal<crate::state::models::UserSettings>>();
    let mut logs = use_context::<Signal<Vec<crate::state::models::DayLog>>>();

    rsx! {
        div { class: "max-w-2xl mx-auto p-6 space-y-6",
            h1 { class: "text-2xl font-bold text-foreground font-heading", "Settings" }

            Card {
                CardHeader {
                    CardTitle { class: "font-heading", "Daily Nutrition Targets" }
                }
                CardContent {
                    div { class: "space-y-5",
                        TargetInput {
                            label: "Calories",
                            value: settings.read().targets.calories,
                            unit: "kcal",
                            on_change: move |v| {
                                settings.write().targets.calories = v;
                            },
                        }
                        TargetInput {
                            label: "Protein",
                            value: settings.read().targets.protein_g,
                            unit: "g",
                            on_change: move |v| {
                                settings.write().targets.protein_g = v;
                            },
                        }
                        TargetInput {
                            label: "Carbs",
                            value: settings.read().targets.carbs_g,
                            unit: "g",
                            on_change: move |v| {
                                settings.write().targets.carbs_g = v;
                            },
                        }
                        TargetInput {
                            label: "Fat",
                            value: settings.read().targets.fat_g,
                            unit: "g",
                            on_change: move |v| {
                                settings.write().targets.fat_g = v;
                            },
                        }
                        TargetInput {
                            label: "Fiber",
                            value: settings.read().targets.fiber_g,
                            unit: "g",
                            on_change: move |v| {
                                settings.write().targets.fiber_g = v;
                            },
                        }
                    }
                }
            }

            Card {
                CardHeader {
                    CardTitle { class: "font-heading", "Data" }
                }
                CardContent {
                    div { class: "flex gap-3",
                        Button {
                            variant: ButtonVariant::Destructive,
                            onclick: move |_| {
                                logs.write().clear();
                            },
                            "Clear All Data"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TargetInput(
    label: &'static str,
    value: f64,
    unit: &'static str,
    on_change: EventHandler<f64>,
) -> Element {
    let mut val = use_signal(|| value.to_string());

    rsx! {
        div { class: "flex flex-col sm:flex-row sm:items-center gap-2 sm:gap-4",
            label { class: "text-sm font-medium text-foreground sm:w-24 shrink-0", "{label}" }
            div { class: "flex-1 flex items-center gap-2",
                input {
                    class: "w-full px-3 py-2.5 rounded-lg border border-border bg-background text-foreground text-sm tabular-nums focus:outline-none focus:ring-2 focus:ring-primary/30 focus:border-primary transition-all duration-200",
                    type: "number",
                    value: val(),
                    min: "0",
                    step: if unit == "kcal" { "50" } else { "5" },
                    oninput: move |e| {
                        let s = e.value();
                        val.set(s.clone());
                        if let Ok(n) = s.parse::<f64>() {
                            on_change.call(n);
                        }
                    },
                }
                span { class: "text-xs text-muted-foreground w-6 shrink-0", "{unit}" }
            }
        }
    }
}
