use dioxus::prelude::*;

use crate::components::card::{Card, CardContent};
use crate::components::icons::IconCalendar;
use crate::state::food_db::find_food;
use crate::state::models::DayLog;

#[component]
pub fn History() -> Element {
    let logs = use_context::<Signal<Vec<DayLog>>>();

    let mut sorted = logs.read().clone();
    sorted.sort_by(|a, b| b.date.cmp(&a.date));

    rsx! {
        div { class: "max-w-4xl mx-auto p-6 space-y-6",
            h1 { class: "text-2xl font-bold text-foreground font-heading", "History" }

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
    let total_cal: f64 = day
        .meals
        .values()
        .flat_map(|m| &m.entries)
        .filter_map(|e| find_food(&e.food_id))
        .map(|f| f.calories)
        .sum();

    let entry_count: usize = day.meals.values().map(|m| m.entries.len()).sum();

    rsx! {
        Link {
            class: "block group",
            to: crate::app::Route::Log {},
            Card { class: "group-hover-shadow-md group-hover--translate-y-0_5 transition-all duration-200 cursor-pointer",
                CardContent { class: "flex items-center justify-between",
                    div { class: "flex items-center gap-3",
                        div { class: "w-10 h-10 rounded-xl bg-surface-secondary flex items-center justify-center text-muted-foreground",
                            IconCalendar { size: 18 }
                        }
                        div {
                            p { class: "font-medium text-card-foreground", "{day.date}" }
                            p { class: "text-xs text-muted-foreground", "{entry_count} items logged" }
                        }
                    }
                    div { class: "text-right",
                        p { class: "font-semibold tabular-nums text-card-foreground text-lg", "{total_cal:.0}" }
                        p { class: "text-xs text-muted-foreground", "kcal" }
                    }
                }
            }
        }
    }
}
