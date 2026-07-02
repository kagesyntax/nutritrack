use dioxus::prelude::*;

#[allow(unused_imports)]
use crate::components::card::{Card, CardContent, CardHeader, CardTitle};
use crate::components::icons::IconFlame;
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
                div { class: "empty-state",
                    div { class: "flex flex-col items-center gap-2",
                        p { class: "text-lg font-medium text-foreground", "No data yet" }
                        p { class: "text-sm text-muted-foreground", "Log some meals to see your analytics." }
                    }
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

    // --- Today logged indicator ---
    let today_logged = recent
        .iter()
        .rev()
        .next()
        .map(|d| d.date == today && day_has_food(d))
        .unwrap_or(false);

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
    let streak_s = if streak == 1 { "" } else { "s" };

    // --- Dot color for today indicator ---
    let dot_color = if today_logged { "#22c55e" } else { "#ef4444" };

    // --- Avg vs target bar width ---
    let vs_target_width = if target > 0.0 {
        (avg_cal / target).min(1.0) * 100.0
    } else {
        0.0
    };

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
            "#22c55e"
        } else if pct >= 75.0 && pct <= 125.0 {
            "#eab308"
        } else {
            "#ef4444"
        };
        let label = day.date.get(5..).unwrap_or(&day.date).to_string();
        (day.date.clone(), label, pct, spark_color)
    }).collect();

    // --- Bar chart data (last 14 days) ---
    let recent_days: Vec<_> = recent.iter().rev().take(14).map(|day| {
        let cals = day_total_cals(day);
        let pct = if target > 0.0 { (cals / target) * 100.0 } else { 0.0 };
        let bar_color = if pct > 100.0 { "bg-fat" } else if pct > 75.0 { "bg-carbs" } else { "bg-primary" };
        (day.date.clone(), format!("{:.0}", cals), pct, bar_color)
    }).collect();

    rsx! {
        div { class: "max-w-4xl mx-auto p-6 space-y-6",
            h1 { class: "text-2xl font-bold text-foreground font-heading", "Analytics" }

            // === Overview ===
            Card {
                CardHeader {
                    CardTitle { class: "font-heading", "Overview" }
                }
                CardContent {
                    div { class: "grid grid-cols-1 md-grid-cols-3 gap-4",
                        StatCard {
                            label: "Days Tracked",
                            value: format!("{}", recent.len()),
                            unit: "days",
                            div {
                                class: "flex items-center gap-1_5 mt-1",
                                div {
                                    style: "width: 8px; height: 8px; border-radius: 50%; background: {dot_color};",
                                }
                                span { class: "text-xs text-muted-foreground",
                                    if today_logged { "Logged today" } else { "Not logged today" }
                                }
                            }
                        }
                        StatCard {
                            label: "Avg Daily Calories",
                            value: avg_str,
                            unit: "kcal",
                            div {
                                class: "flex items-center gap-1 mt-1",
                                span { class: "{weekly_trend_color} text-sm font-bold", "{weekly_arrow}" }
                                span { class: "text-xs text-muted-foreground", "{weekly_trend_text}" }
                            }
                        }
                        StatCard {
                            label: "Consistency",
                            value: format!("{:.0}%", consistency_pct),
                            unit: "30 days",
                            ConsistencyRing { pct: consistency_pct }
                        }
                        StatCard {
                            label: "Current Streak",
                            value: format!("{}", streak),
                            unit: "days",
                            div { class: "mt-1",
                                if streak > 0 {
                                    IconFlame { size: 14 }
span { class: "text-xs text-muted-foreground ml-1", "{streak} day{streak_s}" }
                                } else {
                                    span { class: "text-xs text-muted-foreground", "Start logging to build a streak" }
                                }
                            }
                        }
                        StatCard {
                            label: "Avg vs Target",
                            value: pct_str,
                            unit: "",
                            div { class: "macro-bar-track mt-2",
                                div {
                                    class: "macro-bar-fill bg-primary",
                                    style: "width: {vs_target_width:.0}%",
                                }
                            }
                        }
                    }
                }
            }

            // === Macro Distribution ===
            Card {
                CardHeader {
                    CardTitle { class: "font-heading", "Macro Distribution" }
                }
                CardContent {
                    div { class: "space-y-3",
                        div {
                            div { class: "flex justify-between text-sm mb-1",
                                span { class: "text-muted-foreground", "Protein" }
                                span { class: "font-medium tabular-nums", "{protein_pct:.0}%" }
                            }
                            div { class: "macro-bar-track",
                                div { class: "macro-bar-fill bg-protein", style: "width: {protein_pct:.0}%" }
                            }
                        }
                        div {
                            div { class: "flex justify-between text-sm mb-1",
                                span { class: "text-muted-foreground", "Carbs" }
                                span { class: "font-medium tabular-nums", "{carbs_pct:.0}%" }
                            }
                            div { class: "macro-bar-track",
                                div { class: "macro-bar-fill bg-carbs", style: "width: {carbs_pct:.0}%" }
                            }
                        }
                        div {
                            div { class: "flex justify-between text-sm mb-1",
                                span { class: "text-muted-foreground", "Fat" }
                                span { class: "font-medium tabular-nums", "{fat_pct:.0}%" }
                            }
                            div { class: "macro-bar-track",
                                div { class: "macro-bar-fill bg-fat", style: "width: {fat_pct:.0}%" }
                            }
                        }
                    }
                }
            }

            // === Last 14 Days + Weekly Trend ===
            Card {
                CardHeader {
                    CardTitle { class: "font-heading", "Last 14 Days" }
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

                    div { class: "mt-1 pt-3 border-t border-border",
                        p { class: "text-sm font-medium text-foreground mb-1", "7-Day Trend" }
                        div { class: "flex gap-2",
                            for (date, label, pct, spark_color) in &week_spark {
                                div {
                                    key: "{date}",
                                    class: "flex flex-col items-center gap-1",
                                    div {
                                        class: "rounded-lg",
                                        style: "width: 24px; height: 24px; background: {spark_color}; opacity: 0.8;",
                                        title: "{date}: {pct:.0}% of target",
                                    }
                                    span { class: "text-xs text-muted-foreground", "{label}" }
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
fn StatCard(
    label: &'static str,
    value: String,
    unit: &'static str,
    children: Element,
) -> Element {
    rsx! {
        Card { class: "hover-shadow-md hover--translate-y-0_5 transition-all duration-200",
            CardContent { class: "stat-card",
                p { class: "stat-card-label", "{label}" }
                p { class: "stat-card-value", "{value}" }
                p { class: "stat-card-unit", "{unit}" }
                {children}
            }
        }
    }
}

#[component]
fn ConsistencyRing(pct: f64) -> Element {
    let ring_color = if pct >= 80.0 {
        "#22c55e"
    } else if pct >= 50.0 {
        "#eab308"
    } else {
        "#ef4444"
    };

    rsx! {
        div { class: "flex justify-center mt-1",
            svg {
                width: "40",
                height: "40",
                view_box: "0 0 40 40",
                circle {
                    cx: "20", cy: "20", r: "17",
                    fill: "none",
                    stroke: "var(--color-surface-tertiary)",
                    stroke_width: "3",
                }
                circle {
                    cx: "20", cy: "20", r: "17",
                    fill: "none",
                    stroke: "{ring_color}",
                    stroke_width: "3",
                    stroke_linecap: "round",
                    stroke_dasharray: "106.8",
                    stroke_dashoffset: "{106.8 - (106.8 * pct / 100.0)}",
                    transform: "rotate(-90 20 20)",
                }
            }
        }
    }
}
