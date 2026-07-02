use dioxus::prelude::*;

use crate::components::card::{Card, CardContent};
use crate::components::icons::IconArrowLeft;
use crate::state::food_db::find_food;
use crate::state::models::DayLog;

#[component]
pub fn MealDetail(id: String) -> Element {
    let logs = use_context::<Signal<Vec<DayLog>>>();

    let meal = {
        let logs_guard = logs.read();
        logs_guard
            .iter()
            .find_map(|day| day.meals.values().find(|m| m.id == id))
            .cloned()
    };

    let entries: Vec<_> = meal
        .as_ref()
        .map(|m| {
            m.entries
                .iter()
                .map(|e| {
                    let food = find_food(&e.food_id);
                    let name = food.as_ref().map(|f| f.name.clone()).unwrap_or_else(|| "Unknown".to_string());
                    let cals_str = food
                        .as_ref()
                        .map(|f| format!("{:.0} kcal", f.calories * e.servings))
                        .unwrap_or_default();
                    let macro_str = food
                        .as_ref()
                        .map(|f| {
                            format!(
                                "P {:.0} / C {:.0} / F {:.0}",
                                f.protein_g * e.servings,
                                f.carbs_g * e.servings,
                                f.fat_g * e.servings,
                            )
                        })
                        .unwrap_or_default();
                    let servings_str = format!("x{:.1} serving", e.servings);
                    (name, cals_str, macro_str, servings_str)
                })
                .collect()
        })
        .unwrap_or_default();

    let meal_type_str = meal.as_ref().map(|m| m.meal_type.label()).unwrap_or("Meal");

    let total_cals: f64 = meal.as_ref().map(|m| {
        m.entries.iter().filter_map(|e| find_food(&e.food_id)).map(|f| f.calories).sum()
    }).unwrap_or(0.0);

    let total_protein: f64 = meal.as_ref().map(|m| {
        m.entries.iter().filter_map(|e| find_food(&e.food_id)).map(|f| f.protein_g).sum()
    }).unwrap_or(0.0);

    let total_carbs: f64 = meal.as_ref().map(|m| {
        m.entries.iter().filter_map(|e| find_food(&e.food_id)).map(|f| f.carbs_g).sum()
    }).unwrap_or(0.0);

    let total_fat: f64 = meal.as_ref().map(|m| {
        m.entries.iter().filter_map(|e| find_food(&e.food_id)).map(|f| f.fat_g).sum()
    }).unwrap_or(0.0);

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

            if !entries.is_empty() {
                div { class: "flex items-center justify-between mb-6",
                    h1 { class: "text-2xl font-bold text-foreground font-heading", "{meal_type_str}" }
                    span { class: "text-lg font-semibold tabular-nums text-foreground", "{total_cals:.0} kcal" }
                }

                if total_protein > 0.0 || total_carbs > 0.0 || total_fat > 0.0 {
                    div { class: "grid grid-cols-3 gap-3 mb-6",
                        div { class: "macro-stat bg-protein-10",
                            p { class: "macro-stat-label text-protein", "Protein" }
                            p { class: "macro-stat-value text-protein", "{total_protein:.0}g" }
                        }
                        div { class: "macro-stat bg-carbs-10",
                            p { class: "macro-stat-label text-carbs", "Carbs" }
                            p { class: "macro-stat-value text-carbs", "{total_carbs:.0}g" }
                        }
                        div { class: "macro-stat bg-fat-10",
                            p { class: "macro-stat-label text-fat", "Fat" }
                            p { class: "macro-stat-value text-fat", "{total_fat:.0}g" }
                        }
                    }
                }

                Card {
                    CardContent {
                        div { class: "divide-y divide-border",
                            for (name, cals_str, macro_str, servings_str) in &entries {
                                div { class: "flex items-center justify-between py-3",
                                    div {
                                        p { class: "text-sm font-medium text-foreground", "{name}" }
                                        p { class: "text-xs text-muted-foreground", "{servings_str}" }
                                    }
                                    div { class: "text-right text-xs text-muted-foreground tabular-nums",
                                        p { class: "font-medium text-foreground", "{cals_str}" }
                                        p { "{macro_str}" }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                Card {
                    CardContent { class: "text-center py-12",
                        p { class: "text-muted-foreground", "Meal not found" }
                    }
                }
            }
        }
    }
}
