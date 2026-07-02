use dioxus::prelude::*;

use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::components::card::{Card, CardContent, CardHeader, CardTitle};
use crate::components::dialog::{Dialog, DialogContent};
use crate::components::icons::{
    IconCoffee, IconMinus, IconMoon, IconPlus, IconSearch, IconSun, IconSunrise, IconX,
};
use crate::state::food_db::{find_food, search_foods};
use crate::state::models::{FoodEntry, FoodItem, Meal, MealType};
use crate::utils::{entry_calories, entry_nutrition, todays_date};

#[component]
pub fn Log() -> Element {
    let mut logs = use_context::<Signal<Vec<crate::state::models::DayLog>>>();
    let today = todays_date();

    // Ensure all meal slots exist in today's log
    {
        let mut guard = logs.write();
        if let Some(day) = guard.iter_mut().find(|l| l.date == today) {
            for mt in MealType::all() {
                if !day.meals.values().any(|m| m.meal_type == mt) {
                    let meal = Meal::new(mt);
                    day.meals.insert(meal.id.clone(), meal);
                }
            }
        }
    }

    let day_log = {
        let logs_guard = logs.read();
        logs_guard.iter().find(|l| l.date == today).cloned()
    };

    let meals: Vec<(MealType, Option<Meal>)> = MealType::all()
        .iter()
        .map(|mt| {
            let meal = day_log
                .as_ref()
                .and_then(|dl| dl.meals.values().find(|m| m.meal_type == *mt))
                .cloned();
            (*mt, meal)
        })
        .collect();

    rsx! {
        div { class: "max-w-4xl mx-auto p-6 space-y-6",
            div { class: "flex items-baseline justify-between",
                h1 { class: "text-2xl font-bold text-foreground font-heading", "Food Log" }
                p { class: "text-sm text-muted-foreground", "{today}" }
            }

            div { class: "space-y-4",
                for (meal_type, meal) in meals {
                    MealSection { meal_type: meal_type, meal: meal }
                }
            }
        }
    }
}

#[component]
fn MealSection(meal_type: MealType, meal: Option<Meal>) -> Element {
    let cals = meal.as_ref().map(|m| {
        m.entries.iter().map(|e| entry_calories(e)).sum::<f64>()
    }).unwrap_or(0.0);

    let mut nut_vals = [0.0_f64; 4];
    if let Some(m) = &meal {
        for e in &m.entries {
            let n = entry_nutrition(e);
            nut_vals[0] += n.0;
            nut_vals[1] += n.1;
            nut_vals[2] += n.2;
            nut_vals[3] += n.3;
        }
    }

    let cals_str = format!("{:.0} kcal", cals);
    let nut_str = format!("{:.0}g P / {:.0}g C / {:.0}g F", nut_vals[0], nut_vals[1], nut_vals[2]);

    let meal_icon = match meal_type {
        MealType::Breakfast => rsx! { IconSunrise { size: 18 } },
        MealType::Lunch => rsx! { IconSun { size: 18 } },
        MealType::Dinner => rsx! { IconMoon { size: 18 } },
        MealType::Snack => rsx! { IconCoffee { size: 18 } },
    };

    rsx! {
        Card { class: "overflow-hidden",
            CardHeader { class: "flex items-center justify-between gap-2",
                div { class: "flex items-center gap-2 min-w-0",
                    div { class: "text-muted-foreground shrink-0", {meal_icon} }
                    CardTitle { class: "font-heading truncate", "{meal_type.label()}" }
                    span { class: "text-xs text-muted-foreground tabular-nums shrink-0", "{cals_str}" }
                }
                FoodSearchTrigger { meal_type: meal_type }
            }
            CardContent { class: "pt-0",
                if let Some(m) = &meal {
                    if m.entries.is_empty() {
                        p { class: "text-sm text-muted-foreground py-6 text-center", "No food logged yet" }
                    } else {
                        div { class: "divide-y divide-border",
                            for (i, entry) in m.entries.iter().enumerate() {
                                FoodEntryRow {
                                    entry: entry.clone(),
                                    meal_id: m.id.clone(),
                                    entry_index: i,
                                }
                            }
                        }
                        div { class: "mt-3 pt-3 border-t border-border flex justify-between text-sm font-medium text-foreground",
                            span { "Totals" }
                            span { class: "tabular-nums", "{nut_str}" }
                        }
                    }
                } else {
                    p { class: "text-sm text-muted-foreground py-6 text-center", "Log your first meal of the day" }
                }
            }
        }
    }
}

#[component]
fn FoodSearchTrigger(meal_type: MealType) -> Element {
    let mut show_search = use_signal(|| false);

    rsx! {
        Button {
            variant: ButtonVariant::Ghost,
            size: ButtonSize::Sm,
            class: "gap-1 shrink-0",
            onclick: move |_| show_search.set(true),
            IconPlus { size: 14 }
            "Add Food"
        }
        if show_search() {
            FoodSearchModal {
                meal_type: meal_type,
                on_close: move |_| show_search.set(false),
            }
        }
    }
}

