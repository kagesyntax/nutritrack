use dioxus::prelude::*;

use crate::components::card::{Card, CardContent, CardHeader, CardTitle};
use crate::components::icons::{IconSettings, IconSun, IconMoon, IconMonitor};
use crate::state::models::ThemeMode;

#[component]
pub fn Settings() -> Element {
    let mut settings = use_context::<Signal<crate::state::models::UserSettings>>();
    let mut logs = use_context::<Signal<Vec<crate::state::models::DayLog>>>();

    rsx! {
        div { class: "max-w-2xl mx-auto p-6 space-y-6",
            h1 { class: "text-2xl font-bold text-foreground font-heading flex items-center gap-2",
                IconSettings { size: 24 }
                "Settings"
            }

            Card {
                CardHeader {
                    CardTitle { class: "font-heading", "Theme" }
                    p { class: "text-sm text-muted-foreground mt-1", "Choose your preferred color scheme" }
                }
                CardContent {
                    ThemeSelector { active: settings.read().theme_mode, on_select: move |m| { settings.write().theme_mode = m; } }
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
fn ThemeSelector(
    active: ThemeMode,
    on_select: EventHandler<ThemeMode>,
) -> Element {
    rsx! {
        div { class: "theme-selector",
            ThemeCard {
                mode: ThemeMode::Light,
                active: active == ThemeMode::Light,
                icon: rsx! { IconSun {} },
                label: "Light",
                on_select: move |_| on_select.call(ThemeMode::Light),
            }
            ThemeCard {
                mode: ThemeMode::Dark,
                active: active == ThemeMode::Dark,
                icon: rsx! { IconMoon {} },
                label: "Dark",
                on_select: move |_| on_select.call(ThemeMode::Dark),
            }
            ThemeCard {
                mode: ThemeMode::System,
                active: active == ThemeMode::System,
                icon: rsx! { IconMonitor {} },
                label: "Auto",
                on_select: move |_| on_select.call(ThemeMode::System),
            }
        }
    }
}

#[component]
fn ThemeCard(
    mode: ThemeMode,
    active: bool,
    icon: Element,
    label: &'static str,
    on_select: EventHandler<MouseEvent>,
) -> Element {
    let classes = if active {
        "theme-card theme-card-active"
    } else {
        "theme-card theme-card-inactive"
    };
    rsx! {
        button {
            class: "{classes}",
            onclick: move |e| on_select.call(e),
            div { class: "theme-card-icon", {icon} }
            span { class: "theme-card-label", "{label}" }
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
