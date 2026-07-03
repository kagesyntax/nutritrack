use dioxus::prelude::*;

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

fn day_of_week(date_str: &str) -> usize {
    let y: i32 = date_str[0..4].parse().unwrap();
    let m: i32 = date_str[5..7].parse().unwrap();
    let d: i32 = date_str[8..10].parse().unwrap();
    let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let y = if m < 3 { y - 1 } else { y };
    ((y + y / 4 - y / 100 + y / 400 + t[(m - 1) as usize] + d) % 7) as usize
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
fn CompactStat(value: String, label: &'static str) -> Element {
    rsx! {
        div { class: "stat-compact",
            div {
                span { class: "stat-compact-value", "{value}" }
                span { class: "text-xs text-muted-foreground ml-1", "{label}" }
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
            let (cals, color) = match log {
                Some(l) => {
                    let c = day_total_cal(l);
                    if target_cal > 0.0 {
                        let pct = c / target_cal;
                        if pct > 1.1 {
                            (c, "cal-dot-over")
                        } else if pct >= 0.9 {
                            (c, "cal-dot-high")
                        } else if pct > 0.5 {
                            (c, "cal-dot-medium")
                        } else if c > 0.0 {
                            (c, "cal-dot-low")
                        } else {
                            (c, "cal-dot-none")
                        }
                    } else if c > 0.0 {
                        (c, "cal-dot-medium")
                    } else {
                        (c, "cal-dot-none")
                    }
                }
                None => (0.0, "cal-dot-none"),
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

            div { class: "stat-compact-row mb-4",
                CompactStat { value: format!("{:.0}", avg_cal_this_week), label: "avg kcal" }
                CompactStat { value: format!("{}/7", days_logged_this_week), label: "logged" }
                CompactStat { value: format!("{:.0}", best_day_cal), label: "best day" }
                CompactStat { value: trend_str, label: "vs last week" }
            }

            div { class: "flex flex-col gap-2 mb-4",
                span { class: "text-sm font-medium text-foreground font-heading", "Last 30 Days" }
                div { class: "cal-grid",
                    div { class: "cal-label", "Su" }
                    div { class: "cal-label", "Mo" }
                    div { class: "cal-label", "Tu" }
                    div { class: "cal-label", "We" }
                    div { class: "cal-label", "Th" }
                    div { class: "cal-label", "Fr" }
                    div { class: "cal-label", "Sa" }
                    {
                        let first_dow = day_of_week(&last_30_days[0].date);
                        rsx! {
                            for _ in 0..first_dow {
                                div { class: "cal-dot cal-dot-empty" }
                            }
                        }
                    }
                    for day in &last_30_days {
                        div {
                            class: "cal-dot {day.color}",
                            title: "{day.date}: {day.cals:.0} kcal",
                        }
                    }
                }
            }

            if sorted.is_empty() {
                div { class: "text-center py-12",
                    p { class: "text-lg font-medium text-foreground", "No food logs yet" }
                    p { class: "text-sm text-muted-foreground mt-1", "Start tracking to see your history here." }
                }
            } else {
                div { class: "compact-list",
                    for day in sorted.iter().take(30) {
                        DayRow { day: day.clone() }
                    }
                }
            }
        }
    }
}

#[component]
fn DayRow(day: DayLog) -> Element {
    let total_cal: f64 = day_total_cal(&day);
    let entry_count: usize = day.meals.values().map(|m| m.entries.len()).sum();
    let settings = use_context::<Signal<crate::state::models::UserSettings>>();
    let pct = if total_cal > 0.0 && settings.read().targets.calories > 0.0 {
        total_cal / settings.read().targets.calories * 100.0
    } else {
        0.0
    };
    let dot_color = if pct > 100.0 { "bg-fat" } else if pct > 75.0 { "bg-primary" } else { "bg-muted" };

    rsx! {
        Link {
            class: "compact-list-item",
            to: crate::app::Route::Log {},
            div { class: "flex flex-col min-w-0",
                span { class: "text-sm font-medium text-foreground", "{day.date}" }
                span { class: "text-xxs text-muted-foreground", "{entry_count} item(s)" }
            }
            div { class: "flex-1" }
            span { class: "text-sm font-semibold tabular-nums text-foreground", "{total_cal:.0}" }
            span { class: "text-xs text-muted-foreground ml-1", "kcal" }
            div { class: "cal-dot {dot_color} ml-2" }
        }
    }
}
