use dioxus::prelude::*;

use crate::components::card::{Card, CardContent, CardHeader, CardTitle};
use crate::components::icons::{IconChevronRight, IconCoffee, IconMoon, IconSun, IconSunrise};
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

    rsx! {
        div { class: "max-w-4xl mx-auto p-6 space-y-6",
            div { class: "flex items-center justify-between",
                div {
                    h1 { class: "text-2xl font-bold text-foreground font-heading", "Dashboard" }
                    p { class: "text-sm text-muted-foreground mt-1", "{today}" }
                }
                Link {
                    to: crate::app::Route::Log {},
                    class: "inline-flex items-center gap-1.5 px-4 py-2 rounded-lg bg-primary text-white text-sm font-medium hover:bg-primary-dark transition-colors duration-200",
                    IconSunrise { size: 16 }
                    "Log Meals"
                }
            }

            div { class: "grid grid-cols-1 md:grid-cols-3 gap-6",
                Card { class: "md:col-span-1",
                    CardContent { class: "flex justify-center py-6",
                        CalorieRing { current: total_cal, target: target_cal }
                    }
                }

                Card { class: "md:col-span-2",
                    CardHeader {
                        CardTitle { class: "font-heading", "Macronutrients" }
                    }
                    CardContent {
                        div { class: "space-y-3",
                            MacroBar { label: "Protein", current: total_p, target: targets.protein_g, color_class: "bg-protein", unit: "g" }
                            MacroBar { label: "Carbs", current: total_c, target: targets.carbs_g, color_class: "bg-carbs", unit: "g" }
                            MacroBar { label: "Fat", current: total_f, target: targets.fat_g, color_class: "bg-fat", unit: "g" }
                            MacroBar { label: "Fiber", current: total_fiber, target: targets.fiber_g, color_class: "bg-fiber", unit: "g" }
                        }
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
        Card { class: "group-hover:shadow-md group-hover:-translate-y-0.5 transition-all duration-200 cursor-pointer",
            CardContent { class: "flex items-center justify-between",
                div { class: "flex items-center gap-3",
                    div { class: "w-10 h-10 rounded-xl bg-surface-secondary flex items-center justify-center text-muted-foreground group-hover:text-primary transition-colors duration-200",
                        {meal_icon}
                    }
                    div {
                        p { class: "font-medium text-card-foreground", "{meal_type.label()}" }
                        p { class: "text-xs text-muted-foreground", "{count_text}" }
                    }
                }
                div { class: "flex items-center gap-2",
                    span { class: "text-sm font-semibold tabular-nums text-card-foreground", "{cals_text}" }
                    IconChevronRight { class: "text-muted-foreground group-hover:text-primary transition-colors duration-200", size: 16 }
                }
            }
        }
    }
}
