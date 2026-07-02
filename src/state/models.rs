use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum MealType {
    Breakfast,
    Lunch,
    Dinner,
    Snack,
}

impl MealType {
    pub fn all() -> [MealType; 4] {
        [MealType::Breakfast, MealType::Lunch, MealType::Dinner, MealType::Snack]
    }

    pub fn label(&self) -> &str {
        match self {
            MealType::Breakfast => "Morning",
            MealType::Lunch => "Afternoon",
            MealType::Dinner => "Evening",
            MealType::Snack => "Snacks",
        }
    }

}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FoodItem {
    pub id: String,
    pub name: String,
    pub brand: Option<String>,
    pub serving_size_g: f64,
    pub calories: f64,
    pub protein_g: f64,
    pub carbs_g: f64,
    pub fat_g: f64,
    pub fiber_g: f64,
    pub sugar_g: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FoodEntry {
    pub food_id: String,
    pub servings: f64,
}

impl FoodEntry {
    pub fn new(food_id: String) -> Self {
        Self { food_id, servings: 1.0 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Meal {
    pub id: String,
    pub meal_type: MealType,
    pub entries: Vec<FoodEntry>,
}

impl Meal {
    pub fn new(meal_type: MealType) -> Self {
        Self {
            id: uuid_v4(),
            meal_type,
            entries: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DayLog {
    pub date: String,
    pub meals: HashMap<String, Meal>,
}

impl DayLog {
    pub fn new(date: String) -> Self {
        let mut meals = HashMap::new();
        for mt in MealType::all() {
            let meal = Meal::new(mt.clone());
            meals.insert(meal.id.clone(), meal);
        }
        Self { date, meals }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NutritionTargets {
    pub calories: f64,
    pub protein_g: f64,
    pub carbs_g: f64,
    pub fat_g: f64,
    pub fiber_g: f64,
}

impl Default for NutritionTargets {
    fn default() -> Self {
        Self {
            calories: 2000.0,
            protein_g: 50.0,
            carbs_g: 250.0,
            fat_g: 65.0,
            fiber_g: 25.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserSettings {
    pub targets: NutritionTargets,
    pub theme_mode: ThemeMode,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            targets: NutritionTargets::default(),
            theme_mode: ThemeMode::System,
        }
    }
}

fn uuid_v4() -> String {
    let ms = js_sys::Date::now();
    format!("id-{:.0}", ms)
}