#[component]
fn FoodEntryRow(entry: FoodEntry, meal_id: String, entry_index: usize) -> Element {
    let food = find_food(&entry.food_id);
    let mut logs = use_context::<Signal<Vec<crate::state::models::DayLog>>>();
    let today = todays_date();
    let t1 = today.clone();
    let t2 = today.clone();
    let t3 = today.clone();

    let mid_dec = meal_id.clone();
    let mid_inc = meal_id.clone();
    let mid_rem = meal_id.clone();

    let servings_str = format!("{:.1}", entry.servings);
    let cals_str = food
        .as_ref()
        .map(|f| format!("{:.0}", f.calories * entry.servings))
        .unwrap_or_default();

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
            }
            div { class: "flex items-center gap-1_5 shrink-0",
                div { class: "serving-control",
                    button {
                        class: "serving-btn",
                        onclick: move |_| {
                            let mut guard = logs.write();
                            if let Some(day) = guard.iter_mut().find(|l| l.date == t1) {
                                if let Some(meal) = day.meals.get_mut(&mid_dec) {
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
                            if let Some(day) = guard.iter_mut().find(|l| l.date == t2) {
                                if let Some(meal) = day.meals.get_mut(&mid_inc) {
                                    if entry_index < meal.entries.len() {
                                        meal.entries[entry_index].servings += 0.5;
                                    }
                                }
                            }
                        },
                        IconPlus { size: 12 }
                    }
                }
                span { class: "text-xs tabular-nums text-muted-foreground w-12 text-right", "{cals_str}" }
                button {
                    class: "btn-remove",
                    onclick: move |_| {
                        let mut guard = logs.write();
                        if let Some(day) = guard.iter_mut().find(|l| l.date == t3) {
                            if let Some(meal) = day.meals.get_mut(&mid_rem) {
                                if entry_index < meal.entries.len() {
                                    meal.entries.remove(entry_index);
                                }
                            }
                        }
                    },
                    IconX { size: 14 }
                }
            }
        }
    }
}

#[component]
fn FoodSearchModal(
    meal_type: MealType,
    on_close: EventHandler<()>,
) -> Element {
    let mut query = use_signal(|| String::new());
    let mut results = use_signal(|| Vec::<FoodItem>::new());
    let mut logs = use_context::<Signal<Vec<crate::state::models::DayLog>>>();
    let today = todays_date();

    let mut update_search = move |new_query: String| {
        query.set(new_query.clone());
        if new_query.len() >= 1 {
            results.set(search_foods(&new_query, 10));
        } else {
            results.set(Vec::new());
        }
    };

    let add_food = move |food_id: String| {
        {
            let mut guard = logs.write();
            if let Some(day) = guard.iter_mut().find(|l| l.date == today) {
                let meal_id = day
                    .meals
                    .values()
                    .find(|m| m.meal_type == meal_type)
                    .map(|m| m.id.clone())
                    .unwrap_or_else(|| {
                        let new_meal = Meal::new(meal_type);
                        let id = new_meal.id.clone();
                        day.meals.insert(id.clone(), new_meal);
                        id
                    });
                if let Some(m) = day.meals.get_mut(&meal_id) {
                    m.entries.push(FoodEntry::new(food_id));
                }
            }
        }
        query.set(String::new());
        results.set(Vec::new());
        on_close.call(());
    };

    let no_results = results.read().is_empty() && query.read().len() >= 1;

    let result_elements = {
        let results = results();
        let mut items = Vec::new();
        for food in results.iter() {
            let cals_str = format!("{:.0} kcal", food.calories);
            let macro_str = format!("P {:.0} / C {:.0} / F {:.0}", food.protein_g, food.carbs_g, food.fat_g);
            let name = food.name.clone();
            let brand = food.brand.clone();
            let id = food.id.clone();
            let mut add = add_food.clone();
            items.push(rsx! {
                div {
                    class: "food-result",
                    onclick: move |_| add(id.clone()),
                    div {
                        p { class: "text-sm font-medium text-foreground", "{name}" }
                        if let Some(b) = &brand {
                            p { class: "text-xs text-muted-foreground", "{b}" }
                        }
                    }
                    div { class: "text-right text-xs text-muted-foreground tabular-nums",
                        p { "{cals_str}" }
                        p { "{macro_str}" }
                    }
                }
            });
        }
        items
    };

    rsx! {
        Dialog {
            show: true,
            onclose: move |_| on_close.call(()),
            DialogContent { class: "p-0",
                div { class: "flex items-center justify-between px-6 py-4 border-b border-border",
                    h3 { class: "text-lg font-semibold text-card-foreground", "Add Food" }
                    button {
                        class: "p-1_5 rounded-lg hover-bg-muted text-muted-foreground transition-colors",
                        onclick: move |_| on_close.call(()),
                        IconX { size: 16 }
                    }
                }
                div { class: "p-6 space-y-3",
                    div { class: "search-wrapper",
                        div { class: "search-icon",
                            IconSearch { size: 16 }
                        }
                        input {
                            class: "search-input",
                            placeholder: "Search foods...",
                            value: query(),
                            oninput: move |e| update_search(e.value()),
                        }
                    }
                    div { class: "divide-y divide-border max-h-64 overflow-y-auto",
                        {result_elements.into_iter()}
                        if no_results {
                            p { class: "text-sm text-muted-foreground text-center py-6", "No foods found" }
                        }
                    }
                }
            }
        }
    }
}
