use std::sync::OnceLock;

use crate::state::models::FoodItem;

pub fn search_foods(query: &str, max_results: usize) -> Vec<FoodItem> {
    let q = query.to_lowercase();
    all_foods()
        .iter()
        .filter(|f| f.name.to_lowercase().contains(&q) || f.brand.as_deref().unwrap_or("").to_lowercase().contains(&q))
        .take(max_results)
        .cloned()
        .collect()
}

pub fn find_food(id: &str) -> Option<FoodItem> {
    all_foods().iter().find(|f| f.id == id).cloned()
}

fn all_foods() -> &'static [FoodItem] {
    static FOODS: OnceLock<Vec<FoodItem>> = OnceLock::new();
    FOODS.get_or_init(|| {
        vec![
            FoodItem { id: "egg-whole".into(), name: "Egg, whole".into(), brand: None, serving_size_g: 50.0, calories: 72.0, protein_g: 6.3, carbs_g: 0.4, fat_g: 4.8, fiber_g: 0.0, sugar_g: 0.2 },
            FoodItem { id: "egg-white".into(), name: "Egg white".into(), brand: None, serving_size_g: 33.0, calories: 17.0, protein_g: 3.6, carbs_g: 0.2, fat_g: 0.1, fiber_g: 0.0, sugar_g: 0.2 },
            FoodItem { id: "oatmeal".into(), name: "Oatmeal, cooked".into(), brand: None, serving_size_g: 234.0, calories: 154.0, protein_g: 5.4, carbs_g: 27.0, fat_g: 2.6, fiber_g: 4.0, sugar_g: 0.5 },
            FoodItem { id: "banana".into(), name: "Banana".into(), brand: None, serving_size_g: 118.0, calories: 105.0, protein_g: 1.3, carbs_g: 27.0, fat_g: 0.4, fiber_g: 3.1, sugar_g: 14.0 },
            FoodItem { id: "apple".into(), name: "Apple".into(), brand: None, serving_size_g: 182.0, calories: 95.0, protein_g: 0.5, carbs_g: 25.0, fat_g: 0.3, fiber_g: 4.4, sugar_g: 19.0 },
            FoodItem { id: "chicken-breast".into(), name: "Chicken breast, grilled".into(), brand: None, serving_size_g: 100.0, calories: 165.0, protein_g: 31.0, carbs_g: 0.0, fat_g: 3.6, fiber_g: 0.0, sugar_g: 0.0 },
            FoodItem { id: "rice-brown".into(), name: "Rice, brown, cooked".into(), brand: None, serving_size_g: 195.0, calories: 218.0, protein_g: 4.5, carbs_g: 46.0, fat_g: 1.8, fiber_g: 3.5, sugar_g: 0.7 },
            FoodItem { id: "rice-white".into(), name: "Rice, white, cooked".into(), brand: None, serving_size_g: 158.0, calories: 205.0, protein_g: 4.3, carbs_g: 45.0, fat_g: 0.4, fiber_g: 0.6, sugar_g: 0.1 },
            FoodItem { id: "broccoli".into(), name: "Broccoli, steamed".into(), brand: None, serving_size_g: 156.0, calories: 55.0, protein_g: 3.7, carbs_g: 11.0, fat_g: 0.6, fiber_g: 5.1, sugar_g: 2.8 },
            FoodItem { id: "salmon".into(), name: "Salmon, grilled".into(), brand: None, serving_size_g: 100.0, calories: 208.0, protein_g: 20.0, carbs_g: 0.0, fat_g: 13.0, fiber_g: 0.0, sugar_g: 0.0 },
            FoodItem { id: "avocado".into(), name: "Avocado".into(), brand: None, serving_size_g: 100.0, calories: 160.0, protein_g: 2.0, carbs_g: 8.5, fat_g: 15.0, fiber_g: 6.7, sugar_g: 0.7 },
            FoodItem { id: "sweet-potato".into(), name: "Sweet potato, baked".into(), brand: None, serving_size_g: 200.0, calories: 180.0, protein_g: 4.0, carbs_g: 41.0, fat_g: 0.3, fiber_g: 6.6, sugar_g: 13.0 },
            FoodItem { id: "greek-yogurt".into(), name: "Greek yogurt, plain".into(), brand: None, serving_size_g: 200.0, calories: 146.0, protein_g: 20.0, carbs_g: 7.9, fat_g: 3.8, fiber_g: 0.0, sugar_g: 7.9 },
            FoodItem { id: "almonds".into(), name: "Almonds".into(), brand: None, serving_size_g: 28.0, calories: 164.0, protein_g: 6.0, carbs_g: 6.1, fat_g: 14.0, fiber_g: 3.5, sugar_g: 1.2 },
            FoodItem { id: "milk-whole".into(), name: "Milk, whole".into(), brand: None, serving_size_g: 244.0, calories: 149.0, protein_g: 7.7, carbs_g: 12.0, fat_g: 8.0, fiber_g: 0.0, sugar_g: 12.0 },
            FoodItem { id: "bread-whole".into(), name: "Bread, whole wheat".into(), brand: None, serving_size_g: 32.0, calories: 82.0, protein_g: 2.7, carbs_g: 14.0, fat_g: 1.1, fiber_g: 2.0, sugar_g: 1.7 },
            FoodItem { id: "pasta".into(), name: "Pasta, cooked".into(), brand: None, serving_size_g: 140.0, calories: 221.0, protein_g: 8.1, carbs_g: 43.0, fat_g: 1.3, fiber_g: 2.5, sugar_g: 0.8 },
            FoodItem { id: "ground-beef".into(), name: "Ground beef, 85/15, cooked".into(), brand: None, serving_size_g: 100.0, calories: 250.0, protein_g: 26.0, carbs_g: 0.0, fat_g: 15.0, fiber_g: 0.0, sugar_g: 0.0 },
            FoodItem { id: "cheese-cheddar".into(), name: "Cheese, cheddar".into(), brand: None, serving_size_g: 28.0, calories: 115.0, protein_g: 6.5, carbs_g: 0.4, fat_g: 9.5, fiber_g: 0.0, sugar_g: 0.1 },
            FoodItem { id: "butter".into(), name: "Butter".into(), brand: None, serving_size_g: 5.0, calories: 36.0, protein_g: 0.0, carbs_g: 0.0, fat_g: 4.1, fiber_g: 0.0, sugar_g: 0.0 },
            FoodItem { id: "olive-oil".into(), name: "Olive oil".into(), brand: None, serving_size_g: 15.0, calories: 119.0, protein_g: 0.0, carbs_g: 0.0, fat_g: 14.0, fiber_g: 0.0, sugar_g: 0.0 },
            FoodItem { id: "protein-shake".into(), name: "Whey protein shake".into(), brand: None, serving_size_g: 35.0, calories: 130.0, protein_g: 25.0, carbs_g: 3.0, fat_g: 1.5, fiber_g: 0.0, sugar_g: 1.0 },
            FoodItem { id: "orange".into(), name: "Orange".into(), brand: None, serving_size_g: 131.0, calories: 62.0, protein_g: 1.2, carbs_g: 15.0, fat_g: 0.2, fiber_g: 3.1, sugar_g: 12.0 },
            FoodItem { id: "strawberries".into(), name: "Strawberries".into(), brand: None, serving_size_g: 144.0, calories: 46.0, protein_g: 1.0, carbs_g: 11.0, fat_g: 0.4, fiber_g: 2.9, sugar_g: 7.0 },
            FoodItem { id: "mixed-greens".into(), name: "Mixed salad greens".into(), brand: None, serving_size_g: 100.0, calories: 20.0, protein_g: 1.5, carbs_g: 3.6, fat_g: 0.2, fiber_g: 2.0, sugar_g: 0.8 },
            FoodItem { id: "tomato".into(), name: "Tomato".into(), brand: None, serving_size_g: 123.0, calories: 22.0, protein_g: 1.1, carbs_g: 4.8, fat_g: 0.2, fiber_g: 1.5, sugar_g: 3.2 },
            FoodItem { id: "tuna".into(), name: "Tuna, canned in water".into(), brand: None, serving_size_g: 100.0, calories: 132.0, protein_g: 28.0, carbs_g: 0.0, fat_g: 1.4, fiber_g: 0.0, sugar_g: 0.0 },
            FoodItem { id: "black-beans".into(), name: "Black beans, cooked".into(), brand: None, serving_size_g: 172.0, calories: 227.0, protein_g: 15.0, carbs_g: 41.0, fat_g: 0.9, fiber_g: 15.0, sugar_g: 0.6 },
            FoodItem { id: "quinoa".into(), name: "Quinoa, cooked".into(), brand: None, serving_size_g: 185.0, calories: 222.0, protein_g: 8.1, carbs_g: 39.0, fat_g: 3.6, fiber_g: 5.2, sugar_g: 1.6 },
            FoodItem { id: "cottage-cheese".into(), name: "Cottage cheese, 2%".into(), brand: None, serving_size_g: 226.0, calories: 194.0, protein_g: 25.0, carbs_g: 8.3, fat_g: 5.1, fiber_g: 0.0, sugar_g: 8.3 },
        ]
    })
}
