use dioxus::prelude::*;

use crate::components::card::{Card, CardContent, CardHeader, CardTitle};
use crate::components::icons::{IconBarChart2, IconChevronRight, IconCoffee, IconFlame, IconMoon, IconSun, IconSunrise};
use crate::components::progress::{CalorieRing, MacroBar};
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

    let total_cal = meals.iter().map(|m| meal_total_calories(m)).sum();
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

            div { class: "flex flex-wrap gap-3",
                div { class: "inline-flex items-center gap-1_5 px-3 py-1_5 rounded-full bg-primary text-white text-xs font-medium",
                    IconFlame { size: 14 }
                    span { "{streak} day streak" }
                }
                div { class: "inline-flex items-center gap-1_5 px-3 py-1_5 rounded-full bg-surface-secondary text-foreground text-xs font-medium",
                    IconBarChart2 { size: 14 }
                    span { "{consistency}% this month" }
                }
            }

            div { class: "grid grid-cols-1 md-grid-cols-3 gap-6",
                Card { class: "md-col-span-1",
                    CardContent { class: "py-6",
                        div { class: "flex flex-col items-center",
                            CalorieRing { current: total_cal, target: target_cal }
                            if total_cal < target_cal {
                                p { class: "text-sm text-muted-foreground mt-2", "{remaining:.0} kcal remaining" }
                                div { class: "macro-bar-track mt-1 w-4/5",
                                    div {
                                        class: "macro-bar-fill bg-primary",
                                        style: "width: {(total_cal / target_cal * 100.0).min(100.0):.0}%",
                                    }
                                }
                            }
                        }
                    }
                }

                Card { class: "md-col-span-2",
                    CardHeader {
                        CardTitle { class: "font-heading", "Today's Macros" }
                    }
                    CardContent {
                        div { class: "space-y-3",
                            MacroBar { label: "Protein", current: total_p, target: targets.protein_g, color: "bg-protein", unit: "g" }
                            MacroBar { label: "Carbs", current: total_c, target: targets.carbs_g, color: "bg-carbs", unit: "g" }
                            MacroBar { label: "Fat", current: total_f, target: targets.fat_g, color: "bg-fat", unit: "g" }
                            MacroBar { label: "Fiber", current: total_fiber, target: targets.fiber_g, color: "bg-fiber", unit: "g" }
                        }
                    }
                }
            }

            Card {
                CardHeader {
                    CardTitle { class: "font-heading", "Daily Goals" }
                }
                CardContent {
                    div { class: "grid grid-cols-2 md-grid-cols-4 gap-3",
                        GoalCircle { label: "Protein", current: total_p, target: targets.protein_g, unit: "g", color: "bg-protein" }
                        GoalCircle { label: "Carbs", current: total_c, target: targets.carbs_g, unit: "g", color: "bg-carbs" }
                        GoalCircle { label: "Fat", current: total_f, target: targets.fat_g, unit: "g", color: "bg-fat" }
                        GoalCircle { label: "Fiber", current: total_fiber, target: targets.fiber_g, unit: "g", color: "bg-fiber" }
                    }
                }
            }

            h2 { class: "text-lg font-semibold text-foreground font-heading", "Today's Meals" }
            div { class: "space-y-3",
                for (meal_type, cals, count) in &meal_summaries {
                    Link {
                        to: crate::app::Route::Log {},
                        class: "block group",
                        MealSummaryCard { meal_type: meal_type.clone(), calories: *cals, item_count: *count }
                    }
                }
            }
        }
    }
}

#[component]
fn GoalCircle(label: &'static str, current: f64, target: f64, unit: &'static str, color: &'static str) -> Element {
    let pct = if target > 0.0 { (current / target * 100.0).min(100.0) } else { 0.0 };
    rsx! {
        div { class: "text-center p-3 rounded-lg bg-surface-secondary",
            p { class: "text-xs text-muted-foreground", "{label}" }
            p { class: "text-lg font-bold tabular-nums text-foreground mt-1", "{current:.0}" }
            p { class: "text-xs text-muted-foreground", "/ {target:.0} {unit}" }
            div { class: "macro-bar-track mt-2",
                div {
                    class: "macro-bar-fill {color}",
                    style: "width: {pct:.0}%",
                }
            }
        }
    }
}

#[component]
fn MealSummaryCard(meal_type: MealType, calories: f64, item_count: usize) -> Element {
    let cals_text = format!("{:.0} kcal", calories);
    let count_text = if item_count == 0 {
        "No items logged".to_string()
    } else {
        format!("{} item(s)", item_count)
    };

    let meal_icon = match meal_type {
        MealType::Breakfast => rsx! { IconSunrise { size: 20 } },
        MealType::Lunch => rsx! { IconSun { size: 20 } },
        MealType::Dinner => rsx! { IconMoon { size: 20 } },
        MealType::Snack => rsx! { IconCoffee { size: 20 } },
    };

    rsx! {
        Card { class: "group-hover-shadow-md group-hover--translate-y-0_5 transition-all duration-200 cursor-pointer",
            CardContent { class: "flex items-center justify-between",
                div { class: "flex items-center gap-3",
                    div { class: "meal-icon",
                        {meal_icon}
                    }
                    div {
                        p { class: "font-medium text-card-foreground", "{meal_type.label()}" }
                        p { class: "text-xs text-muted-foreground", "{count_text}" }
                    }
                }
                div { class: "flex items-center gap-2",
                    span { class: "text-sm font-semibold tabular-nums text-card-foreground", "{cals_text}" }
                    IconChevronRight { size: 16 }
                }
            }
        }
    }
}
