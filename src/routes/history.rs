use dioxus::prelude::*;

use crate::components::card::{Card, CardContent, CardHeader, CardTitle};
use crate::components::icons::IconCalendar;
use crate::state::food_db::find_food;
use crate::state::models::DayLog;

fn day_total_cal(day: &DayLog) -> f64 {
    day.meals
        .values()
        .flat_map(|m| &m.entries)
        .filter_map(|e| find_food(&e.food_id))
        .map(|f| f.calories)
        .sum()
}

fn date_minus_days(date: &str, days: i32) -> String {
    let mut year: i32 = date[0..4].parse().unwrap();
    let mut month: i32 = date[5..7].parse().unwrap();
    let mut day: i32 = date[8..10].parse().unwrap();

    day -= days;
    while day < 1 {
        month -= 1;
        if month < 1 {
            month = 12;
            year -= 1;
        }
        day += match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => 28,
            _ => 30,
        };
    }

    format!("{:04}-{:02}-{:02}", year, month, day)
}

#[component]
fn WeekStat(label: &'static str, value: String, unit: &'static str) -> Element {
    rsx! {
        div { class: "text-center p-4 rounded-lg bg-surface-secondary",
            p { class: "text-xs text-muted-foreground uppercase tracking-wider", "{label}" }
            p { class: "text-xl font-bold tabular-nums text-foreground mt-1", "{value}" }
            if !unit.is_empty() {
                p { class: "text-xs text-muted-foreground mt-0_5", "{unit}" }
            }
        }
    }
}

#[component]
pub fn History() -> Element {
    let logs = use_context::<Signal<Vec<DayLog>>>();
    let settings = use_context::<Signal<crate::state::models::UserSettings>>();

    let mut sorted = logs.read().clone();
    sorted.sort_by(|a, b| b.date.cmp(&a.date));

    let today = crate::utils::todays_date();

    let this_week_dates: Vec<String> = (0..7).map(|i| date_minus_days(&today, i)).collect();
    let last_week_dates: Vec<String> = (7..14).map(|i| date_minus_days(&today, i)).collect();

    let this_week_logs: Vec<&DayLog> = sorted.iter().filter(|d| this_week_dates.contains(&d.date)).collect();
    let last_week_logs: Vec<&DayLog> = sorted.iter().filter(|d| last_week_dates.contains(&d.date)).collect();

    let days_logged_this_week = this_week_logs.len();
    let avg_cal_this_week = if days_logged_this_week > 0 {
        this_week_logs.iter().map(|d| day_total_cal(d)).sum::<f64>() / days_logged_this_week as f64
    } else {
        0.0
    };
    let best_day_cal = this_week_logs
        .iter()
        .map(|d| day_total_cal(d))
        .fold(0.0_f64, |a, b| a.max(b));

    let days_logged_last_week = last_week_logs.len();
    let avg_cal_last_week = if days_logged_last_week > 0 {
        last_week_logs.iter().map(|d| day_total_cal(d)).sum::<f64>() / days_logged_last_week as f64
    } else {
        0.0
    };

    let trend_str = if avg_cal_last_week > 0.0 {
        let diff = (avg_cal_this_week - avg_cal_last_week) / avg_cal_last_week * 100.0;
        if diff >= 0.0 {
            format!("↑ {:.0}%", diff)
        } else {
            format!("↓ {:.0}%", diff.abs())
        }
    } else {
        "—".to_string()
    };

    let target_cal = settings.read().targets.calories;

    struct HeatmapDay {
        date: String,
        cals: f64,
        color: &'static str,
    }

    let last_30_days: Vec<HeatmapDay> = (0..30)
        .rev()
        .map(|i| {
            let d = date_minus_days(&today, i);
            let log = sorted.iter().find(|l| l.date == d);
            let is_today = d == today;
            let (cals, color) = match log {
                Some(l) => {
                    let c = day_total_cal(l);
                    if is_today {
                        (c, "bg-surface-tertiary")
                    } else if c >= target_cal * 0.9 && c <= target_cal * 1.1 {
                        (c, "bg-primary")
                    } else {
                        (c, "bg-carbs")
                    }
                }
                None => {
                    if is_today {
                        (0.0, "bg-surface-tertiary")
                    } else {
                        (0.0, "bg-muted")
                    }
                }
            };
            HeatmapDay {
                date: d,
                cals,
                color,
            }
        })
        .collect();

    rsx! {
        div { class: "max-w-4xl mx-auto p-6 space-y-6",
            h1 { class: "text-2xl font-bold text-foreground font-heading", "History" }

            Card {
                CardHeader {
                    CardTitle { class: "font-heading", "This Week" }
                }
                CardContent {
                    div { class: "grid grid-cols-2 md-grid-cols-4 gap-4",
                        WeekStat { label: "Avg Calories", value: format!("{:.0}", avg_cal_this_week), unit: "kcal" }
                        WeekStat { label: "Days Logged", value: format!("{}/7", days_logged_this_week), unit: "" }
                        WeekStat { label: "Best Day", value: format!("{:.0}", best_day_cal), unit: "kcal" }
                        WeekStat { label: "vs Last Week", value: trend_str, unit: "" }
                    }
                }
            }

            Card {
                CardHeader {
                    CardTitle { class: "font-heading", "Last 30 Days" }
                }
                CardContent {
                    div { class: "flex flex-wrap gap-1",
                        for day in last_30_days {
                            div {
                                class: "w-3 h-3 rounded-sm {day.color}",
                                title: "{day.date}: {day.cals:.0} kcal",
                            }
                        }
                    }
                }
            }

            if sorted.is_empty() {
                Card { class: "empty-state",
                    CardContent { class: "text-center py-12",
                        p { class: "text-lg font-medium text-card-foreground", "No food logs yet" }
                        p { class: "text-sm text-muted-foreground mt-1", "Start tracking to see your history here." }
                    }
                }
            } else {
                div { class: "space-y-3",
                    for day in sorted.iter().take(30) {
                        DayCard { day: day.clone() }
                    }
                }
            }
        }
    }
}

