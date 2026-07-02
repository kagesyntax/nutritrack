use dioxus::prelude::*;

use crate::components::card::{Card, CardContent, CardHeader, CardTitle};
use crate::components::icons::IconSettings;
use crate::state::models::ThemeMode;

#[component]
pub fn Settings() -> Element {
    let mut settings = use_context::<Signal<crate::state::models::UserSettings>>();
    let mut logs = use_context::<Signal<Vec<crate::state::models::DayLog>>>();

    rsx! {
        div { class: "max-w-2xl mx-auto p-6 space-y-6",
            h1 { class: "text-2xl font-bold text-foreground font-heading flex items-center gap-2",
                IconSettings { size: 28 }
                "Settings"
            }

            Card {
                CardHeader {
                    CardTitle { class: "font-heading", "Theme" }
                    p { class: "text-sm text-muted-foreground mt-1", "Choose your preferred color scheme" }
                }
                CardContent {
                    div { class: "flex gap-3",
                        ThemeOption {
                            label: "Light",
                            active: settings.read().theme_mode == ThemeMode::Light,
                            onclick: move |_| {
                                settings.write().theme_mode = ThemeMode::Light;
                            },
                        }
                        ThemeOption {
                            label: "Dark",
                            active: settings.read().theme_mode == ThemeMode::Dark,
                            onclick: move |_| {
                                settings.write().theme_mode = ThemeMode::Dark;
                            },
                        }
                        ThemeOption {
                            label: "System",
                            active: settings.read().theme_mode == ThemeMode::System,
                            onclick: move |_| {
                                settings.write().theme_mode = ThemeMode::System;
                            },
                        }
                    }
                }
            }

            Card {
                CardHeader {
                    CardTitle { class: "font-heading", "Daily Nutrition Targets" }
                    p { class: "text-sm text-muted-foreground mt-1", "Set your daily goals for calories and macronutrients" }
                }
                CardContent {
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                        TargetInput {
                            label: "Daily Calories",
                            value: settings.read().targets.calories,
                            unit: "kcal",
                            on_change: move |v| {
                                settings.write().targets.calories = v;
                            },
                        }
                        TargetInput {
                            label: "Daily Protein",
                            value: settings.read().targets.protein_g,
                            unit: "g",
                            on_change: move |v| {
                                settings.write().targets.protein_g = v;
                            },
                        }
                        TargetInput {
                            label: "Daily Carbs",
                            value: settings.read().targets.carbs_g,
                            unit: "g",
                            on_change: move |v| {
                                settings.write().targets.carbs_g = v;
                            },
                        }
                        TargetInput {
                            label: "Daily Fat",
                            value: settings.read().targets.fat_g,
                            unit: "g",
                            on_change: move |v| {
                                settings.write().targets.fat_g = v;
                            },
                        }
                        TargetInput {
                            label: "Daily Fiber",
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
                    CardTitle { class: "font-heading", "Data Management" }
                    p { class: "text-sm text-muted-foreground mt-1", "Manage your stored nutrition data" }
                }
                CardContent {
                    div { class: "space-y-4",
                        div { class: "rounded-lg border border-border bg-red-50 p-4",
                            p { class: "text-sm font-medium", style: "color: var(--color-destructive);", "Warning" }
                            p { class: "text-sm text-muted-foreground mt-1", "This will permanently delete all your logged meals and nutrition data. This action cannot be undone." }
                        }
                        button {
                            class: "btn btn-outline btn-md hover-bg-red-50 hover-text-red-500",
                            style: "color: var(--color-destructive); border-color: var(--color-destructive);",
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
fn ThemeOption(
    label: &'static str,
    active: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let active_class = if active {
        "bg-primary text-white border-primary shadow-md"
    } else {
        "bg-card text-foreground border-border hover-bg-muted"
    };
    rsx! {
        button {
            class: "flex-1 px-4 py-2_5 rounded-lg border text-sm font-medium transition-all duration-200 {active_class}",
            onclick: move |e| {
                onclick.call(e);
            },
            "{label}"
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
        div { class: "bg-card border border-border rounded-lg p-4 space-y-2 transition-all duration-200 hover-shadow-md",
            label { class: "text-sm font-medium text-foreground", "{label}" }
            div { class: "flex items-center gap-2",
                input {
                    class: "flex-1 px-3 py-2 rounded-lg border border-border bg-background text-foreground text-sm tabular-nums focus-outline-none focus-ring-2 transition-all duration-200",
                    "type": "number",
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
                span { class: "text-xs font-medium text-muted-foreground bg-muted px-2 py-1 rounded", "{unit}" }
            }
        }
    }
}
