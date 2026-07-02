use crate::state::food_db::find_food;
use crate::state::models::{FoodEntry, Meal};

pub fn meal_total_calories(meal: &Meal) -> f64 {
    meal.entries.iter().map(|e| entry_calories(e)).sum()
}

pub fn entry_calories(entry: &FoodEntry) -> f64 {
    find_food(&entry.food_id)
        .map(|f| f.calories * entry.servings)
        .unwrap_or(0.0)
}

pub fn entry_nutrition(entry: &FoodEntry) -> (f64, f64, f64, f64) {
    find_food(&entry.food_id)
        .map(|f| {
            (
                f.protein_g * entry.servings,
                f.carbs_g * entry.servings,
                f.fat_g * entry.servings,
                f.fiber_g * entry.servings,
            )
        })
        .unwrap_or((0.0, 0.0, 0.0, 0.0))
}

pub fn todays_date() -> String {
    let date = js_sys::Date::new_0();
    let year = date.get_full_year();
    let month = date.get_month() + 1;
    let day = date.get_date();
    format!("{:04}-{:02}-{:02}", year, month, day)
}
