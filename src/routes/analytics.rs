use dioxus::prelude::*;

use crate::components::card::{Card, CardContent};
use crate::components::icons::IconActivity;
use crate::components::icons::IconFlame;
use crate::components::icons::IconTarget;
use crate::state::food_db::find_food;

fn day_total_cals(day: &crate::state::models::DayLog) -> f64 {
    day.meals
        .values()
        .flat_map(|m| &m.entries)
        .filter_map(|e| find_food(&e.food_id))
        .map(|f| f.calories)
        .sum()
}

fn day_has_food(day: &crate::state::models::DayLog) -> bool {
    day.meals.values().any(|m| !m.entries.is_empty())
}

fn avg_cals_for(days: &[&crate::state::models::DayLog]) -> f64 {
    if days.is_empty() {
        return 0.0;
    }
    days.iter().map(|d| day_total_cals(d)).sum::<f64>() / days.len() as f64
}

#[component]
fn CompactStat(icon: Element, value: String, label: &'static str) -> Element {
    rsx! {
        div { class: "stat-compact",
            div { class: "stat-compact-glyph", {icon} }
            div {
                span { class: "stat-compact-value", "{value}" }
                span { class: "text-xs text-muted-foreground ml-1", "{label}" }
            }
        }
    }
}

#[component]
pub fn Analytics() -> Element {
    let logs = use_context::<Signal<Vec<crate::state::models::DayLog>>>();
    let settings = use_context::<Signal<crate::state::models::UserSettings>>();

    let today = crate::utils::todays_date();

    let recent: Vec<_> = {
        let guard = logs.read();
        guard
            .iter()
            .filter(|l| l.date.as_str() <= today.as_str())
            .cloned()
            .collect()
    };

    if recent.is_empty() {
        return rsx! {
            div { class: "max-w-4xl mx-auto p-6 space-y-6",
                h1 { class: "text-2xl font-bold text-foreground font-heading", "Analytics" }
                div { class: "flex flex-col items-center gap-4",
                    p { class: "text-lg font-medium text-foreground", "No data yet" }
                    p { class: "text-sm text-muted-foreground", "Log some meals to see your analytics." }
                }
            }
        };
    }

    let target = settings.read().targets.calories;

    // --- Avg daily calories ---
    let avg_cal = {
        let total: f64 = recent.iter().map(day_total_cals).sum();
        total / recent.len() as f64
    };
    let avg_str = format!("{:.0}", avg_cal);

    let pct_str = if target > 0.0 {
        format!("{:.0}%", (avg_cal / target) * 100.0)
    } else {
        "N/A".to_string()
    };

    // --- Consistency (last 30 days) ---
    let last_30: Vec<_> = recent.iter().rev().take(30).collect();
    let logged_30 = last_30.iter().filter(|d| day_has_food(d)).count();
    let consistency_pct = if !last_30.is_empty() {
        (logged_30 as f64 / last_30.len() as f64) * 100.0
    } else {
        0.0
    };

    // --- Current streak ---
    let streak = {
        let mut s = 0u32;
        for day in recent.iter().rev() {
            if day_has_food(day) {
                s += 1;
            } else {
                break;
            }
        }
        s
    };
    let streak_label = if streak == 1 { "day" } else { "days" };

    // --- Weekly comparison ---
    let days_this_week: Vec<_> = recent.iter().rev().take(7).collect();
    let days_last_week: Vec<_> = recent.iter().rev().skip(7).take(7).collect();

    let this_week_avg = avg_cals_for(&days_this_week);
    let last_week_avg = avg_cals_for(&days_last_week);

    let (weekly_arrow, weekly_trend_text, weekly_trend_color) =
        if last_week_avg > 0.0 && this_week_avg > 0.0 {
            let diff = ((this_week_avg - last_week_avg) / last_week_avg) * 100.0;
            let arrow = if diff > 0.0 { "↑" } else { "↓" };
            let color = if diff > 0.0 { "text-destructive" } else { "text-primary" };
            (arrow, format!("{:.1}% vs last week", diff.abs()), color)
        } else {
            ("→", "Insufficient data".to_string(), "text-muted-foreground")
        };

    // --- Macro averages ---
    let (avg_protein, avg_carbs, avg_fat) = {
        let mut tp = 0.0;
        let mut tc = 0.0;
        let mut tf = 0.0;
        for day in &recent {
            for meal in day.meals.values() {
                for entry in &meal.entries {
                    if let Some(food) = find_food(&entry.food_id) {
                        tp += food.protein_g;
                        tc += food.carbs_g;
                        tf += food.fat_g;
                    }
                }
            }
        }
        let n = recent.len() as f64;
        (tp / n, tc / n, tf / n)
    };

    let total_macro_cals = avg_protein * 4.0 + avg_carbs * 4.0 + avg_fat * 9.0;
    let protein_pct = if total_macro_cals > 0.0 {
        (avg_protein * 4.0 / total_macro_cals) * 100.0
    } else {
        0.0
    };
    let carbs_pct = if total_macro_cals > 0.0 {
        (avg_carbs * 4.0 / total_macro_cals) * 100.0
    } else {
        0.0
    };
    let fat_pct = if total_macro_cals > 0.0 {
        (avg_fat * 9.0 / total_macro_cals) * 100.0
    } else {
        0.0
    };

    // --- Weekly sparkline (last 7 days) ---
    let week_spark: Vec<_> = recent.iter().rev().take(7).map(|day| {
        let cals = day_total_cals(day);
        let pct = if target > 0.0 { (cals / target) * 100.0 } else { 0.0 };
        let spark_color = if pct >= 90.0 && pct <= 110.0 {
            "var(--color-success)"
        } else if pct >= 75.0 && pct <= 125.0 {
            "var(--color-warning)"
        } else {
            "var(--color-destructive)"
        };
        let label = day.date.get(5..).unwrap_or(&day.date).to_string();
        (day.date.clone(), label, pct, spark_color)
    }).collect();

    // --- Last 30 days for the day strip ---
    let last_30_days: Vec<_> = recent.iter().rev().take(30).map(|day| {
        let cals = day_total_cals(day);
        let has_food = day_has_food(day);
        let color = if !has_food {
            "var(--color-surface-tertiary)".to_string()
        } else {
            let pct = if target > 0.0 { (cals / target) * 100.0 } else { 0.0 };
            if pct >= 90.0 && pct <= 110.0 {
                "var(--color-success)".to_string()
            } else if pct >= 75.0 {
                "var(--color-warning)".to_string()
            } else {
                "var(--color-destructive)".to_string()
            }
        };
        (day.date.clone(), cals, color)
    }).collect();

    rsx! {
        div { class: "max-w-4xl mx-auto p-6 space-y-6",
            h1 { class: "text-2xl font-bold text-foreground font-heading", "Analytics" }

            // === Compact Stat Row ===
            Card {
                CardContent { class: "pb-4",
                    div { class: "stat-compact-row",
                        CompactStat { icon: rsx! { IconActivity { size: 14 } }, value: format!("{}", recent.len()), label: "days" }
                        CompactStat { icon: rsx! { IconTarget { size: 14 } }, value: avg_str, label: "avg kcal" }
                        CompactStat { icon: rsx! { IconTarget { size: 14 } }, value: format!("{:.0}%", consistency_pct), label: "consistent" }
                        CompactStat { icon: rsx! { IconFlame { size: 14 } }, value: format!("{}", streak), label: streak_label }
                        CompactStat { icon: rsx! { IconTarget { size: 14 } }, value: pct_str, label: "of goal" }
                    }
                }
            }

            // === Macro Distribution ===
            Card {
                CardContent {
                    div { class: "flex flex-col gap-2",
                        span { class: "text-sm font-medium text-foreground font-heading", "Macro Distribution" }
                        div { class: "macro-bar-segmented",
                            if protein_pct > 0.0 {
                                div { class: "macro-bar-segment bg-protein", style: "width: {protein_pct:.0}%" }
                            }
                            if carbs_pct > 0.0 {
                                div { class: "macro-bar-segment bg-carbs", style: "width: {carbs_pct:.0}%" }
                            }
                            if fat_pct > 0.0 {
                                div { class: "macro-bar-segment bg-fat", style: "width: {fat_pct:.0}%" }
                            }
                        }
                        div { class: "flex justify-between text-xs",
                            span { class: "text-protein font-medium", "P {protein_pct:.0}%" }
                            span { class: "text-carbs font-medium", "C {carbs_pct:.0}%" }
                            span { class: "text-fat font-medium", "F {fat_pct:.0}%" }
                        }
                    }
                }
            }

            // === 7-Day Trend Sparkline ===
            Card {
                CardContent { class: "space-y-3",
                    div { class: "flex flex-col gap-2",
                        span { class: "text-sm font-medium text-foreground font-heading", "7-Day Trend" }
                        div { class: "sparkline",
                            for (date, _label, pct, spark_color) in &week_spark {
                                div {
                                    key: "{date}",
                                    class: "sparkline-bar",
                                    style: "height: {pct.min(100.0):.0}%; background: {spark_color}",
                                    title: "{date}: {pct:.0}% of target",
                                }
                            }
                        }
                        div { class: "flex justify-between text-xxs text-muted-foreground",
                            for (_, label, _, _) in &week_spark {
                                span { "{label}" }
                            }
                        }
                    }

                    div { class: "flex items-center gap-2",
                        span { class: "{weekly_trend_color} text-sm font-bold", "{weekly_arrow}" }
                        span { class: "text-xs text-muted-foreground", "{weekly_trend_text}" }
                    }
                }
            }

            // === Last 30 Days ===
            Card {
                CardContent {
                    div { class: "flex flex-col gap-2",
                        span { class: "text-sm font-medium text-foreground font-heading", "Last 30 Days" }
                        div { class: "day-strip",
                            for day in &last_30_days {
                                div {
                                    class: "day-strip-item",
                                    style: "background: {day.2}",
                                    title: "{day.0}: {day.1:.0} kcal",
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