#[component]
fn DayCard(day: DayLog) -> Element {
    let total_cal: f64 = day_total_cal(&day);
    let entry_count: usize = day.meals.values().map(|m| m.entries.len()).sum();
    let settings = use_context::<Signal<crate::state::models::UserSettings>>();
    let pct = if total_cal > 0.0 && settings.read().targets.calories > 0.0 {
        total_cal / settings.read().targets.calories * 100.0
    } else {
        0.0
    };
    let bar_color = if pct > 100.0 { "bg-fat" } else if pct > 75.0 { "bg-primary" } else { "bg-muted" };

    rsx! {
        Link {
            class: "block group",
            to: crate::app::Route::Log {},
            Card { class: "group-hover-shadow-md group-hover--translate-y-0_5 transition-all duration-200 cursor-pointer",
                CardHeader {
                    CardTitle { class: "font-heading text-base", "{day.date}" }
                }
                CardContent { class: "flex items-center justify-between",
                    div { class: "flex items-center gap-3",
                        div { class: "w-10 h-10 rounded-xl bg-surface-secondary flex items-center justify-center text-muted-foreground",
                            IconCalendar { size: 18 }
                        }
                        div {
                            p { class: "text-xs text-muted-foreground", "{entry_count} items logged" }
                        }
                    }
                    div { class: "text-right",
                        p { class: "font-semibold tabular-nums text-card-foreground text-lg", "{total_cal:.0}" }
                        p { class: "text-xs text-muted-foreground", "kcal" }
                    }
                }
                if total_cal > 0.0 {
                    div { class: "px-4 pb-4",
                        div { class: "macro-bar-track",
                            div {
                                class: "macro-bar-fill {bar_color}",
                                style: "width: {pct.min(100.0):.0}%",
                            }
                        }
                    }
                }
            }
        }
    }
}
