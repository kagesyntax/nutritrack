use dioxus::prelude::*;

use crate::components::card::{Card, CardContent, CardHeader, CardTitle};
use crate::state::food_db::find_food;

#[component]
pub fn Analytics() -> Element {
    let logs = use_context::<Signal<Vec<crate::state::models::DayLog>>>();
    let settings = use_context::<Signal<crate::state::models::UserSettings>>();

    let recent: Vec<_> = {
        let guard = logs.read();
        let today = crate::utils::todays_date();
        guard
            .iter()
            .filter(|l| l.date.as_str() <= today.as_str())
            .cloned()
            .collect()
    };

    let avg_cal = if !recent.is_empty() {
        let total: f64 = recent
            .iter()
            .map(|day| {
                day.meals
                    .values()
                    .flat_map(|m| &m.entries)
                    .filter_map(|e| find_food(&e.food_id))
                    .map(|f| f.calories)
                    .sum::<f64>()
            })
            .sum();
        total / recent.len() as f64
    } else {
        0.0
    };

    let target = settings.read().targets.calories;
    let avg_str = format!("{:.0}", avg_cal);
    let pct_str = if target > 0.0 {
        format!("{:.0}%", (avg_cal / target) * 100.0)
    } else {
        "N/A".to_string()
    };

    let recent_days: Vec<_> = recent.iter().rev().take(14).map(|day| {
        let cals: f64 = day
            .meals
            .values()
            .flat_map(|m| &m.entries)
            .filter_map(|e| find_food(&e.food_id))
            .map(|f| f.calories)
            .sum();
        let pct = if target > 0.0 { (cals / target) * 100.0 } else { 0.0 };
        let bar_color = if pct > 100.0 { "bg-fat" } else if pct > 75.0 { "bg-carbs" } else { "bg-primary" };
        let cals_str = format!("{:.0}", cals);
        (day.date.clone(), cals_str, pct, bar_color)
    }).collect();

    rsx! {
        div { class: "max-w-4xl mx-auto p-6 space-y-6",
            h1 { class: "text-2xl font-bold text-foreground font-heading", "Analytics" }

            if recent.is_empty() {
                div { class: "empty-state",
                    div { class: "flex flex-col items-center gap-2",
                        p { class: "text-lg font-medium text-foreground", "No data yet" }
                        p { class: "text-sm text-muted-foreground", "Log some meals to see your analytics." }
                    }
                }
            } else {
                div { class: "grid grid-cols-1 md-grid-cols-3 gap-4",
                    StatCard { label: "Days Tracked", value: format!("{}", recent.len()), unit: "days" }
                    StatCard { label: "Avg Daily Calories", value: avg_str, unit: "kcal" }
                    StatCard { label: "Avg vs Target", value: pct_str, unit: "" }
                }

                Card {
                    CardHeader {
                        CardTitle { class: "font-heading", "Recent Days" }
                    }
                    CardContent {
                        div { class: "space-y-3",
                            for (date, cals_str, pct, bar_color) in &recent_days {
                                div { class: "day-bar",
                                    span { class: "day-bar-date", "{date}" }
                                    div { class: "day-bar-track",
                                        div {
                                            class: "day-bar-fill {bar_color}",
                                            style: "width: {pct.min(100.0):.0}%",
                                        }
                                    }
                                    span { class: "day-bar-cals", "{cals_str}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StatCard(label: &'static str, value: String, unit: &'static str) -> Element {
    rsx! {
        Card { class: "hover-shadow-md hover--translate-y-0_5 transition-all duration-200",
            CardContent { class: "stat-card",
                p { class: "stat-card-label", "{label}" }
                p { class: "stat-card-value", "{value}" }
                p { class: "stat-card-unit", "{unit}" }
            }
        }
    }
}
