use dioxus::prelude::*;

use crate::components::card::{Card, CardContent, CardHeader, CardTitle};
use crate::components::icons::{IconArrowLeft, IconCoffee, IconMinus, IconMoon, IconPlus, IconSun, IconSunrise};
use crate::state::food_db::find_food;
use crate::state::models::{DayLog, FoodEntry, UserSettings};

#[component]
pub fn MealDetail(id: String) -> Element {
    let logs = use_context::<Signal<Vec<DayLog>>>();
    let settings = use_context::<Signal<UserSettings>>();

    let Some((meal, day_date)) = ({
        let logs_guard = logs.read();
        let mut result = None;
        for day in logs_guard.iter() {
            if let Some(m) = day.meals.values().find(|m| m.id == id) {
                result = Some((m.clone(), day.date.clone()));
                break;
            }
        }
        result
    }) else {
        return rsx! {
            div { class: "max-w-4xl mx-auto p-6",
                Link {
                    class: "back-link",
                    to: crate::app::Route::Log {},
                    button {
                        class: "inline-flex items-center gap-1_5 px-3 py-1_5 rounded-lg hover-bg-muted transition-colors",
                        IconArrowLeft { size: 16 }
                        "Back to Log"
                    }
                }
                Card {
                    CardContent { class: "text-center py-12",
                        p { class: "text-muted-foreground", "Meal not found" }
                    }
                }
            }
        };
    };

    let m_entries = meal.entries.clone();

    let total_cals: f64 = m_entries.iter().map(|e| {
        find_food(&e.food_id).map(|f| f.calories * e.servings).unwrap_or(0.0)
    }).sum();

    let total_protein: f64 = m_entries.iter().map(|e| {
        find_food(&e.food_id).map(|f| f.protein_g * e.servings).unwrap_or(0.0)
    }).sum();

    let total_carbs: f64 = m_entries.iter().map(|e| {
        find_food(&e.food_id).map(|f| f.carbs_g * e.servings).unwrap_or(0.0)
    }).sum();

    let total_fat: f64 = m_entries.iter().map(|e| {
        find_food(&e.food_id).map(|f| f.fat_g * e.servings).unwrap_or(0.0)
    }).sum();

    let targets = settings.read().targets.clone();

    let cal_pct = if targets.calories > 0.0 { (total_cals / targets.calories * 100.0).min(100.0) } else { 0.0 };
    let protein_pct = if targets.protein_g > 0.0 { (total_protein / targets.protein_g * 100.0).min(100.0) } else { 0.0 };
    let carbs_pct = if targets.carbs_g > 0.0 { (total_carbs / targets.carbs_g * 100.0).min(100.0) } else { 0.0 };
    let fat_pct = if targets.fat_g > 0.0 { (total_fat / targets.fat_g * 100.0).min(100.0) } else { 0.0 };

    let meal_type_str = meal.meal_type.label();

    let meal_icon = match meal.meal_type {
        crate::state::models::MealType::Breakfast => rsx! { IconSunrise { size: 24 } },
        crate::state::models::MealType::Lunch => rsx! { IconSun { size: 24 } },
        crate::state::models::MealType::Dinner => rsx! { IconMoon { size: 24 } },
        crate::state::models::MealType::Snack => rsx! { IconCoffee { size: 24 } },
    };

    rsx! {
        div { class: "max-w-4xl mx-auto p-6",
            Link {
                class: "back-link",
                to: crate::app::Route::Log {},
                button {
                    class: "inline-flex items-center gap-1_5 px-3 py-1_5 rounded-lg hover-bg-muted transition-colors",
                    IconArrowLeft { size: 16 }
                    "Back to Log"
                }
            }

            div { class: "flex items-center gap-3 mb-6",
                div { class: "text-muted-foreground", {meal_icon} }
                h1 { class: "text-2xl font-bold text-foreground font-heading", "{meal_type_str}" }
            }

            Card { class: "mb-6",
                CardContent { class: "text-center py-6",
                    p { class: "text-xs text-muted-foreground uppercase tracking-wider", "Total Calories" }
                    p { class: "text-3xl font-bold tabular-nums text-foreground font-mono mt-1", "{total_cals:.0}" }
                    p { class: "text-sm text-muted-foreground mt-1", "of {targets.calories:.0} daily goal" }
                    div { class: "macro-bar-track mt-3 max-w-md mx-auto",
                        div { class: "macro-bar-fill bg-primary", style: "width: {cal_pct:.0}%" }
                    }
                }
            }

            if total_protein > 0.0 || total_carbs > 0.0 || total_fat > 0.0 {
                div { class: "grid grid-cols-3 gap-3 mb-6",
                    div { class: "macro-stat bg-protein-10",
                        p { class: "macro-stat-label text-protein", "Protein" }
                        p { class: "macro-stat-value text-protein", "{total_protein:.0}g" }
                        div { class: "macro-bar-track mt-2",
                            div { class: "macro-bar-fill bg-protein", style: "width: {protein_pct:.0}%" }
                        }
                        p { class: "text-10px text-muted-foreground mt-1", "{protein_pct:.0}% of {targets.protein_g:.0}g" }
                    }
                    div { class: "macro-stat bg-carbs-10",
                        p { class: "macro-stat-label text-carbs", "Carbs" }
                        p { class: "macro-stat-value text-carbs", "{total_carbs:.0}g" }
                        div { class: "macro-bar-track mt-2",
                            div { class: "macro-bar-fill bg-carbs", style: "width: {carbs_pct:.0}%" }
                        }
                        p { class: "text-10px text-muted-foreground mt-1", "{carbs_pct:.0}% of {targets.carbs_g:.0}g" }
                    }
                    div { class: "macro-stat bg-fat-10",
                        p { class: "macro-stat-label text-fat", "Fat" }
                        p { class: "macro-stat-value text-fat", "{total_fat:.0}g" }
                        div { class: "macro-bar-track mt-2",
                            div { class: "macro-bar-fill bg-fat", style: "width: {fat_pct:.0}%" }
                        }
                        p { class: "text-10px text-muted-foreground mt-1", "{fat_pct:.0}% of {targets.fat_g:.0}g" }
                    }
                }
            }

            Card {
                CardHeader { class: "flex items-center justify-between",
                    div { class: "flex items-center gap-2",
                        CardTitle { class: "font-heading", "Foods" }
                    }
                    span { class: "text-sm text-muted-foreground", "{meal.entries.len()} item(s)" }
                }
                CardContent { class: "pt-0",
                    if meal.entries.is_empty() {
                        p { class: "text-sm text-muted-foreground py-6 text-center", "No food logged in this meal" }
                    } else {
                        div { class: "divide-y divide-border",
                            for (i, entry) in meal.entries.iter().enumerate() {
                                EntryRow {
                                    entry: entry.clone(),
                                    meal_id: meal.id.clone(),
                                    day_date: day_date.clone(),
                                    entry_index: i,
                                }
                            }
                        }
                    }
                }
            }

            Card { class: "mt-6",
                CardHeader {
                    CardTitle { class: "font-heading text-md", "Daily Contribution" }
                }
                CardContent {
                    div { class: "space-y-3",
                        div { class: "macro-bar",
                            span { class: "macro-bar-label", "Protein" }
                            div { class: "macro-bar-track",
                                div { class: "macro-bar-fill bg-protein", style: "width: {protein_pct:.0}%" }
                            }
                            span { class: "macro-bar-value", "{total_protein:.0} / {targets.protein_g:.0}g" }
                        }
                        div { class: "macro-bar",
                            span { class: "macro-bar-label", "Carbs" }
                            div { class: "macro-bar-track",
                                div { class: "macro-bar-fill bg-carbs", style: "width: {carbs_pct:.0}%" }
                            }
                            span { class: "macro-bar-value", "{total_carbs:.0} / {targets.carbs_g:.0}g" }
                        }
                        div { class: "macro-bar",
                            span { class: "macro-bar-label", "Fat" }
                            div { class: "macro-bar-track",
                                div { class: "macro-bar-fill bg-fat", style: "width: {fat_pct:.0}%" }
                            }
                            span { class: "macro-bar-value", "{total_fat:.0} / {targets.fat_g:.0}g" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn EntryRow(entry: FoodEntry, meal_id: String, day_date: String, entry_index: usize) -> Element {
    let food = find_food(&entry.food_id);
    let mut logs = use_context::<Signal<Vec<DayLog>>>();

    let dec_date = day_date.clone();
    let dec_mid = meal_id.clone();
    let inc_date = day_date.clone();
    let inc_mid = meal_id.clone();

    let servings_str = format!("{:.1}", entry.servings);

    let cals_total = food
        .as_ref()
        .map(|f| format!("{:.0}", f.calories * entry.servings))
        .unwrap_or_default();

    let cals_per = food
        .as_ref()
        .map(|f| format!("{:.0} kcal/serving", f.calories))
        .unwrap_or_default();

    let serving_size = food
        .as_ref()
        .map(|f| format!("({:.0}g)", f.serving_size_g))
        .unwrap_or_default();

    let protein = food.as_ref().map(|f| f.protein_g).unwrap_or(0.0);
    let carbs = food.as_ref().map(|f| f.carbs_g).unwrap_or(0.0);
    let fat = food.as_ref().map(|f| f.fat_g).unwrap_or(0.0);

    rsx! {
        div { class: "flex items-center justify-between py-2_5 gap-2 group",
            div { class: "flex-1 min-w-0",
                p { class: "text-sm font-medium text-foreground truncate",
                    if let Some(f) = &food {
                        "{f.name}"
                    } else {
                        "Unknown food"
                    }
                }
                p { class: "text-10px text-muted-foreground mt-1",
                    "{servings_str} serving(s) x {serving_size} — {cals_per}"
                }
                div { class: "flex items-center gap-2 mt-1",
                    span { class: "text-10px tabular-nums text-protein font-medium", "P {protein:.0}g" }
                    span { class: "text-10px tabular-nums text-carbs font-medium", "C {carbs:.0}g" }
                    span { class: "text-10px tabular-nums text-fat font-medium", "F {fat:.0}g" }
                }
            }
            div { class: "flex items-center gap-1_5 shrink-0",
                div { class: "serving-control",
                    button {
                        class: "serving-btn",
                        onclick: move |_| {
                            let mut guard = logs.write();
                            if let Some(day) = guard.iter_mut().find(|l| l.date == dec_date) {
                                if let Some(meal) = day.meals.get_mut(&dec_mid) {
                                    if entry_index < meal.entries.len() {
                                        let new_val = (meal.entries[entry_index].servings - 0.5).max(0.0);
                                        meal.entries[entry_index].servings = (new_val * 10.0).round() / 10.0;
                                    }
                                }
                            }
                        },
                        IconMinus { size: 12 }
                    }
                    span { class: "serving-value", "{servings_str}" }
                    button {
                        class: "serving-btn",
                        onclick: move |_| {
                            let mut guard = logs.write();
                            if let Some(day) = guard.iter_mut().find(|l| l.date == inc_date) {
                                if let Some(meal) = day.meals.get_mut(&inc_mid) {
                                    if entry_index < meal.entries.len() {
                                        meal.entries[entry_index].servings += 0.5;
                                    }
                                }
                            }
                        },
                        IconPlus { size: 12 }
                    }
                }
                span { class: "text-xs tabular-nums text-muted-foreground w-12 text-right", "{cals_total}" }
            }
        }
    }
}
