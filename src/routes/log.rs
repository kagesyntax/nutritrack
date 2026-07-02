use dioxus::prelude::*;
use dioxus_components::{
    Button, ButtonSize, ButtonVariant, Card, CardContent, CardHeader, CardTitle, Dialog,
    DialogContent, DialogDescription, DialogOverlay, DialogTitle,
};
use dioxus_free_icons::icons::fi_icons::{FiCoffee, FiMoon, FiPlus, FiSearch, FiSun, FiSunrise, FiX};
use dioxus_free_icons::Icon;

use crate::state::food_db::{find_food, search_foods};
use crate::state::models::{FoodEntry, FoodItem, Meal, MealType};
use crate::utils::{entry_calories, entry_nutrition, todays_date};

#[component]
pub fn Log() -> Element {
    let logs = use_context::<Signal<Vec<crate::state::models::DayLog>>>();
    let today = todays_date();

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
            (mt.clone(), meal)
        })
        .collect();

    rsx! {
        div { class: "max-w-4xl mx-auto p-6 space-y-6",
            h1 { class: "text-2xl font-bold text-foreground", "Food Log" }
            p { class: "text-sm text-muted-foreground -mt-4", "{today}" }

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
        MealType::Breakfast => rsx! { Icon { width: 18, height: 18, fill: "currentColor", icon: FiSunrise } },
        MealType::Lunch => rsx! { Icon { width: 18, height: 18, fill: "currentColor", icon: FiSun } },
        MealType::Dinner => rsx! { Icon { width: 18, height: 18, fill: "currentColor", icon: FiMoon } },
        MealType::Snack => rsx! { Icon { width: 18, height: 18, fill: "currentColor", icon: FiCoffee } },
    };

    rsx! {
        Card {
            CardHeader { class: "flex items-center justify-between",
                div { class: "flex items-center gap-2",
                    div { class: "text-muted-foreground", {meal_icon} }
                    CardTitle { "{meal_type.label()}" }
                    span { class: "text-xs text-muted-foreground", "{cals_str}" }
                }
                FoodSearchTrigger { meal_type: meal_type }
            }
            CardContent {
                if let Some(m) = &meal {
                    if m.entries.is_empty() {
                        p { class: "text-sm text-muted-foreground py-4 text-center", "No food logged yet" }
                    } else {
                        div { class: "divide-y divide-border",
                            for entry in &m.entries {
                                FoodEntryRow { entry: entry.clone() }
                            }
                        }
                        div { class: "mt-3 pt-3 border-t border-border flex justify-between text-sm font-medium text-foreground",
                            span { "Totals" }
                            span { class: "tabular-nums", "{nut_str}" }
                        }
                    }
                } else {
                    p { class: "text-sm text-muted-foreground py-4 text-center", "Log your first meal of the day" }
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
            class: "gap-1",
            onclick: move |_| show_search.set(true),
            Icon { width: 14, height: 14, fill: "currentColor", icon: FiPlus }
            "Add Food"
        }
        if show_search() {
            FoodSearchModal {
                meal_type: meal_type,
                is_open: true,
                on_close: move |_| show_search.set(false),
            }
        }
    }
}

#[component]
fn FoodEntryRow(entry: FoodEntry) -> Element {
    let food = find_food(&entry.food_id);
    let servings_str = format!("x{:.1}", entry.servings);

    let cals_str = food
        .as_ref()
        .map(|f| format!("{:.0} kcal", f.calories * entry.servings))
        .unwrap_or_default();

    rsx! {
        div { class: "flex items-center justify-between py-2 text-sm",
            div { class: "flex-1",
                p { class: "text-foreground font-medium",
                    if let Some(f) = &food {
                        "{f.name}"
                    } else {
                        "Unknown food"
                    }
                }
                p { class: "text-xs text-muted-foreground", "{servings_str}" }
            }
            if let Some(_f) = &food {
                span { class: "tabular-nums text-muted-foreground", "{cals_str}" }
            }
        }
    }
}

#[component]
fn FoodSearchModal(
    meal_type: MealType,
    is_open: bool,
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
                let meal = day.meals.values_mut().find(|m| m.meal_type == meal_type);
                if let Some(m) = meal {
                    m.entries.push(FoodEntry::new(food_id));
                }
            }
        }
        query.set(String::new());
        results.set(Vec::new());
        on_close.call(());
    };

    let search_results: Vec<_> = {
        let results = results();
        results
            .iter()
            .map(|food| {
                let cals_str = format!("{:.0} kcal", food.calories);
                let macro_str = format!("P {:.0} / C {:.0} / F {:.0}", food.protein_g, food.carbs_g, food.fat_g);
                let name = food.name.clone();
                let brand = food.brand.clone();
                let id = food.id.clone();
                let mut add = add_food.clone();
                rsx! {
                    div {
                        class: "flex items-center justify-between py-2 cursor-pointer hover:bg-muted px-2 rounded-lg transition-colors",
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
                }
            })
            .collect::<Vec<_>>()
    };

    let no_results = results.read().is_empty() && query.read().len() >= 1;

    rsx! {
        Dialog {
            open: Some(is_open),
            on_open_change: move |open: bool| if !open { on_close.call(()) },
            DialogOverlay { class: "fixed inset-0 bg-black/40" }
            DialogContent { class: "fixed left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 bg-card rounded-xl shadow-lg max-w-lg w-full mx-4 max-h-[85vh] overflow-y-auto p-0",
                div { class: "flex items-center justify-between px-6 py-4 border-b border-border",
                    DialogTitle { class: "text-lg font-semibold text-card-foreground", "Add Food" }
                    button {
                        class: "p-1.5 rounded-lg hover:bg-muted text-muted-foreground transition-colors",
                        onclick: move |_| on_close.call(()),
                        Icon { width: 16, height: 16, fill: "currentColor", icon: FiX }
                    }
                }
                DialogDescription { class: "sr-only", "Search for a food to add to your log" }
                div { class: "p-6 space-y-3",
                    div { class: "relative",
                        Icon { class: "absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground", width: 16, height: 16, fill: "currentColor", icon: FiSearch }
                        input {
                            class: "w-full pl-10 pr-3 py-2 rounded-lg border border-border bg-background text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-primary/30 focus:border-primary text-sm",
                            placeholder: "Search foods...",
                            value: query(),
                            oninput: move |e| update_search(e.value()),
                        }
                    }
                    div { class: "divide-y divide-border max-h-64 overflow-y-auto",
                        {search_results.into_iter()}
                        if no_results {
                            p { class: "text-sm text-muted-foreground text-center py-4", "No foods found" }
                        }
                    }
                }
            }
        }
    }
}
