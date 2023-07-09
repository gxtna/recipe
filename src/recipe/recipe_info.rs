use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize,sqlx::FromRow)]
pub struct RecipeInfo {
    pub id: i32,
    pub name: String,
    pub taste: String,
}
