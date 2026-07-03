use dioxus::prelude::*;

use crate::components::card::{Card, CardContent, CardSize};
use crate::components::icons::{IconActivity, IconCoffee, IconFlame, IconMoon, IconPlus, IconSun, IconSunrise};
use crate::components::progress::CalorieRing;
use crate::state::food_db::find_food;
use crate::state::models::{Meal, MealType};
use crate::utils::{meal_total_calories, todays_date};

#[component]
pub fn Dashboard() -> Element {
    let logs = use_context::<Signal<Vec<crate::state::models::DayLog>>>();
    let settings = use_context::<Signal<crate::state::models::UserSettings>>();
    let today = todays_date();

    let day_log = logs.read().iter().find(|l| l.date == today).cloned();
    let meals: Vec<Meal> = day_log
        .map(|dl| dl.meals.into_values().collect())
        .unwrap_or_default();

    let total_cal: f64 = meals.iter().map(|m| meal_total_calories(m)).sum();
    let target_cal = settings.read().targets.calories;
    let targets = settings.read().targets.clone();

    let mut total_p = 0.0;
    let mut total_c = 0.0;
    let mut total_f = 0.0;
    let mut total_fiber = 0.0;
    for meal in &meals {
        for entry in &meal.entries {
            if let Some(food) = find_food(&entry.food_id) {
                total_p += food.protein_g * entry.servings;
                total_c += food.carbs_g * entry.servings;
                total_f += food.fat_g * entry.servings;
                total_fiber += food.fiber_g * entry.servings;
            }
        }
    }

    let meal_summaries: Vec<_> = MealType::all()
        .iter()
        .map(|meal_type| {
            let meal = meals.iter().find(|m| m.meal_type == *meal_type);
            let cals = meal.map(|m| meal_total_calories(m)).unwrap_or(0.0);
            let count = meal.map(|m| m.entries.len()).unwrap_or(0);
            (meal_type.clone(), cals, count)
        })
        .collect();

    let mut food_dates: Vec<String> = logs
        .read()
        .iter()
        .filter(|dl| dl.meals.values().any(|m| !m.entries.is_empty()))
        .map(|dl| dl.date.clone())
        .collect();
    food_dates.sort();
    food_dates.dedup();

    let mut streak = 0i32;
    let cursor = js_sys::Date::new_0();
    loop {
        let date_str = format!(
            "{:04}-{:02}-{:02}",
            cursor.get_full_year(),
            cursor.get_month() + 1,
            cursor.get_date(),
        );
        if food_dates.contains(&date_str) {
            streak += 1;
            cursor.set_date(cursor.get_date() - 1);
        } else {
            break;
        }
    }

    let today_dt = js_sys::Date::new_0();
    let current_year = today_dt.get_full_year();
    let current_month_0idx = today_dt.get_month();

    let days_this_month_with_food = food_dates
        .iter()
        .filter(|d| {
            let parts: Vec<&str> = d.split('-').collect();
            parts.len() == 3
                && parts[0].parse::<u32>().unwrap_or(0) == current_year
                && parts[1].parse::<u32>().unwrap_or(0) == current_month_0idx + 1
        })
        .count();

    let (next_year, next_month_0idx) = if current_month_0idx == 11 {
        (current_year + 1, 0u32)
    } else {
        (current_year, current_month_0idx + 1)
    };
    let temp = js_sys::Date::new_with_year_month_day(next_year, next_month_0idx as i32, 0);
    let total_days_in_month = temp.get_date();

    let consistency = if total_days_in_month > 0 {
        (days_this_month_with_food as f64 / total_days_in_month as f64 * 100.0) as i32
    } else {
        0
    };

    let remaining = target_cal - total_cal;

    let week_spark: Vec<(String, f64, f64, String)> = {
        let mut days = Vec::new();
        let cursor = js_sys::Date::new_0();
        for _ in 0..7 {
            let date_str = format!(
                "{:04}-{:02}-{:02}",
                cursor.get_full_year(),
                cursor.get_month() + 1,
                cursor.get_date(),
            );
            let cals = logs
                .read()
                .iter()
                .find(|l| l.date == date_str)
                .map(|dl| {
                    dl.meals
                        .values()
                        .flat_map(|m| &m.entries)
                        .filter_map(|e| {
                            let servings = e.servings;
                            find_food(&e.food_id).map(|f| f.calories * servings)
                        })
                        .sum::<f64>()
                })
                .unwrap_or(0.0);
            let pct = if target_cal > 0.0 {
                (cals / target_cal) * 100.0
            } else {
                0.0
            };
            let color = if pct >= 90.0 && pct <= 110.0 {
                "var(--color-success)"
            } else if pct >= 75.0 {
                "var(--color-warning)"
            } else {
                "var(--color-destructive)"
            };
            days.push((date_str, cals, pct, color.to_string()));
            cursor.set_date(cursor.get_date() - 1);
        }
        days
    };

    rsx! {
        div { class: "max-w-4xl mx-auto p-6 space-y-6",
            div { class: "flex items-center justify-between",
                div {
                    h1 { class: "text-2xl font-bold text-foreground font-heading", "Dashboard" }
                    p { class: "text-sm text-muted-foreground mt-1", "{today}" }
                }
                Link {
                    to: crate::app::Route::Log {},
                    class: "inline-flex items-center gap-1_5 px-4 py-2 rounded-lg bg-primary text-white text-sm font-medium hover-bg-primary-dark transition-colors",
                    IconSunrise { size: 16 }
                    "Log Meals"
                }
            }

            div { class: "flex items-center gap-2 flex-wrap",
                div { class: "inline-flex items-center gap-1_5 px-3 py-1_5 rounded-full bg-primary text-white text-xs font-medium",
                    IconFlame { size: 14 }
                    span { "{streak} day streak" }
                }
                div { class: "inline-flex items-center gap-1_5 px-3 py-1_5 rounded-full bg-surface-secondary text-foreground text-xs font-medium",
                    IconActivity { size: 14 }
                    span { "{consistency}% this month" }
                }
            }

            div { class: "bento-grid",
                div { class: "bento-hero",
                    Card { size: CardSize::Sm,
                        CardContent {
                            div { class: "calorie-hero",
                                div { class: "calorie-hero-ring",
                                    CalorieRing { current: total_cal, target: target_cal }
                                }
                                div { class: "calorie-hero-info",
                                    div { class: "flex items-baseline gap-2",
                                        span { class: "calorie-hero-value", "{total_cal:.0}" }
                                        span { class: "calorie-hero-target", "of {target_cal:.0} kcal" }
                                    }
                                    div { class: "calorie-hero-sparkline",
                                        div { class: "sparkline",
                                            for (_, _, pct, spark_color) in &week_spark {
                                                div {
                                                    class: "sparkline-bar",
                                                    style: "height: {pct.min(100.0):.0}%; background: {spark_color}",
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                div { class: "bento-grid",
                    MacroTile { label: "Protein", current: total_p, target: targets.protein_g, unit: "g", color: "bg-protein", text_color: "text-protein" }
                    MacroTile { label: "Carbs", current: total_c, target: targets.carbs_g, unit: "g", color: "bg-carbs", text_color: "text-carbs" }
                    MacroTile { label: "Fat", current: total_f, target: targets.fat_g, unit: "g", color: "bg-fat", text_color: "text-fat" }
                    MacroTile { label: "Fiber", current: total_fiber, target: targets.fiber_g, unit: "g", color: "bg-fiber", text_color: "text-fiber" }
                }
            }

            h2 { class: "text-lg font-semibold text-foreground font-heading", "Today's Meals" }
            div { class: "timeline",
                for (meal_type, cals, count) in &meal_summaries {
                    if *count == 0 {
                        Link {
                            to: crate::app::Route::Log {},
                            class: "timeline-add",
                            IconPlus { size: 14 }
                            span { "Add {meal_type.label()}" }
                        }
                    } else {
                        Link {
                            to: crate::app::Route::Log {},
                            class: "timeline-item",
                            div { class: "timeline-icon",
                                match meal_type {
                                    MealType::Breakfast => rsx! { IconSunrise { size: 16 } },
                                    MealType::Lunch => rsx! { IconSun { size: 16 } },
                                    MealType::Dinner => rsx! { IconMoon { size: 16 } },
                                    MealType::Snack => rsx! { IconCoffee { size: 16 } },
                                }
                            }
                            span { class: "timeline-name", "{meal_type.label()}" }
                            span { class: "timeline-cals", "{cals:.0} kcal" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MacroTile(label: &'static str, current: f64, target: f64, unit: &'static str, color: &'static str, text_color: &'static str) -> Element {
    let pct = if target > 0.0 { (current / target * 100.0).min(100.0) } else { 0.0 };
    rsx! {
        Card { size: CardSize::Sm,
            CardContent { class: "text-center",
                p { class: "text-xs text-muted-foreground uppercase tracking-wider", "{label}" }
                p { class: "text-xl font-bold tabular-nums {text_color} mt-1", "{current:.0}" }
                p { class: "text-xs text-muted-foreground", "/ {target:.0} {unit}" }
                div { class: "macro-bar-track mt-2",
                    div { class: "macro-bar-fill {color}", style: "width: {pct:.0}%" }
                }
            }
        }
    }
}
