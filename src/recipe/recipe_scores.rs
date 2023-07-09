use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize,sqlx::FromRow)]
pub struct RecipeScores {
    pub id: i32,
    pub recipe_id: i32,
    pub recipe_like: i32,
    pub suggest_number: i32,
    pub recipe_scores: f64,
}
